//! Handles tracking mouse state, identifying discrete events and sending
//! them to a delegate.

use std::mem;
use druid::{KeyModifiers, MouseButton, MouseEvent};
use druid::kurbo::Point;

const DEFAULT_MIN_DRAG_DISTANCE: f64 = 10.0;

/// Handles raw mouse events, parsing them into gestures that it forwards
/// to a delegate.
#[derive(Debug, Clone)]
pub struct Mouse {
    state: MouseState,
    /// The distance the mouse must travel with a button down for it to
    /// be considered a drag gesture.
    pub min_drag_distance: f64,
}

#[derive(Debug, Clone)]
enum MouseState {
    /// No mouse buttons are active.
    Up(MouseEvent),
    /// A mouse button has been pressed.
    Down(MouseEvent),
    /// The mouse has been moved some threshold distance with a button pressed.
    Drag {
        start: MouseEvent,
        current: MouseEvent,
    },
    /// A state only used as a placeholder during event handling.
    #[doc(hidden)]
    Transition,
}

/// The state of an in-progress drag gesture.
#[allow(unused)]
pub struct Drag<'a> {
    /// The event that started this drag
    pub start: &'a MouseEvent,
    /// The previous event in this drag
    pub prev: &'a MouseEvent,
    /// The current event in this drag
    pub current: &'a MouseEvent,
}

/// A trait for types that want fine grained information about mouse events.
pub trait MouseDelegate<T> {
    #[allow(unused)]
    fn mouse_moved(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }

    #[allow(unused)]
    fn left_down(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }
    #[allow(unused)]
    fn left_up(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }
    #[allow(unused)]
    fn left_click(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }

    #[allow(unused)]
    fn left_drag_began(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }
    #[allow(unused)]
    fn left_drag_changed(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }
    #[allow(unused)]
    fn left_drag_ended(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }

    #[allow(unused)]
    fn right_down(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }
    #[allow(unused)]
    fn right_up(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }
    #[allow(unused)]
    fn right_click(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }

    #[allow(unused)]
    fn right_drag_began(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }
    #[allow(unused)]
    fn right_drag_changed(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }
    #[allow(unused)]
    fn right_drag_ended(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }
    #[allow(unused)]
    fn other_down(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }
    #[allow(unused)]
    fn other_up(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }
    #[allow(unused)]
    fn other_click(&mut self, _data: &mut T, _event: &MouseEvent) -> bool {
        false
    }

    #[allow(unused)]
    fn other_drag_began(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }
    #[allow(unused)]
    fn other_drag_changed(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }
    #[allow(unused)]
    fn other_drag_ended(&mut self, _data: &mut T, _drag: Drag) -> bool {
        false
    }

    #[allow(unused)]
    fn cancel(&mut self, data: &mut T);
}

impl Mouse {
    pub fn new() -> Mouse {

        Mouse {
            min_drag_distance: DEFAULT_MIN_DRAG_DISTANCE,
            state: MouseState::Up(MouseEvent {
                pos: Point::ZERO,
                mods: KeyModifiers::default(),
                count: 0,
                button: MouseButton::Left,
            }),
        }
    }

    /// The current position of  the mouse.
    pub fn pos(&self) -> Point {
        match &self.state {
            MouseState::Up(e) => e.pos,
            MouseState::Down(e) => e.pos,
            MouseState::Drag { current, .. } => current.pos,
            _ => panic!("transition is not an actual state :/"),
        }
    }
}

impl Mouse {
    pub fn mouse_moved<T>(
        &mut self,
        data: &mut T,
        event: MouseEvent,
        delegate: &mut dyn MouseDelegate<T>,
    ) -> bool {
        let prev_state = mem::replace(&mut self.state, MouseState::Transition);
        let (next_state, handled) = match prev_state {
            MouseState::Up(_) => {
                let handled = delegate.mouse_moved(data, &event);
                (MouseState::Up(event), handled)
            }
            MouseState::Down(prev) => {
                if prev.pos.distance(event.pos) > self.min_drag_distance {
                    let drag = Drag::new(&prev, &prev, &event);
                    let handled = if prev.button.is_left() {
                        delegate.left_drag_began(data, drag)
                    } else if prev.button.is_right() {
                        delegate.right_drag_began(data, drag)
                    } else {
                        delegate.other_drag_began(data, drag)
                    };
                    (
                        MouseState::Drag {
                            start: prev,
                            current: event,
                        },
                        handled,
                    )
                } else {
                    (MouseState::Down(prev), false)
                }
            }
            MouseState::Drag { start, current } => {
                let drag = Drag::new(&start, &current, &event);
                let handled = if start.button.is_left() {
                    delegate.left_drag_changed(data, drag)
                } else if start.button.is_right() {
                    delegate.right_drag_changed(data, drag)
                } else {
                    delegate.other_drag_changed(data, drag)
                };
                (
                    MouseState::Drag {
                        start,
                        current: event,
                    },
                    handled,
                )
            }
            MouseState::Transition => panic!("ahhhhhhh"),
        };
        self.state = next_state;
        handled
    }

