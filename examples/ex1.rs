use std::fs::File;
use turtle_crab::{Canvas, Turtle};

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
    t.save_svg(&mut File::create("test.svg").unwrap(), None)
        .unwrap();
    t.save_eps(&mut File::create("test.eps").unwrap(), None)
        .unwrap();
}
