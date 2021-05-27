use nightmaregl::{Size, Sprite, Texture, Position, Transform};
use nightmaregl::pixels::{Pixels, Pixel};

use crate::Node;

pub struct Cursor {
    pub node: Node<i32>,
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
        let mut node = Node::new(&texture);
        node.sprite.z_index = 20;

        Self {
            texture,
            node,
            color: pixel,
            position,
        }
    }
}
