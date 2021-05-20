use nightmaregl::{Size, Sprite, Texture, Position};
use nightmaregl::pixels::{Pixels, Pixel};

pub struct Cursor {
    pub sprite: Sprite<i32>,
    pub texture: Texture<i32>,
    pub color: Pixel,
    pub position: Position<i32>,
}

impl Cursor {
    pub fn new(position: Position<i32>) -> Self {
        let size = Size::new(1, 1);
        let pixel = Pixel::black();
        let pixels = Pixels::from_pixel(pixel, size.cast());
        let texture = Texture::default_with_data(size, pixels.as_bytes());
        let mut sprite = Sprite::new(&texture);

        sprite.z_index = 20;

        Self {
            texture,
            sprite,
            color: pixel,
            position,
        }
    }
}
