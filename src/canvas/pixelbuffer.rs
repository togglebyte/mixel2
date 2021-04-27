use nightmaregl::pixels::{Pixels, Pixel};
use nightmaregl::Size;

pub struct PixelBuffer(pub Pixels<Pixel>);

impl PixelBuffer {
    pub fn new(pixel: Pixel, size: Size<usize>) -> Self {
        let pixels = Pixels::from_pixel(pixel, size);
        Self(pixels)
    }
}
