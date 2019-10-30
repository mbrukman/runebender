//! the main editor widget.

use druid::kurbo::{Affine, Point, Rect, Size};
use druid::piet::{Color, RenderContext};
use druid::{
    BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
};

use crate::data::EditorState;
use crate::draw;
use crate::toolbar::{Toolbar, ToolbarItem};

/// The root widget of the glyph editor window.
pub struct Editor {
    toolbar: Toolbar,
}

pub const CANVAS_SIZE: Size = Size::new(5000., 5000.);
const TOOLBAR_POSITION: Point = Point::new(8., 8.);

impl Editor {
    pub fn new() -> Editor {
        Editor {
            toolbar: Toolbar::new(vec![ToolbarItem::select(), ToolbarItem::pen()]),
        }
    }

    fn paint_toolbar(&mut self, ctx: &mut PaintCtx, env: &Env) {
        if let Err(e) = ctx.save() {
            log::error!("saving render context failed: {:?}", e);
            return;
        }

        let offset = ctx.region().to_rect().origin().to_vec2() + TOOLBAR_POSITION.to_vec2();
        ctx.transform(Affine::translate(offset));
        self.toolbar.paint(ctx, env);

        if let Err(e) = ctx.restore() {
            log::error!("restoring render context failed: {:?}", e);
        }
    }
}

impl Widget<EditorState> for Editor {
    fn paint(&mut self, ctx: &mut PaintCtx, _: &BaseState, data: &EditorState, env: &Env) {
        let rect =
            Rect::ZERO.with_size((CANVAS_SIZE.to_vec2() * data.session.viewport.zoom).to_size());
        ctx.fill(rect, &Color::WHITE);

        draw::draw_session(
            ctx,
            data.session.viewport,
            ctx.region().to_rect(),
            &data.metrics,
            &data.session,
            &data.ufo,
        );

        self.paint_toolbar(ctx, env);
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        data: &EditorState,
        _env: &Env,
    ) -> Size {
        (CANVAS_SIZE.to_vec2() * data.session.viewport.zoom).to_size()
    }

    fn event(&mut self, _event: &Event, _ctx: &mut EventCtx, _data: &mut EditorState, _env: &Env) {}

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old: Option<&EditorState>,
        new: &EditorState,
        _env: &Env,
    ) {
        if !Some(new).same(&old) {
            ctx.invalidate();
        }
    }
}
