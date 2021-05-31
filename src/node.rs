use std::ops::{Div, MulAssign};

use nightmaregl::{VertexData, Sprite, Transform, Texture};
use nalgebra::Scalar;
use num_traits::cast::NumCast;
use num_traits::{One, Zero};

/// Node to keep track of sprites and transforms.
pub struct Node<T: Copy + NumCast + Zero + MulAssign + Default + Scalar + Div<Output = T>> {
    pub sprite: Sprite<T>,
    pub transform: Transform<T>,
}

impl<T> Node<T>
    where T: Copy + NumCast + Zero + One + MulAssign + Default + Scalar + Div<Output = T>
{
    /// Creata `Node` from a texture
    pub fn new(texture: &Texture<T>) -> Self {
        let sprite = Sprite::new(texture);
        let transform = Transform::default();

        Self {
            sprite,
            transform,
        }
    }

    /// Create a `Node` from a sprite
    pub fn from_sprite(sprite: Sprite<T>) -> Self {
        Self {
            sprite,
            transform: Transform::default(),
        }
    }

    /// Get vertex data
    pub fn vertex_data(&self) -> VertexData {
        VertexData::new(&self.sprite, &self.transform)
    }

    /// Pass in the parent nodes transform
    pub fn relative_vertex_data(&self, transform: &Transform<T>) -> VertexData {
        let mut vd = self.vertex_data();
        let parent = transform.matrix();
        vd.model = parent * vd.model;
        vd
    }
}
