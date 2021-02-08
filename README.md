# turtle

A turtle graphics engine for Rust. Generates Scalable Vector Graphics (SVG).

Forked from https://github.com/mneumann/turtle-graphics-rs.  Thank you to the original author!

This version primarily supports [Terkwood/forest](https://github.com/Terkwood/forest).

## Example

```rust
use turtle::{Canvas, Turtle, SvgParams};

fn main() {
    let mut t = Canvas::new();
    // move the turtle 100.0 points upwards
    t.forward(100.0);
    // rotate the head of the turtle by 90 degree to the right
    t.right(90.0);
    // move 100.0 forward again (now to the right).
    t.forward(100.0);
    // ...

    // write the graphic (SVG) to stdout.
    t.save_svg(&mut std::io::stdout(), SvgParams::default()).unwrap();
}
```