    pub fn mouse_down<T>(
        &mut self,
        data: &mut T,
        event: MouseEvent,
        delegate: &mut dyn MouseDelegate<T>,
    ) -> bool {
        let prev_state = mem::replace(&mut self.state, MouseState::Transition);
        let (new_state, handled) = match prev_state {
            MouseState::Up(_) => {
                let handled = if event.button.is_left() {
                    delegate.left_down(data, &event)
                } else if event.button.is_right() {
                    delegate.right_down(data, &event)
                } else {
                    delegate.other_down(data, &event)
                };
                (MouseState::Down(event), handled)
            }
            MouseState::Down(prev) => {
                assert!(prev.button != event.button);
                // if a second button is pressed while we're handling an event
                // we just ignore it. At some point we could consider an event for this.
                (MouseState::Down(prev), false)
            }
            MouseState::Drag { start, .. } => {
                assert!(start.button != event.button);
                (
                    MouseState::Drag {
                        start,
                        current: event,
                    },
                    false,
                )
            }
            MouseState::Transition => panic!("ahhhhhhh"),
        };
        self.state = new_state;
        handled
    }

    pub fn mouse_up<T>(
        &mut self,
        data: &mut T,
        event: MouseEvent,
        delegate: &mut dyn MouseDelegate<T>,
    ) -> bool {
        let prev_state = mem::replace(&mut self.state, MouseState::Transition);
        let (new_state, handled) = match prev_state {
            MouseState::Up(_) => (MouseState::Up(event), false),
            MouseState::Down(prev) => {
                if event.button == prev.button {
                    let handled = if prev.button.is_left() {
                        delegate.left_up(data, &event) | delegate.left_click(data, &event)
                    } else if prev.button.is_right() {
                        delegate.right_up(data, &event) | delegate.right_click(data, &event)
                    } else {
                        delegate.other_up(data, &event) | delegate.other_click(data, &event)
                    };
                    (MouseState::Up(event), handled)
                } else {
                    (MouseState::Down(prev), false)
                }
            }
            MouseState::Drag { start, current } => {
                if event.button == start.button {
                    let drag = Drag {
                        start: &start,
                        current: &event,
                        prev: &current,
                    };
                    let handled = if start.button.is_left() {
                        delegate.left_up(data, &event) | delegate.left_drag_ended(data, drag)
                    } else if start.button.is_right() {
                        delegate.left_up(data, &event) | delegate.right_drag_ended(data, drag)
                    } else {
                        delegate.left_up(data, &event) | delegate.other_drag_ended(data, drag)
                    };
                    (MouseState::Up(event), handled)
                } else {
                    (MouseState::Drag { start, current }, false)
                }
            }
            MouseState::Transition => panic!("ahhhhhhh"),
        };
        self.state = new_state;
        handled
    }

    fn cancel<T>(&mut self, data: &mut T, delegate: &mut dyn MouseDelegate<T>) {
        let prev_state = mem::replace(&mut self.state, MouseState::Transition);
        let last_event = match prev_state {
            MouseState::Down(event) => event,
            MouseState::Up(event) => event,
            MouseState::Drag { current, .. } => current,
            MouseState::Transition => panic!("ahhhhhhh"),
        };
        delegate.cancel(data);
        self.state = MouseState::Up(last_event);
    }
}

impl<'a> Drag<'a> {
    fn new(start: &'a MouseEvent, prev: &'a MouseEvent, current: &'a MouseEvent) -> Drag<'a> {
        Drag {
            start,
            prev,
            current,
        }
    }
}

