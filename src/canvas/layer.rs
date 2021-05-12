use nightmaregl::{Sprite, Size, Position};
use nightmaregl::texture::Texture;
use nightmaregl::pixels::{Pixel, Pixels};

// -----------------------------------------------------------------------------
//     - Layers -
// -----------------------------------------------------------------------------
pub struct Layer {
    pub texture: Texture<i32>,
    pub buffer: Pixels<Pixel>,
    dirty: bool,
}

impl Layer {
    pub fn new(size: Size<i32>) -> Self {
        let buffer = Pixels::from_pixel(Pixel::transparent(), size.cast());
        let texture = Texture::default_with_data(size, buffer.as_bytes());
        Self { texture, buffer, dirty: false }
    }

    pub fn push_pixel(&mut self, pixel: Pixel, position: Position<i32>) {
        self.buffer.insert_pixel(pixel, position.cast());
        self.dirty = true;
    }

    fn draw_to_texture(&mut self) {
        self.texture.write_region(
            Position::zero(),
            self.buffer.size().cast(),
            self.buffer.as_bytes(),
        );

        self.dirty = false;
    }
}

