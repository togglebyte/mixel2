use std::rc::Rc;
use std::fmt;

use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Context, Position, Renderer, Size, Sprite, VertexData, Viewport, Transform};
use nightmaregl::pixels::Pixel;

use super::layer::Layer;
use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use crate::Node;
use super::{Cursor, Image};

// -----------------------------------------------------------------------------
//     - Direction -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Horz,
    Vert,
}

// -----------------------------------------------------------------------------
//     - Containers -
// -----------------------------------------------------------------------------
pub struct Containers {
    /// All containers
    inner: Vec<Container>,
    /// Selected container id
    selected: usize,
    /// List of images to draw and render.
    /// An image can be rendererd in multiple containers
    /// which is why the container has an `image_id` rather than owning 
    /// an image.
    images: Vec<Image>
}

impl Containers {
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
            Transform::default(),
        )?;

        let inst = Self {
            selected: 0,
            inner: vec![container],
            images: Vec::new(),
        };

        Ok(inst)
    }

    /// Add a new image to the current container
    /// TODO: can't delete an image now because of how stupid this is.
    ///       Deleting an image would offset every image after it in 
    ///       the vector.
    pub fn add_image(&mut self, size: Size<i32>, image: Image) {
        let image_id = self.images.len();
        self.images.push(image);

        self.inner[self.selected].image_id = Some(image_id);
        let mut selected = &mut self.inner[self.selected];
        let sprite = Sprite::from_size(size);
        let pos = (*selected.viewport.size() / 2 / selected.renderer.pixel_size).to_vector();
        selected.node.sprite = sprite;
        selected.node.transform.translate_mut(pos);
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
        // TODO: put this back in once the bin tree is done.
        // // Get the current size, position and sprite as
        // // well as the image id for the selected container.
        // // Use these to construct the children.
        // let (size, pos, sprite, image_id) = {
        //     let selected = &self.inner[self.selected];
        //     let image_id = self.images.get_id_by_node(self.selected);
        //     (
        //         selected.viewport.size(),
        //         selected.viewport.position,
        //         selected.sprite,
        //         image_id,
        //     )
        // };

        // let (left, right) = match dir {
        //     Direction::Horz => {
        //         let right = Viewport::new(
        //             pos,
        //             Size::new(size.width, size.height / 2), // - Size::new(10, 10)
        //         );

        //         let left = Viewport::new(
        //             Position::new(pos.x, pos.y + size.height / 2 /* + 10*/),
        //             Size::new(size.width, size.height / 2), // - Size::new(10, 10)
        //         );

        //         (left, right)
        //     }
        //     Direction::Vert => {
        //         let left = Viewport::new(
        //             pos,
        //             Size::new(size.width / 2, size.height), // - Size::new(10, 10)
        //         );

        //         let right = Viewport::new(
        //             Position::new(pos.x + size.width / 2 /*+ 10*/, pos.y),
        //             Size::new(size.width / 2, size.height), // - Size::new(10, 10)
        //         );

        //         (left, right)
        //     }
        // };

        // // Create child containers
        // let left = Container::new(left, dir, ctx, sprite).unwrap();
        // let right = Container::new(right, dir, ctx, sprite).unwrap();

        // let left_id = self.inner.insert_left(self.selected, left);
        // let right_id = self.inner.insert_right(self.selected, right);
        // self.selected = right_id;

        // // Assign the image to the newly created container
        // if let Some(image_id) = image_id {
        //     self.images.attach(image_id, left_id);
        //     self.images.attach(image_id, right_id);
        // }

        // // Set the active border.
        // // Ignore the previous active border as that
        // // is not rendered since it now has children.
        // let mut selected = &mut self.inner[self.selected];
        // selected.border.border_type = BorderType::Active;
    }

    // TODO: removing the last container should close the application.
    pub fn close_selected(&mut self) {
        // TODO: put this back in once we can merge nodes in the bin tree.
        // let selected = self.inner.sibling(self.selected);
        // self.inner.remove(self.selected);
        // if let Some(selected) = selected {
        //     self.inner.collapse_into_parent(selected);
        //     self.selected = selected;
        // }
    }

    pub(crate) fn render(
        &mut self,
        background: &Texture<i32>,
        ctx: &mut MessageCtx,
    ) -> Result<()> {

        for container in &mut self.inner {
            let image = match container.image_id {
                Some(id) => &self.images[id],
                None => continue,
            };

            container.render(background, ctx, image);
        }

        Ok(())
    }

    pub fn draw(&mut self, pos: Position<i32>) {
        let pixel = Pixel::black();
        let container = &mut self.inner[self.selected];
        let mut image = match container.image_id {
            Some(id) => &mut self.images[id],
            None => return,
        };

        image.put_pixel(pixel, pos);
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
    // sprite: Sprite<i32>,
    // transform: Transform<i32>,
    node: Node<i32>,
    image_id: Option<usize>,
    cursor: Cursor,
}

impl Container {
    fn new(
        viewport: Viewport,
        dir: Direction,
        ctx: &mut MessageCtx,
        mut sprite: Sprite<i32>,
        transform: Transform<i32>,
    ) -> Result<Self> {
        let border_type = BorderType::Inactive;

        let mut inst = Self {
            border: Border::new(border_type, ctx.textures, &viewport),
            viewport,
            renderer: Renderer::default(ctx.context)?,
            node: Node::from_sprite(sprite),
            // sprite,
            // transform,
            dir,
            image_id: None,
            cursor: Cursor::new(Position::zero()),
        };

        inst.renderer.pixel_size = 8;

        // Centre the sprite
        // TODO: it doesn't quite look like it is in the centre
        //       is it the border? is it the viewport?
        let position = (*inst.viewport.size() / 2 / inst.renderer.pixel_size).to_vector();
        inst.node.transform.translate_mut(position);

        Ok(inst)
    }

    pub fn move_cursor(&mut self, pos: Position<i32>) {
        todo!();
        // self.cursor.transform.translate_mut(pos);
    }

    fn render(
        &self,
        background_texture: &Texture<i32>,
        ctx: &mut MessageCtx,
        image: &Image,
    ) -> Result<()> {
        // Border
        self.border.render(
            &self.node.transform,
            ctx.textures,
            &self.viewport,
            ctx.border_renderer,
            ctx.context,
        );

        // Cursor
        self.renderer.render(
            &self.cursor.texture,
            &[self.cursor.node.relative_vertex_data(&self.node.transform)],
            &self.viewport,
            ctx.context,
        );

        // Images
        let vertex_data = self.node.vertex_data();

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
