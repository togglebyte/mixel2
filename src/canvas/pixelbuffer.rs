use nightmaregl::{Pixels, Pixel, Size};

pub struct PixelBuffer(pub Pixels);

impl PixelBuffer {
    pub fn new(pixel: Pixel, size: Size<usize>) -> Self {
        let pixels = Pixels::from_pixel(pixel, size);
        Self(pixels)
    }
}
