use nightmare::{Size, Texture, Position};
use nightmare::pixels::{Pixels, Pixel};

use crate::{Coords, Node};

pub struct Cursor {
    pub node: Node,
    pub texture: Texture,
    pub coords: Coords,
    pub visible: bool,
    colour: Pixel,
}

impl Cursor {
    pub fn new(coords: Coords, sprite_offset: Position) -> Self {
        let size = Size::new(1, 1);
        let pixel = Pixel::black();
        let pixels = Pixels::from_pixel(pixel, size.cast());
        let texture = Texture::default_with_data(size, pixels.as_bytes());
        let mut node = Node::new(&texture);
        node.sprite.z_index = 20;
        node.sprite.anchor = sprite_offset;

        Self {
            texture,
            node,
            colour: pixel,
            coords,
            visible: true,
        }
    }

    pub fn set_colour(&mut self, colour: Pixel) {
        let size = Size::new(1usize, 1);
        self.texture.write_region(Position::zero(), size.cast(), Pixels::from_pixel(colour, size).as_bytes());
    }
}
