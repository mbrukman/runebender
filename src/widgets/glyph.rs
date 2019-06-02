//! A widget that draws a glyph

use kurbo::{Affine, BezPath, Rect, Shape, Vec2};
use norad::glyph::{Contour, ContourPoint, Glyph, PointType};
use piet::{FillRule, RenderContext};
use druid::{
    BoxConstraints, Geometry, HandlerCtx, Id, LayoutResult, LayoutCtx, MouseEvent, PaintCtx, Ui, Widget,
};

//const CELL_BG_COLOR: u32 =  0x_24_24_24_ff;
const GLYPH_COLOR: u32 =  0x6a_6a_6a_ff;
const HIGHLIGHT_COLOR: u32 =  0xfa_fa_fa_ff;
const ON_CLICK_COLOR: u32 =  0x_24_24_84_ff;

#[allow(dead_code)]
pub struct GlyphWidget {
    glyph: Glyph,
    path: BezPath,
}

impl GlyphWidget {
    pub fn new(glyph: Glyph) -> Self {
        let path = path_for_glyph(&glyph);
        GlyphWidget { glyph, path }
    }

    pub fn ui(self, ctx: &mut Ui) -> Id {
        ctx.add(self, &[])
    }
}

impl Widget for GlyphWidget {
    fn paint(&mut self, ctx: &mut PaintCtx, geom: &Geometry) {

        let is_active = ctx.is_active();
        let is_hot = ctx.is_hot();
        let (x, y) = geom.pos;
        let (width, height) = geom.size;
        let rect = Rect::new(
            x as f64,
            y as f64,
            x as f64 + width as f64,
            y as f64 + height as f64,
            );

        if is_active {
            let brush = ctx.render_ctx.solid_brush(ON_CLICK_COLOR).unwrap();
            ctx.render_ctx.fill(rect, &brush, FillRule::NonZero);
        }

        let bb = self.path.bounding_box();
        let scale = geom.size.1 as f64 / 1000.;
        let scale = scale * 0.85; // some margins around glyphs
        let scaled_width = bb.width() * scale as f64;
        let l_pad = ((geom.size.0 as f64 - scaled_width) / 2.).round();
        let baseline = (geom.size.1 * 0.16) as f64;
        let affine = Affine::new([
                                 scale as f64,
                                 0.0,
                                 0.0,
                                 -scale as f64,
                                 x as f64 + l_pad,
                                 (height +  y) as f64 - baseline,
        ]);
        let fill_color = if is_active { HIGHLIGHT_COLOR } else { GLYPH_COLOR };
        let fill = ctx.render_ctx.solid_brush(fill_color).unwrap();
        ctx.render_ctx.fill(affine * &self.path, &fill, FillRule::NonZero);
        if is_hot {
            let outline_color = ctx.render_ctx.solid_brush(HIGHLIGHT_COLOR).unwrap();
            //eprintln!("{} {:?} x{} {:?}", self.glyph.name, bb, scale, rect);
            ctx.render_ctx.stroke(affine * &self.path, &outline_color, 0.5, None);
            ctx.render_ctx.stroke(rect, &outline_color, 0.5, None);
        }
    }

    fn layout(
        &mut self,
        bc: &BoxConstraints,
        _children: &[Id],
        _size: Option<(f32, f32)>,
        _ctx: &mut LayoutCtx,
    ) -> LayoutResult {
        LayoutResult::Size(bc.max())
    }

    fn on_hot_changed(&mut self, _hot: bool, ctx: &mut HandlerCtx) {
        ctx.invalidate();
    }

    fn mouse(&mut self, event: &MouseEvent, ctx: &mut HandlerCtx) -> bool {
        if event.count > 0 {
            ctx.set_active(true);
        } else {
            ctx.set_active(false);
            if ctx.is_hot() {
                ctx.send_event(true);
            }
        }
        ctx.invalidate();
        true
    }
}

pub fn path_for_glyph(glyph: &Glyph) -> BezPath {
    /// An outline can have multiple contours, which correspond to subpaths
    fn add_contour(path: &mut BezPath, contour: &Contour) {
        let mut close: Option<&ContourPoint> = None;

        if contour.points.is_empty() { return; }

        let first = &contour.points[0];
        path.moveto((first.x as f64, first.y as f64));
        if first.typ != PointType::Move {
            close = Some(first);
        }

        let mut idx = 1;
        let mut controls = Vec::with_capacity(2);

        let mut add_curve = |to_point: Vec2, controls: &mut Vec<Vec2>| {
            match controls.as_slice() {
                &[] => path.lineto(to_point),
                &[a] => path.quadto(a, to_point),
                &[a, b] => path.curveto(a, b, to_point),
                _illegal => panic!("existence of second point implies first"),
            };
            controls.clear();
        };

        while idx < contour.points.len() {
            let next = &contour.points[idx];
            let point: Vec2 = (next.x as f64, next.y as f64).into();
            match next.typ {
                PointType::OffCurve => controls.push(point),
                PointType::Line => {
                    debug_assert!(controls.is_empty(), "line type cannot follow offcurve");
                    add_curve(point, &mut controls);
                }
                PointType::Curve => add_curve(point, &mut controls),
                PointType::QCurve => {
                    eprintln!("TODO: handle qcurve");
                    add_curve(point, &mut controls);
                }
                PointType::Move => debug_assert!(false, "illegal move point in path?"),
            }
            idx += 1;
        }

        if let Some(to_close) = close.take() {
            add_curve((to_close.x as f64, to_close.y as f64).into(), &mut controls);
        }
    }

    let mut path = BezPath::new();
    if let Some(outline) = glyph.outline.as_ref() {
        outline.contours.iter().for_each(|c| add_contour(&mut path, c));
    }
    path
}