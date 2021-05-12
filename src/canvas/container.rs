use std::rc::Rc;

use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Context, Position, Renderer, Size, Sprite, VertexData, Viewport};

use super::layer::Layer;
use crate::binarytree::{Node, NodeId, Tree};
use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use super::Image;

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

impl Containers {
    pub fn new(viewport: Viewport, ctx: &mut MessageCtx) -> Result<Self> {
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

    pub fn add_image(&mut self, image: Image) {
        self.inner[self.selected].image = Some(Rc::new(image));
    }

    pub fn split(&mut self, dir: Direction, ctx: &mut MessageCtx) {
        let (size, pos, image) = {
            let selected = &self.inner[self.selected];
            let image = match selected.image {
                Some(ref i) => Some(Rc::clone(i)),
                None => None,
            };
            (selected.viewport.size(), selected.viewport.position, image)
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

        let left = Container::new(left, dir, ctx, image.clone()).unwrap();
        let right = Container::new(right, dir, ctx, image).unwrap();

        self.inner.insert_left(self.selected, left);
        self.selected = self.inner.insert_right(self.selected, right);
        let mut selected = &mut self.inner[self.selected];
        selected.border.border_type = BorderType::Active;
    }

    // TODO: removing the last container should close the application.
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

    pub(crate) fn render(&self, background_texture: &Texture<i32>, ctx: &mut MessageCtx) -> Result<()> {
        for container in self
            .inner
            .iter()
            .filter(|node| node.left.is_none() && node.right.is_none())
        {
            container.render(background_texture, ctx)?;
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
    border: Border,
    sprite: Sprite<i32>, // TODO: add the sprite. 
                         // It needs to live on the container so we can move 
                         // the sprite around inside the container
    image: Option<Rc<Image>>,
}

impl Container {
    fn new(
        viewport: Viewport,
        dir: Direction,
        ctx: &mut MessageCtx,
        image: Option<Rc<Image>>,
    ) -> Result<Self> {
        let border_type = BorderType::Inactive;

        let mut inst = Self {
            border: Border::new(border_type, ctx.textures, &viewport),
            viewport,
            renderer: Renderer::default(ctx.context)?,
            image,
            dir,
        };

        inst.renderer.pixel_size = 8;

        Ok(inst)
    }

    fn render(&self, background_texture: &Texture<i32> , ctx: &mut MessageCtx) -> Result<()> {
        self.border
            .render(ctx.textures, &self.viewport, ctx.border_renderer, ctx.context);

        match self.image {
            Some(ref image) => {
                let vertex_data = image.vertex_data();

                self.renderer.render(
                    background_texture,
                    &vertex_data,
                    &self.viewport,
                    ctx.context
                );

                image.layers.iter().for_each(|layer| {
                    self.renderer.render(
                        &layer.texture,
                        &vertex_data,
                        &self.viewport,
                        ctx.context
                    );
                });
            }
            None => {}
        }

        Ok(())
    }
}
