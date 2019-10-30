use std::collections::BTreeSet;
use std::sync::Arc;

use druid::kurbo::{Point, Rect, Vec2};
use druid::{KeyCode, KeyEvent, MouseEvent};

use crate::mouse::{Drag, MouseDelegate};
use crate::design_space::DVec2;
use crate::path::EntityId;
use crate::edit_session::EditSession;


///// The state of the selection tool.
//#[derive(Debug, Clone)]
//pub struct Select {
    ///// when a drag is in progress, this is the state of the selection at the start
    ///// of the drag.
    //prev_selection: Option<Arc<BTreeSet<EntityId>>>,
    //drag_rect: Option<Rect>,
    //last_drag_pos: Option<Point>,
    //last_pos: Point,
//}

//impl Select {
    //pub fn new() -> Self {
        //Select {
            //prev_selection: None,
            //drag_rect: None,
            //last_drag_pos: None,
            //last_pos: Point::ZERO,
        //}
    //}

    //fn update_selection_for_drag(
        //&self,
        //data: &mut EditSession,
        //prev_sel: &BTreeSet<EntityId>,
        //rect: Rect,
        //shift: bool,
    //) {
        //let vport = data.viewport;
        //let in_select_rect = data
            //.iter_points()
            //.filter(|p| rect.contains(p.to_screen(vport)))
            //.map(|p| p.id)
            //.collect();
        //let new_sel = if shift {
            //prev_sel
                //.symmetric_difference(&in_select_rect)
                //.copied()
                //.collect()
        //} else {
            //prev_sel.union(&in_select_rect).copied().collect()
        //};
        //*data.selection_mut() = new_sel;
    //}

    //fn nudge(&mut self, data: &mut EditSession, event: &KeyEvent) {
        //use KeyCode::*;
        //let mut nudge = match event.key_code {
            //ArrowLeft => Vec2::new(-1.0, 0.),
            //ArrowRight => Vec2::new(1.0, 0.),
            //ArrowUp => Vec2::new(0.0, -1.0),
            //ArrowDown => Vec2::new(0.0, 1.0),
            //_ => unreachable!(),
        //};

        //if event.mods.meta {
            //nudge *= 100.;
        //} else if event.mods.shift {
            //nudge *= 10.;
        //}
        //data.nudge_selection(DVec2::from_raw(nudge));
    //}
//}

//impl MouseDelegate<EditSession> for Select {
    //fn mouse_moved(&mut self, _data: &mut EditSession, event: &MouseEvent) -> bool {
        //self.last_pos = event.pos;
        //false
    //}

    //fn left_down(&mut self, data: &mut EditSession, event: &MouseEvent) -> bool {
        //if event.count == 1 {
            //let sel = data
                //.iter_items_near_point(event.pos, MIN_POINT_DISTANCE)
                //.next();
            //if let Some(point_id) = sel {
                //if !event.mods.shift {
                    //// when clicking a point, if it is not selected we set it as the selection,
                    //// otherwise we keep the selection intact for a drag.
                    //if !data.selection.contains(&point_id) {
                        //data.selection_mut().clear();
                        //data.selection_mut().insert(point_id);
                    //}
                //} else if !data.selection_mut().remove(&point_id) {
                    //data.selection_mut().insert(point_id);
                //}
            //} else if !event.mods.shift {
                //data.selection_mut().clear();
            //}
        //} else if event.count == 2 {
            //let sel = data
                //.iter_items_near_point(event.pos, MIN_POINT_DISTANCE)
                //.next()
                //.clone();
            //match sel {
                //Some(id)
                    //if data
                        //.path_point_for_id(id)
                        //.map(|p| p.is_on_curve())
                        //.unwrap_or(false) =>
                //{
                    //data.toggle_selected_on_curve_type()
                //}
                //Some(id) if id.is_guide() => data.toggle_guide(id, event.pos),
                //_ => {
                    //data.select_path(event.pos, event.mods.shift);
                //}
            //}
        //}
        //true
    //}

    //fn left_up(&mut self, _data: &mut EditSession, _event: &MouseEvent) -> bool {
        //self.prev_selection = None;
        //self.drag_rect = None;
        //true
    //}

    //fn left_drag_began(&mut self, data: &mut EditSession, drag: Drag) -> bool {
        //self.prev_selection = if data
            //.iter_items_near_point(drag.start.pos, MIN_POINT_DISTANCE)
            //.next()
            //.is_some()
        //{
            //None
        //} else {
            //Some(data.selection.clone())
        //};
        //true
    //}

    //fn left_drag_changed(&mut self, data: &mut EditSession, drag: Drag) -> bool {
        //if let Some(prev_selection) = self.prev_selection.as_ref() {
            //let rect = Rect::from_points(drag.current.pos, drag.start.pos);
            //self.drag_rect = Some(rect);
            //self.update_selection_for_drag(data, prev_selection, rect, drag.current.mods.shift);
        //} else {
            //let last_drag_pos = self.last_drag_pos.unwrap_or(drag.start.pos);
            //let dvec = drag.current.pos - last_drag_pos;
            //let drag_vec = dvec * (1.0 / data.vport.zoom);
            //let drag_vec = DVec2::from_raw((drag_vec.x.floor(), drag_vec.y.floor()));
            //if drag_vec.hypot() > 0. {
                //// multiple small drag updates that don't make up a single point in design
                //// space should be aggregated
                //let aligned_drag_delta = drag_vec.to_screen(data.vport);
                //let aligned_last_drag = last_drag_pos + aligned_drag_delta;
                //self.last_drag_pos = Some(aligned_last_drag);
                //data.nudge_selection(drag_vec);
            //}
        //}
        //true
    //}

    //fn left_drag_ended(&mut self, _data: &mut EditSession, _drag: Drag) -> bool {
        //self.last_drag_pos = None;
        //true
    //}

    //fn cancel(&mut self, data: &mut EditSession) {
        //if let Some(prev) = self.prev_selection.take() {
            //data.selection = prev;
        //}
        //self.drag_rect = None;
    //}
//}
