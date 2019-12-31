// quad.rs     Example plotting a quadratic bézier spline.
use footile::{PathBuilder, Plotter};

pub mod png;

fn main() -> Result<(), std::io::Error> {
    let path = PathBuilder::new()
        .relative()
        .pen_width(2.0)
        .move_to(0.0, 16.0)
        .quad_to(100.0, 16.0, 0.0, 32.0)
        .build();
    let mut p = Plotter::new(64, 64);
    png::write_mask(p.stroke(&path), "./quad.png")
}
