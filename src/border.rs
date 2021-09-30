use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use nightmare::texture::Texture;
use nightmare::{Size, Viewport, VertexData, Context, Transform};
use nightmare::render2d::{SimpleRenderer, Model};

use crate::Node;

const BORDER_ZINDEX: i32 = 999;

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
pub struct Textures(pub HashMap<BorderType, Texture>);

impl Textures {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for Textures {
    type Target = HashMap<BorderType, Texture>;

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
    top: Node,
    right: Node,
    bottom: Node,
    left: Node,
}

impl Border {
    pub fn new(border_type: BorderType, textures: &Textures, viewport: &Viewport) -> Self {
        let texture = &textures[&border_type];

        let mut top = Node::new(texture);
        top.sprite.z_index = BORDER_ZINDEX;
        top.sprite.size = Size::new(viewport.size().width, 4);
        top.transform.translation.y = viewport.size().height - 4;

        let mut right = Node::new(texture);
        right.sprite.z_index = BORDER_ZINDEX;
        right.sprite.size = Size::new(4, viewport.size().height);
        right.transform.translation.x = viewport.size().width - 4;

        let mut bottom = Node::new(texture);
        bottom.sprite.z_index = BORDER_ZINDEX;
        bottom.sprite.size = Size::new(viewport.size().width, 4);

        let mut left = Node::new(texture);
        left.sprite.z_index = BORDER_ZINDEX;
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

    fn vertex_data(&self, parent: &Transform) -> [VertexData; 4] {
        [
            self.top.relative_vertex_data(parent),
            self.right.relative_vertex_data(parent),
            self.bottom.relative_vertex_data(parent),
            self.left.relative_vertex_data(parent),
        ]
    }

    pub fn render(
        &self,
        parent: &Transform,
        textures: &Textures,
        viewport: &Viewport,
        renderer: &SimpleRenderer<Model>,
        context: &mut Context,
    ) -> Result<()> {
        let texture = &textures[&self.border_type];
        renderer.render(
            texture,
            &self.vertex_data(parent),
            viewport,
            context,
        )?;

        Ok(())
    }
}
