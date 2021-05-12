use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Context, Position, Renderer, Size, Sprite, VertexData, Viewport};

use super::layer::Layer;
use crate::binarytree::{Node, NodeId, Tree};
use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Horz,
    Vert,
}

// -----------------------------------------------------------------------------
//     - Containers -
// -----------------------------------------------------------------------------
pub struct Containers {
    inner: Tree<Container>,
    selected: NodeId,
}

// TODO: Map Container to a specific texture + layers
impl Containers {
    pub(crate) fn new(viewport: Viewport, ctx: &mut MessageCtx) -> Result<Self> {
        let viewport = Viewport::new(
            viewport.position + Position::new(10, 10),
            *viewport.size() - Size::new(20, 20),
        );
        let mut container = Container::new(viewport, Direction::Horz, ctx, None)?;

        let tree = Tree::new(container);
        let inst = Self {
            selected: tree.root_id(),
            inner: tree,
        };

        Ok(inst)
    }

    pub(crate) fn split(&mut self, dir: Direction, ctx: &mut MessageCtx) {
        let (size, pos, image) = {
            let selected = &self.inner[self.selected];
            (selected.viewport.size(), selected.viewport.position, selected.image)
        };

        let (left, right) = match dir {
            Direction::Horz => {
                let right = Viewport::new(
                    pos,
                    Size::new(size.width, size.height / 2), // - Size::new(10, 10)
                );

                let left = Viewport::new(
                    Position::new(pos.x, pos.y + size.height / 2 /* + 10*/),
                    Size::new(size.width, size.height / 2), // - Size::new(10, 10)
                );

                (left, right)
            }
            Direction::Vert => {
                let left = Viewport::new(
                    pos,
                    Size::new(size.width / 2, size.height), // - Size::new(10, 10)
                );

                let right = Viewport::new(
                    Position::new(pos.x + size.width / 2 /*+ 10*/, pos.y),
                    Size::new(size.width / 2, size.height), // - Size::new(10, 10)
                );

                (left, right)
            }
        };

        let left = Container::new(left, dir, ctx, image).unwrap();
        let right = Container::new(right, dir, ctx, image).unwrap();

        self.inner.insert_left(self.selected, left);
        self.selected = self.inner.insert_right(self.selected, right);
        let mut selected = &mut self.inner[self.selected];
        selected.border.border_type = BorderType::Active;
    }

    pub fn remove_container(&mut self, node_id: NodeId) {
        // 1. Remove the node
        // 2. Collpase the parent so the remaning child
        //    becomes the parent

        let node = self.inner.remove(node_id);

        if let Some(parent_id) = self.inner[node.id].parent {
            let parent = &self.inner[parent_id];

            if let Some(left) = parent.left {
                // swap the left value with the parent
            }

            if let Some(right) = parent.right {
                // swap the right value with the parent
            }
        }
    }

    pub(crate) fn render(&self, ctx: &mut MessageCtx) -> Result<()> {
        for node in self
            .inner
            .iter()
            .filter(|node| node.left.is_none() && node.right.is_none())
        {
            node.render(ctx)?;
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
//     - Container -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Image;

pub(super) struct Container {
    dir: Direction,
    viewport: Viewport,
    renderer: Renderer<VertexData>,
    border: Border,
    image: Option<Image>,
    // image: &(Texture<i32>, Vec<Layer>),
}

impl Container {
    fn new(
        viewport: Viewport,
        dir: Direction,
        ctx: &mut MessageCtx,
        image: Option<Image>,
    ) -> Result<Self> {
        let border_type = BorderType::Inactive;

        let inst = Self {
            border: Border::new(border_type, ctx.textures, &viewport),
            viewport,
            renderer: Renderer::default(ctx.context)?,
            image,
            dir,
        };

        Ok(inst)
    }

    fn render(&self, ctx: &mut MessageCtx) -> Result<()> {
        self.border
            .render(ctx.textures, &self.viewport, &self.renderer, ctx.context);
        Ok(())
    }
}
