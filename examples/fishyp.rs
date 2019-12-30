// fishyp.rs
extern crate footile;

use footile::{FillRule,PathBuilder,Plotter,Raster,Rgba8,PixFmt};

struct NonOwnedRaster<P: PixFmt>(Option<Raster<P>>);

impl<P: PixFmt> NonOwnedRaster<P> {
    fn new(pixels: &mut [P], width: u32, height: u32) -> Self {
        // Safely convert our slice into a Box<[T]>, then into a Vec<T>.
        // This is safe because slice & box are fat ptrs.
        let v: Box<[P]> = unsafe { std::mem::transmute::<_, Box<[P]>>(pixels) };
        NonOwnedRaster(Some(Raster::with_pixels(width, height, v)))
    }

    fn raster(&mut self) -> &mut Raster<P> {
        self.0.as_mut().unwrap()
    }
}

impl<P: PixFmt> Drop for NonOwnedRaster<P> {
    fn drop(&mut self) {
        // Convert raster back to slice to avoid double free.
        let b: Box<[P]> = self.0.take().unwrap().into();
        let _: &mut [P] = unsafe { std::mem::transmute(b) };
    }
}

fn main() -> Result<(), std::io::Error> {
    // Emulate non-owned, aligned mutable pointer to Vulkan Buffer:
    let mut array: [[u8; 4]; 128 * 128] = [[0, 0, 0, 0]; 128 * 128];
    let buffer: *mut u8 = &mut array[0][0];

    // Turn non-owned, aligned mutable pointer into a mutable slice.
    let slice: &mut [Rgba8] = unsafe {
        std::slice::from_raw_parts_mut(buffer.cast(), 128 * 128)
    };

    // Create the safely-unowned raster.
    let mut r = NonOwnedRaster::new(slice, 128, 128);

    // Draw on the buffer.
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
    let mut p = Plotter::new(128, 128);
    r.raster().over(p.fill(&fish, FillRule::NonZero), Rgba8::rgb(127, 96, 96));
    r.raster().over(p.stroke(&fish), Rgba8::rgb(255, 208, 208));
    r.raster().over(p.stroke(&eye), Rgba8::rgb(0, 0, 0));
    r.raster().write_png("./fishyp.png")?;

    Ok(())
}
