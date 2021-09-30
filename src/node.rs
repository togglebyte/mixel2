use std::ops::{Div, MulAssign};

use nightmare::{VertexData, Sprite, Transform, Texture, create_model_matrix};
use nightmare::render2d::Model;
use nalgebra::Scalar;
use num_traits::cast::NumCast;
use num_traits::{One, Zero};

/// Node to keep track of sprites and transforms.
#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Node {
    /// Creata `Node` from a texture
    pub fn new(texture: &Texture) -> Self {
        let sprite = Sprite::new(texture);
        let transform = Transform::default();

        Self {
            sprite,
            transform,
        }
    }

    /// Create a `Node` from a sprite
    pub fn from_sprite(sprite: Sprite) -> Self {
        Self {
            sprite,
            transform: Transform::default(),
        }
    }

    /// Get the view model
    pub fn model(&self) -> Model {
        create_model_matrix(&self.sprite, &self.transform)
    }

    /// Pass in the parent nodes transform
    pub fn relative_vertex_data(&self, transform: &Transform) -> Model {
        todo!()
        // let mut model = self.model();
        // vd.make_relative(transform);
        // vd
    }
}
