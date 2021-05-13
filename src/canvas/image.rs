use std::path::Path;

use anyhow::Result;
use nightmaregl::{Position, Size, Sprite, VertexData};

use super::layer::Layer;

pub struct Image {
    sprite: Sprite<i32>, // TODO: remove this once it's on the Container
    pub layers: Vec<Layer>,
}

impl Image {
    pub(super) fn new(size: Size<i32>) -> Self {
        let layer = Layer::new(size);
        let mut sprite = Sprite::new(&layer.texture);

        Self {
            layers: vec![layer],
            sprite,
        }
    }

    pub(super) fn from_disk(path: impl AsRef<Path>) -> Result<Image> {
        // 1. Read a png file = 1 layer, 1 texture
        // 2. Mixel format: lots of layers and misc
        unimplemented!()
    }

    pub(super) fn vertex_data(&self) -> [VertexData; 1] {
        [self.sprite.vertex_data()]
    }
}
