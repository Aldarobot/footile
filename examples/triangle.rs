// triangle.rs
extern crate footile;

use footile::{FillRule,PathBuilder,Plotter};

fn main() -> Result<(), std::io::Error> {
    let path = PathBuilder::new().absolute()
                           .move_to(0.75 * 84.0, 0.75 * 64.0)
                           .line_to(0.25 * 84.0, 0.75 * 64.0)
                           .line_to(0.5 * 84.0, 0.25 * 64.0)
                           .line_to(0.75 * 84.0, 0.75 * 64.0)
                           .close().build();
    let mut p = Plotter::new(84, 64);
    p.fill(&path, FillRule::NonZero).write_png("./triangle.png")
}
