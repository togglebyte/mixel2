use nightmaregl::{Size, Texture, Position};
use nightmaregl::pixels::{Pixels, Pixel};

use crate::Node;

pub struct Cursor {
    pub node: Node<i32>,
    pub texture: Texture<i32>,
    pub position: Position<i32>,
    pub visible: bool,
    colour: Pixel,
}

impl Cursor {
    pub fn new(position: Position<i32>) -> Self {
        let size = Size::new(1, 1);
        let pixel = Pixel::black();
        let pixels = Pixels::from_pixel(pixel, size.cast());
        let texture = Texture::default_with_data(size, pixels.as_bytes());
        let mut node = Node::new(&texture);
        node.sprite.z_index = 20;

        Self {
            texture,
            node,
            colour: pixel,
            position,
            visible: true,
        }
    }

    pub fn set_colour(&mut self, colour: Pixel) {
        let size = Size::new(1usize, 1);
        self.texture.write_region(Position::zero(), size.cast(), Pixels::from_pixel(colour, size).as_bytes());
    }
}
