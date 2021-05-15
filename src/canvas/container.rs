use std::rc::Rc;
use std::fmt;

use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Context, Position, Renderer, Size, Sprite, VertexData, Viewport};
use nightmaregl::pixels::Pixel;

use super::layer::Layer;
use crate::binarytree::{Node, NodeId, Tree};
use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use super::{Image, Images};

// -----------------------------------------------------------------------------
//     - Direction -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Horz,
    Vert,
}

// -----------------------------------------------------------------------------
//     - Images -
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
//     - Containers -
// -----------------------------------------------------------------------------
pub struct Containers {
    /// All containers
    inner: Tree<Container>,
    /// Selected container id
    selected: NodeId,
    /// A collection of images and the nodes
    /// that are currently drawing them.
    images: Images,
}

impl Containers {
    // TODO: remove this
    pub(super) fn print_tree(&self) {
        eprintln!("{:#?}", self.inner);
        eprintln!("selected: {:?}", self.selected);
        eprintln!("{:?}", "--------------");
    }

    /// Create a new instance of a container.
    /// The container holds the drawable area of the screen.
    /// A container can be split into multiple containers.
    pub fn new(viewport: Viewport, ctx: &mut MessageCtx) -> Result<Self> {
        let viewport = Viewport::new(
            viewport.position + Position::new(10, 10),
            *viewport.size() - Size::new(20, 20),
        );

        let container = Container::new(
            viewport,
            Direction::Horz,
            ctx,
            Sprite::from_size(Size::new(32, 32)),
        )?;
        let tree = Tree::new(container);

        let inst = Self {
            selected: tree.root_id(),
            inner: tree,
            images: Images::new(),
        };

        Ok(inst)
    }

    /// Add a new image to the current container
    pub fn add_image(&mut self, size: Size<i32>, image: Image) {
        self.images.push(image, self.selected);
        let mut selected = &mut self.inner[self.selected];
        selected.sprite = Sprite::from_size(size);
        let pos = (*selected.viewport.size() / 2 - selected.sprite.size) / selected.renderer.pixel_size;
        selected.sprite.position = pos.to_vector();
    }

    // TODO: when you set the anchor point, don't set it to the centre
    //       of the sprite. I know this seems like a good idea, but it will
    //       look naff when you move the texture around
    pub fn resize(&mut self, new_size: Size<i32>) {
        panic!("Resize ALL sprites that reference the same image");
        let mut selected = &mut self.inner[self.selected];
        // match selected.image {
        //     Some((ref mut image, ref mut sprite)) => {
        //         // image.layers.iter_mut().for_each(|layer| layer.resize(new_size));
        //         // sprite.size = new_size;
        //     },
        //     None => {}
        // }
    }

    pub fn split(&mut self, dir: Direction, ctx: &mut MessageCtx) {
        // Get the current size, position and sprite as
        // well as the image id for the selected container.
        // Use these to construct the children.
        let (size, pos, sprite, image_id) = {
            let selected = &self.inner[self.selected];
            let image_id = self.images.get_id_by_node(self.selected);
            (
                selected.viewport.size(),
                selected.viewport.position,
                selected.sprite,
                image_id,
            )
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

        // Create child containers
        let left = Container::new(left, dir, ctx, sprite).unwrap();
        let right = Container::new(right, dir, ctx, sprite).unwrap();

        let left_id = self.inner.insert_left(self.selected, left);
        let right_id = self.inner.insert_right(self.selected, right);
        self.selected = right_id;

        // Assign the image to the newly created container
        if let Some(image_id) = image_id {
            self.images.attach(image_id, left_id);
            self.images.attach(image_id, right_id);
        }

        // Set the active border.
        // Ignore the previous active border as that
        // is not rendered since it now has children.
        let mut selected = &mut self.inner[self.selected];
        selected.border.border_type = BorderType::Active;
    }

    // TODO: removing the last container should close the application.
    pub fn close_selected(&mut self) {
        let selected = self.inner.sibling(self.selected);
        self.inner.remove(self.selected);
        if let Some(selected) = selected {
            self.inner.collapse_into_parent(selected);
            self.selected = selected;
        }
    }

    pub(crate) fn render(
        &self,
        background: &Texture<i32>,
        ctx: &mut MessageCtx,
    ) -> Result<()> {

        self.images
            .images()
            .into_iter()
            .map(|(image, nodes)| {
                let nodes = nodes.into_iter()
                    .filter_map(|id| self.inner.get(*id))
                    .filter(|node| node.left.is_none() && node.right.is_none());

                (image, nodes)
            })
            .for_each(|(image, nodes)| {
                nodes.into_iter().for_each(|node| {
                    node.render(background, ctx, image);
                });
            });

        Ok(())
    }

    pub fn draw(&mut self, pos: Position<i32>) {
        let pixel = Pixel::black();
        if let Some(image) = self.images.get_by_node_mut(self.selected) {
            image.put_pixel(pixel, pos);
        }
        // let mut selected = &mut self.inner[self.selected];
    }
}

// -----------------------------------------------------------------------------
//     - Container -
// -----------------------------------------------------------------------------
// TODO: delete the debug impl
impl fmt::Debug for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub(super) struct Container {
    dir: Direction,
    viewport: Viewport,
    renderer: Renderer<VertexData>,
    border: Border,
    sprite: Sprite<i32>,
}

impl Container {
    fn new(
        viewport: Viewport,
        dir: Direction,
        ctx: &mut MessageCtx,
        mut sprite: Sprite<i32>,
    ) -> Result<Self> {
        let border_type = BorderType::Inactive;

        let mut inst = Self {
            border: Border::new(border_type, ctx.textures, &viewport),
            viewport,
            renderer: Renderer::default(ctx.context)?,
            sprite,
            dir,
        };

        inst.renderer.pixel_size = 8;

        // Centre the sprite
        // TODO: it doesn't quite look like it is in the centre
        //       is it the border? is it the viewport?
        let position = (*inst.viewport.size() / 2 / inst.renderer.pixel_size) - inst.sprite.size / 2;
        inst.sprite.position = position.to_vector();

        Ok(inst)
    }

    fn render(
        &self,
        background_texture: &Texture<i32>,
        ctx: &mut MessageCtx,
        image: &Image,
    ) -> Result<()> {
        self.border.render(
            ctx.textures,
            &self.viewport,
            ctx.border_renderer,
            ctx.context,
        );

        let vertex_data = self.sprite.vertex_data();

        // Render all layers
        image.layers.iter().for_each(|layer| {
            self.renderer
                .render(&layer.texture, &[vertex_data], &self.viewport, ctx.context);
        });

        // Render the "transparent" background texture
        self.renderer.render(
            background_texture,
            &[vertex_data],
            &self.viewport,
            ctx.context,
        );

        Ok(())
    }
}
