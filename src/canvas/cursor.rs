use nightmaregl::{Size, Sprite, Texture, Pixels, Pixel};

pub struct Cursor {
    pub sprite: Sprite<i32>,
    pub texture: Texture<i32>,
}

impl Cursor {
    pub fn new() -> Self {
        let size = Size::new(1, 1);
        let pixels = Pixels::from_pixel(Pixel { g: 200, ..Default::default() }, size.cast());
        let texture = Texture::default_with_data(size, pixels.as_bytes());
        let mut sprite = Sprite::new(&texture);
        sprite.z_index = 20;

        Self {
            texture,
            sprite,
        }
    }
}
