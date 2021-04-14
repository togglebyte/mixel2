use nightmaregl::{Position, Size, Sprite, Texture, Pixels, Pixel};

pub struct Cursor {
    size: Size<i32>,
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
            size: Size::new(1, 1),
            texture,
            sprite,
        }
    }

    pub fn move_by(&mut self, offset: Position<i32>) {
        self.sprite.position += offset;
    }

    pub fn position(&mut self, position: Position<i32>) {
    }
}
