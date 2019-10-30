//! A tool accepts user input and modifies the canvas.

//mod select;

use crate::mouse::MouseDelegate;
use crate::data::EditorState;
use druid::{PaintCtx, KeyEvent, EventCtx, Env};

pub trait Tool: MouseDelegate<EditorState> {
    const NAME: &'static str;
    #[allow(unused)]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditorState, env: &Env) {}
    #[allow(unused)]
    fn key_down(&mut self, event: &KeyEvent, data: &mut EditorState, env: &Env, ctx: &mut EventCtx) -> bool { false }
}
