use std::fs::File;
use turtle::{Canvas, SvgParams, SvgStrokeColor, SvgStrokeWidth, Turtle};

fn main() {
    let mut t = Canvas::new();
    t.forward(100.0);
    t.right(90.0);
    t.forward(100.0);
    t.pen_up();
    t.forward(10.0);
    t.pen_down();
    t.right(90.0);
    t.forward(100.0);
    t.right(90.0);
    t.forward(100.0);
    t.save_svg(
        &mut File::create("test2.svg").unwrap(),
        SvgParams {
            stroke_color: SvgStrokeColor::from("white"),
            stroke_width: SvgStrokeWidth(4.0),
        },
    )
    .unwrap();
}
