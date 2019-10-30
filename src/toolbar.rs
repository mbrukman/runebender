//! The toolbar.

use druid::kurbo::{Affine, BezPath, Line, Point, Rect, Shape, Size};
use druid::piet::RenderContext;
use druid::{Env, HotKey, PaintCtx};
//use crate::widgets::ToolbarItem;
use crate::theme;

const TOOLBAR_HEIGHT: f64 = 32.;
const TOOLBAR_ICON_PADDING: f64 = 4.;
const TOOLBAR_ITEM_WIDTH: f64 = 32.;

#[derive(Debug, Clone)]
pub struct ToolbarItem {
    pub name: String,
    pub hotkey: Option<HotKey>,
    pub icon: BezPath,
}

#[derive(Debug, Default, Clone)]
pub struct Toolbar {
    hot: Option<usize>,
    selected: usize,
    items: Vec<ToolbarItem>,
}

impl ToolbarItem {
    pub fn new(name: impl Into<String>, hotkey: impl Into<Option<HotKey>>, icon: BezPath) -> Self {
        let padding = TOOLBAR_ICON_PADDING * 2.;
        let icon = scale_path(
            &icon,
            (TOOLBAR_ITEM_WIDTH - padding, TOOLBAR_HEIGHT - padding),
        );
        ToolbarItem {
            name: name.into(),
            hotkey: hotkey.into(),
            icon,
        }
    }

    pub fn select() -> Self {
        let path = select_tool_icon_path();
        ToolbarItem::new("select", HotKey::new(None, "v"), path)
    }

    pub fn pen() -> Self {
        let path = pen_tool_icon_path();
        ToolbarItem::new("pen", HotKey::new(None, "p"), path)
    }
}

impl Toolbar {
    pub fn new(items: Vec<ToolbarItem>) -> Self {
        Toolbar {
            hot: None,
            selected: 0,
            items,
        }
    }

    pub fn paint(&self, ctx: &mut PaintCtx, env: &Env) {
        let total_size = Size::new(TOOLBAR_ITEM_WIDTH * self.items.len() as f64, TOOLBAR_HEIGHT);
        let total_bounds = Rect::ZERO.with_size(total_size);
        ctx.stroke(total_bounds, &env.get(theme::TOOLBAR_BORDER_COLOR), 1.0);

        let item_size = Size::new(TOOLBAR_ITEM_WIDTH, TOOLBAR_HEIGHT);

        // the trailing edge of the last drawn button, for drawing separators if needed.
        let mut last = None;

        for (i, tool) in self.items.iter().enumerate() {
            let bg = if i == self.selected {
                env.get(theme::TOOLBAR_BG_SELECTED_COLOR)
            } else if Some(i) == self.hot {
                env.get(theme::TOOLBAR_BG_HOVER_COLOR)
            } else {
                env.get(theme::TOOLBAR_BG_COLOR)
            };

            let fg = if i == self.selected {
                env.get(theme::TOOLBAR_ICON_SELECTED_COLOR)
            } else {
                env.get(theme::TOOLBAR_ICON_COLOR)
            };

            let item_rect = Rect::from_origin_size((i as f64 * TOOLBAR_ITEM_WIDTH, 0.), item_size);
            ctx.fill(item_rect, &bg);

            let icon_size = tool.icon.bounding_box().size();
            let x_pad = TOOLBAR_ICON_PADDING.max((TOOLBAR_ITEM_WIDTH - icon_size.width) * 0.5);
            let y_pad = TOOLBAR_ICON_PADDING.max((TOOLBAR_HEIGHT - icon_size.height) * 0.5);
            let tool_pos = Affine::translate((x_pad + i as f64 * TOOLBAR_ITEM_WIDTH, y_pad));
            ctx.fill(tool_pos * &tool.icon, &fg);

            if let Some(last) = last {
                let line = Line::new((last, 0.), (last, TOOLBAR_HEIGHT));
                ctx.stroke(line, &env.get(theme::TOOLBAR_BORDER_COLOR), 0.5);
            }

            last = Some((i + 1) as f64 * TOOLBAR_ITEM_WIDTH);
        }
    }
}

fn scale_path(path: &BezPath, fitting_size: impl Into<Size>) -> BezPath {
    let mut out = path.clone();
    let fitting_size = fitting_size.into();
    let path_size = path.bounding_box().size();
    let scale_factor =
        (fitting_size.width / path_size.width).min(fitting_size.height / path_size.height);
    out.apply_affine(Affine::scale(scale_factor));
    let translation = Point::ZERO - out.bounding_box().origin();
    out.apply_affine(Affine::translate(translation));
    out
}

fn select_tool_icon_path() -> BezPath {
    let mut path = BezPath::new();
    path.move_to((45., 100.));
    path.line_to((55., 100.));
    path.line_to((55., 70.));
    path.line_to((80., 70.));
    path.line_to((50., 10.));
    path.line_to((20., 70.));
    path.line_to((45., 70.));
    path.close_path();
    path.apply_affine(Affine::rotate(-0.5));
    path
}

fn pen_tool_icon_path() -> BezPath {
    let mut path = BezPath::new();
    path.move_to((173., 0.));
    path.line_to((277., 0.));
    path.line_to((277., 93.));
    path.curve_to((277., 93.), (364., 186.), (364., 265.));
    path.curve_to((364., 344.), (255., 481.), (255., 481.));
    path.curve_to((255., 481.), (86., 344.), (86., 265.));
    path.curve_to((86., 186.), (173., 93.), (173., 93.));
    path.close_path();
    path.apply_affine(Affine::rotate(-3.5));
    path
}
