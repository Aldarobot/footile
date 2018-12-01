// triangle.rs
extern crate footile;

use footile::{FillRule,PathBuilder,Plotter};

fn main() -> Result<(), std::io::Error> {
    let path = PathBuilder::new().absolute()
                           .move_to(53.0, 48.0)
                           .line_to(11.0, 48.0)
                           .line_to(32.0, 16.0)
                           .line_to(53.0, 48.0)
                           .close().build();
    let mut p = Plotter::new(64, 64);
    p.fill(&path, FillRule::NonZero).write_png("./triangle.png")
}
