use std::path::Path;

use anyhow::Result;
use nightmaregl::{Position, Size};
use nightmaregl::pixels::Pixel;

use super::layer::Layer;

// -----------------------------------------------------------------------------
//     - Image -
// -----------------------------------------------------------------------------
pub struct Image {
    pub layers: Vec<Layer>,
    pub selected_layer: usize,
}

impl Image {
    pub(super) fn new(size: Size<i32>) -> Self {
        Self {
            layers: vec![Layer::new(size)],
            selected_layer: 0,
        }
    }

    pub(super) fn from_disk(path: impl AsRef<Path>) -> Result<Image> {
        // 1. Read a png file = 1 layer, 1 texture
        // 2. Mixel format: lots of layers and misc
        unimplemented!()
    }

    pub(super) fn put_pixel(&mut self, pixel: Pixel, pos: Position<i32>) {
        self.layers[self.selected_layer].push_pixel(pixel, pos);
        self.layers[self.selected_layer].draw_to_texture();
    }

    pub(super) fn clear_pixel(&mut self, pos: Position<i32>) {
        self.layers[self.selected_layer].push_pixel(Pixel::transparent(), pos);
        self.layers[self.selected_layer].draw_to_texture();
    }
}
