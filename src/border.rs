use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use nightmaregl::pixels::{Pixel, Pixels};
use nightmaregl::texture::{Format, Texture};
use nightmaregl::{Rect, Size, Sprite, Viewport, Renderer, VertexData, Context};

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
    top: Sprite<i32>,
    right: Sprite<i32>,
    bottom: Sprite<i32>,
    left: Sprite<i32>,
}

impl Border {
    pub fn new(border_type: BorderType, textures: &Textures, viewport: &Viewport) -> Self {
        let texture = &textures[&border_type];

        let mut top = Sprite::new(texture);
        top.size = Size::new(viewport.size().width, 4);
        top.position.y = viewport.size().height - 4;

        let mut right = Sprite::new(texture);
        right.size = Size::new(4, viewport.size().height);
        right.position.x = viewport.size().width - 4;

        let mut bottom = Sprite::new(texture);
        bottom.size = Size::new(viewport.size().width, 4);

        let mut left = Sprite::new(texture);
        left.size = Size::new(4, viewport.size().height);

        Self {
            border_type,
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn resize(&mut self, viewport: &Viewport) {
        self.top.size = Size::new(viewport.size().width, 4);
        self.right.size = Size::new(4, viewport.size().height);
        self.bottom.size = Size::new(viewport.size().width, 4);
        self.left.size = Size::new(4, viewport.size().height);
        self.right.position.x = viewport.size().width - 4;
        self.top.position.y = viewport.size().height - 4;
    }

    fn vertex_data(&self) -> [VertexData; 4] {
        [
            self.top.vertex_data(),
            self.right.vertex_data(),
            self.bottom.vertex_data(),
            self.left.vertex_data(),
        ]
    }

    pub fn render(
        &self,
        textures: &Textures,
        viewport: &Viewport,
        renderer: &Renderer<VertexData>,
        context: &mut Context,
    ) {

        let texture = &textures[&self.border_type];
        let _ = renderer.render(
            texture,
            &self.vertex_data(),
            viewport,
            context,
        );
    }
}
