//! for missing glyphs

#![allow(non_snake_case)]

use std::ops::Deref;
use std::sync::Arc;

use druid::kurbo::BezPath;
use norad::GlyphName;

pub fn placeholder_for_glyph(name: &GlyphName) -> Option<Arc<BezPath>> {
    eprintln!("placeholder for glyph");
    match name.deref() {
        "A" => Some(Arc::new(outline_A())),
        "B" => Some(Arc::new(outline_B())),
        "C" => Some(Arc::new(outline_C())),
        "D" => Some(Arc::new(outline_D())),
        _ => {
            eprintln!("no placeholder for {}", name);
            Some(Arc::new(outline_question()))
        }
    }
}

fn outline_A() -> BezPath {
    let mut bez = BezPath::new();

    bez.move_to((36.0, 0.0));
    bez.line_to((225.0, 672.0));
    bez.line_to((255.0, 671.0));
    bez.line_to((409.0, -1.0));
    bez.line_to((358.0, -2.0));
    bez.line_to((233.0, 535.0));
    bez.line_to((80.0, 0.0));
    bez.line_to((36.0, 0.0));
    bez.close_path();

    bez.move_to((129.0, 278.0));
    bez.line_to((298.0, 282.0));
    bez.line_to((297.0, 324.0));
    bez.line_to((147.0, 319.0));
    bez.line_to((129.0, 278.0));
    bez.close_path();
    bez
}

fn outline_B() -> BezPath {
    let mut bez = BezPath::new();

    bez.move_to((40.0, 0.0));
    bez.line_to((40.0, 714.0));
    bez.curve_to((40.0, 714.0), (123.0, 713.0), (248.0, 714.0));
    bez.curve_to((373.0, 715.0), (407.0, 605.0), (407.0, 558.0));
    bez.curve_to((407.0, 511.0), (340.0, 403.0), (282.0, 406.0));
    bez.line_to((224.0, 406.0));
    bez.curve_to((224.0, 406.0), (408.0, 358.0), (408.0, 246.0));
    bez.curve_to((408.0, 134.0), (402.0, 6.0), (268.0, 2.0));
    bez.line_to((40.0, 0.0));
    bez.close_path();

    bez.move_to((108.0, 292.0));
    bez.line_to((114.0, 87.0));
    bez.curve_to((114.0, 87.0), (241.0, 89.0), (276.0, 90.0));
    bez.curve_to((311.0, 91.0), (317.0, 165.0), (316.0, 197.0));
    bez.curve_to((315.0, 229.0), (233.0, 295.0), (198.0, 295.0));
    bez.line_to((108.0, 292.0));
    bez.close_path();

    bez.move_to((106.0, 678.0));
    bez.line_to((106.0, 470.0));
    bez.curve_to((106.0, 470.0), (221.0, 469.0), (253.0, 469.0));
    bez.curve_to((285.0, 469.0), (325.0, 510.0), (325.0, 575.0));
    bez.curve_to((325.0, 640.0), (272.0, 678.0), (241.0, 678.0));
    bez.line_to((106.0, 678.0));
    bez.close_path();
    bez
}

fn outline_C() -> BezPath {
    let mut bez = BezPath::new();

    bez.move_to((430.0, 620.0));
    bez.curve_to((430.0, 620.0), (272.0, 653.0), (233.0, 653.0));
    bez.curve_to((68.0, 651.0), (18.0, 463.0), (18.0, 373.0));
    bez.curve_to((18.0, 263.0), (68.0, 2.0), (217.0, 4.0));
    bez.curve_to((348.0, 6.0), (379.0, 21.0), (407.0, 61.0));
    bez.line_to((383.0, 93.0));
    bez.curve_to((383.0, 93.0), (327.0, 81.0), (237.0, 81.0));
    bez.curve_to((128.0, 81.0), (93.0, 169.0), (93.0, 357.0));
    bez.curve_to((93.0, 525.0), (171.0, 575.0), (247.0, 575.0));
    bez.line_to((415.0, 569.0));
    bez.line_to((430.0, 620.0));
    bez.close_path();
    bez
}

fn outline_D() -> BezPath {
    let mut bez = BezPath::new();

    bez.move_to((59.0, 4.0));
    bez.line_to((59.0, 677.0));
    bez.curve_to((59.0, 677.0), (125.0, 681.0), (224.0, 681.0));
    bez.curve_to((323.0, 681.0), (463.0, 522.0), (463.0, 366.0));
    bez.curve_to((463.0, 180.0), (335.0, 0.0), (240.0, 0.0));
    bez.line_to((59.0, 4.0));
    bez.close_path();

    bez.move_to((123.0, 615.0));
    bez.line_to((117.0, 47.0));
    bez.curve_to((117.0, 47.0), (161.0, 49.0), (192.0, 50.0));
    bez.curve_to((293.0, 52.0), (396.0, 190.0), (395.0, 350.0));
    bez.curve_to((394.0, 480.0), (309.0, 616.0), (195.0, 615.0));
    bez.line_to((123.0, 615.0));
    bez.close_path();
    bez
}

fn outline_question() -> BezPath {
    let mut bez = BezPath::new();

    bez.move_to((51.0, 482.0));
    bez.line_to((112.0, 482.0));
    bez.curve_to((112.0, 482.0), (134.0, 631.0), (246.0, 631.0));
    bez.curve_to((308.0, 631.0), (356.0, 625.0), (356.0, 551.0));
    bez.curve_to((356.0, 487.0), (202.0, 432.0), (202.0, 241.0));
    bez.line_to((275.0, 241.0));
    bez.curve_to((276.0, 417.0), (430.0, 385.0), (430.0, 562.0));
    bez.curve_to((430.0, 699.0), (301.0, 700.0), (246.0, 700.0));
    bez.curve_to((201.0, 700.0), (51.0, 653.0), (51.0, 482.0));
    bez.close_path();

    bez.move_to((202.0, 172.0));
    bez.line_to((275.0, 172.0));
    bez.line_to((275.0, 105.0));
    bez.line_to((203.0, 105.0));
    bez.line_to((202.0, 172.0));
    bez.close_path();
    bez
}
