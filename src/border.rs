use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use nightmaregl::texture::Texture;
use nightmaregl::{Size, Viewport, Renderer, VertexData, Context, Transform};

use crate::Node;

// -----------------------------------------------------------------------------
//     - Border type -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BorderType {
    Canvas,
    Active,
    Inactive,
}

// -----------------------------------------------------------------------------
//     - Textures -
// -----------------------------------------------------------------------------
pub struct Textures(pub HashMap<BorderType, Texture<i32>>);

impl Textures {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for Textures {
    type Target = HashMap<BorderType, Texture<i32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Textures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// -----------------------------------------------------------------------------
//     - Border -
// -----------------------------------------------------------------------------
pub struct Border {
    pub border_type: BorderType,
    top: Node<i32>,
    right: Node<i32>,
    bottom: Node<i32>,
    left: Node<i32>,
}

impl Border {
    pub fn new(border_type: BorderType, textures: &Textures, viewport: &Viewport) -> Self {
        let texture = &textures[&border_type];

        let mut top = Node::new(texture);
        top.sprite.z_index = 9999;
        top.sprite.size = Size::new(viewport.size().width, 4);
        top.transform.translation.y = viewport.size().height - 4;

        let mut right = Node::new(texture);
        right.sprite.z_index = 9999;
        right.sprite.size = Size::new(4, viewport.size().height);
        right.transform.translation.x = viewport.size().width - 4;

        let mut bottom = Node::new(texture);
        bottom.sprite.z_index = 9999;
        bottom.sprite.size = Size::new(viewport.size().width, 4);

        let mut left = Node::new(texture);
        left.sprite.z_index = 9999;
        left.sprite.size = Size::new(4, viewport.size().height);

        Self {
            border_type,
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn resize(&mut self, viewport: &Viewport) {
        self.top.sprite.size = Size::new(viewport.size().width, 4);
        self.right.sprite.size = Size::new(4, viewport.size().height);
        self.bottom.sprite.size = Size::new(viewport.size().width, 4);
        self.left.sprite.size = Size::new(4, viewport.size().height);
        self.right.transform.translation.x = viewport.size().width - 4;
        self.top.transform.translation.y = viewport.size().height - 4;
    }

    fn vertex_data(&self, parent: &Transform<i32>) -> [VertexData; 4] {
        [
            self.top.relative_vertex_data(parent),
            self.right.relative_vertex_data(parent),
            self.bottom.relative_vertex_data(parent),
            self.left.relative_vertex_data(parent),
        ]
    }

    pub fn render(
        &self,
        parent: &Transform<i32>,
        textures: &Textures,
        viewport: &Viewport,
        renderer: &Renderer<VertexData>,
        context: &mut Context,
    ) {
        let texture = &textures[&self.border_type];
        let _ = renderer.render(
            texture,
            &self.vertex_data(parent),
            viewport,
            context,
        );
    }
}
