use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Context, Position, Renderer, Size, Sprite, VertexData, Viewport};

use super::layer::Layer;
use crate::binarytree::{NodeId, Tree, Node};
use crate::listener::MessageCtx;
use crate::border::{BorderType, Border};

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
            *viewport.size() - Size::new(20, 20)
        );
        let mut container = Container::new(viewport, Direction::Horz, ctx)?;
        
        let tree = Tree::new(container);
        let inst = Self {
            selected: tree.root_id(),
            inner: tree,
        };

        Ok(inst)
    }

    pub(crate) fn split(&mut self, dir: Direction, ctx: &mut MessageCtx) {
        let (size, pos) = {
            let selected = &self.inner[self.selected];
            (
                selected.viewport.size(),
                selected.viewport.position
            )
        };

        let (left, right) = match dir {
            Direction::Horz => {
                let left = Viewport::new(
                    pos,
                    Size::new(size.width, size.height / 2) - Size::new(10, 10)
                );

                let right = Viewport::new(
                    Position::new(pos.x, pos.y + size.height + 10),
                    Size::new(size.width, size.height / 2) - Size::new(10, 10)
                );

                (left, right)
            }
            Direction::Vert => { todo!("omg help"); }
        };

        let left = Container::new(left, dir, ctx).unwrap();
        let right = Container::new(right, dir, ctx).unwrap();

        self.inner.insert_left(self.selected, left);
        self.inner.insert_right(self.selected, right);
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

    pub(crate) fn render(
        &self,
        ctx: &mut MessageCtx,
    ) -> Result<()> {
        for node in self.inner.iter().filter(|node| node.left.is_none() && node.right.is_none()) {
            node.render(ctx)?;
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
//     - Container -
// -----------------------------------------------------------------------------
pub(super) struct Container {
    dir: Direction,
    viewport: Viewport,
    renderer: Renderer<VertexData>,
    // image: &(Texture<i32>, Vec<Layer>),
    border: Border,
}

impl Container {
    fn new(viewport: Viewport, dir: Direction, ctx: &mut MessageCtx) -> Result<Self> {
        let border_type = BorderType::Inactive;

        let inst = Self {
            border: Border::new(border_type, ctx.textures, &viewport),
            viewport,
            renderer: Renderer::default(ctx.context)?,
            dir,
        };

        Ok(inst)
    }

    fn render(&self, ctx: &mut MessageCtx) -> Result<()> {
        self.border.render(
            ctx.textures,
            &self.viewport,
            &self.renderer,
            ctx.context
        );
        Ok(())
    }
}
