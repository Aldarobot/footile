// fishy2.rs
use footile::{FillRule, PathBuilder, Plotter};
use pix::{Ch8, Format, RasterBuilder, Rgba8};
use pixops::raster_over;

pub mod png;

fn main() -> Result<(), std::io::Error> {
    let fish = PathBuilder::new().relative().pen_width(3.0)
                           .move_to(112.0, 24.0)
                           .line_to(-32.0, 24.0)
                           .cubic_to(-96.0, -48.0, -96.0, 80.0, 0.0, 32.0)
                           .line_to(32.0, 24.0)
                           .line_to(-16.0, -40.0)
                           .close().build();
    let eye = PathBuilder::new().relative().pen_width(2.0)
                          .move_to(24.0, 48.0)
                          .line_to(8.0, 8.0)
                          .move_to(0.0, -8.0)
                          .line_to(-8.0, 8.0)
                          .build();
    let v = vec![
        Rgba8::with_rgba([Ch8::new(0), Ch8::new(0), Ch8::new(0), Ch8::new(0)]); 128 * 128
    ];
    let mut p = Plotter::new(128, 128);
    let mut r = RasterBuilder::new().with_pixels(p.width(), p.height(), v);
    raster_over(&mut r, p.fill(&fish, FillRule::NonZero),
        Rgba8::new(127, 96, 96), 0, 0);
    raster_over(&mut r, p.stroke(&fish), Rgba8::new(255, 208, 208), 0, 0);
    raster_over(&mut r, p.stroke(&eye), Rgba8::new(0, 0, 0), 0, 0);
    png::write_rgba(&r, "./fishy2.png")
}