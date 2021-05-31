use anyhow::Result;
use nightmaregl::events::{ButtonState, MouseButton};
use nightmaregl::pixels::Pixel;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Size, Sprite, Transform, Viewport};

use crate::config::Action;
use crate::listener::MessageCtx;
use crate::Mouse;

use super::{Container, Image};

// -----------------------------------------------------------------------------
//     - Orientation -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum Orientation {
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
    images: Vec<Image>,
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
            Orientation::Horz,
            ctx,
            Sprite::from_size(Size::new(32, 32)),
            Transform::default(),
        )?;

        let mut inst = Self {
            selected: 0,
            inner: vec![container],
            images: Vec::new(),
        };

        let size = Size::new(32, 32);
        let image = Image::new(size);
        inst.add_image(size, image);

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
        // let pos = (*selected.viewport.size() / 2 / selected.renderer.pixel_size).to_vector();
        // selected.node.sprite = sprite;
        // selected.node.transform.translate_mut(pos);
    }

    // // TODO: when you set the anchor point, don't set it to the centre
    // //       of the sprite. I know this seems like a good idea, but it will
    // //       look naff when you move the texture around
    // pub fn resize(&mut self, new_size: Size<i32>) {
    //     panic!("Resize ALL sprites that reference the same image");
    //     let mut selected = &mut self.inner[self.selected];
    //     // match selected.image {
    //     //     Some((ref mut image, ref mut sprite)) => {
    //     //         // image.layers.iter_mut().for_each(|layer| layer.resize(new_size));
    //     //         // sprite.size = new_size;
    //     //     },
    //     //     None => {}
    //     // }
    // }

    pub fn split(&mut self, _dir: Orientation, _ctx: &mut MessageCtx) {
        //     // TODO: put this back in once the bin tree is done.
        //     // // Get the current size, position and sprite as
        //     // // well as the image id for the selected container.
        //     // // Use these to construct the children.
        //     // let (size, pos, sprite, image_id) = {
        //     //     let selected = &self.inner[self.selected];
        //     //     let image_id = self.images.get_id_by_node(self.selected);
        //     //     (
        //     //         selected.viewport.size(),
        //     //         selected.viewport.position,
        //     //         selected.sprite,
        //     //         image_id,
        //     //     )
        //     // };

        //     // let (left, right) = match dir {
        //     //     Orientation::Horz => {
        //     //         let right = Viewport::new(
        //     //             pos,
        //     //             Size::new(size.width, size.height / 2), // - Size::new(10, 10)
        //     //         );

        //     //         let left = Viewport::new(
        //     //             Position::new(pos.x, pos.y + size.height / 2 /* + 10*/),
        //     //             Size::new(size.width, size.height / 2), // - Size::new(10, 10)
        //     //         );

        //     //         (left, right)
        //     //     }
        //     //     Orientation::Vert => {
        //     //         let left = Viewport::new(
        //     //             pos,
        //     //             Size::new(size.width / 2, size.height), // - Size::new(10, 10)
        //     //         );

        //     //         let right = Viewport::new(
        //     //             Position::new(pos.x + size.width / 2 /*+ 10*/, pos.y),
        //     //             Size::new(size.width / 2, size.height), // - Size::new(10, 10)
        //     //         );

        //     //         (left, right)
        //     //     }
        //     // };

        //     // // Create child containers
        //     // let left = Container::new(left, dir, ctx, sprite).unwrap();
        //     // let right = Container::new(right, dir, ctx, sprite).unwrap();

        //     // let left_id = self.inner.insert_left(self.selected, left);
        //     // let right_id = self.inner.insert_right(self.selected, right);
        //     // self.selected = right_id;

        //     // // Assign the image to the newly created container
        //     // if let Some(image_id) = image_id {
        //     //     self.images.attach(image_id, left_id);
        //     //     self.images.attach(image_id, right_id);
        //     // }

        //     // // Set the active border.
        //     // // Ignore the previous active border as that
        //     // // is not rendered since it now has children.
        //     // let mut selected = &mut self.inner[self.selected];
        //     // selected.border.border_type = BorderType::Active;
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

    pub fn render(&mut self, background: &Texture<i32>, ctx: &mut MessageCtx) -> Result<()> {
        for (id, container) in self.inner.iter_mut().enumerate() {
            let image = match container.image_id {
                Some(id) => &self.images[id],
                None => continue,
            };

            let render_cursor = self.selected == id;
            container.render(background, ctx, image, render_cursor)?;
        }

        Ok(())
    }

    pub fn draw(&mut self, pos: Position<i32>) {
        let pixel = Pixel::black();
        let container = &mut self.inner[self.selected];
        let image = match container.image_id {
            Some(id) => &mut self.images[id],
            None => return,
        };

        image.put_pixel(pixel, pos);
    }

    pub fn clear_pixel(&mut self, pos: Position<i32>) {
        let container = &mut self.inner[self.selected];

        let image = match container.image_id {
            Some(id) => &mut self.images[id],
            None => return,
        };

        image.clear_pixel(pos);
    }

    pub fn action(&mut self, action: Action) {
        use Action::*;
        match action {
            Left => self.selected().move_cursor(Position::new(-1, 0)),
            Right => self.selected().move_cursor(Position::new(1, 0)),
            Up => self.selected().move_cursor(Position::new(0, 1)),
            Down => self.selected().move_cursor(Position::new(0, -1)),
            CanvasZoomIn => self.selected().renderer.pixel_size += 1,
            CanvasZoomOut => self.selected().renderer.pixel_size -= 1,
            _ => {}
        }
    }

    pub fn mouse_input(&mut self, mouse: Mouse, ctx: &MessageCtx) -> Position<i32> {
        let pos = self.selected().translate_mouse(mouse.pos, ctx);

        if let ButtonState::Pressed = mouse.state {
            // TODO: Once the mouse coords are sorted,
            // don't 
            let image = self.selected();
            let height = image.node.sprite.size.height;
            let mut new_pos = pos;
            new_pos.y = height - new_pos.y;

            // Clamp the pos
            new_pos.x = new_pos.x.clamp(0, height - 1);
            new_pos.y = new_pos.y.clamp(0, height - 1);


            if let Some(MouseButton::Left) = mouse.button {
                eprintln!("{:?}", new_pos);
                self.draw(new_pos);
            }
            if let Some(MouseButton::Right) = mouse.button {
                self.clear_pixel(new_pos);
            }
        }

        pos
    }

    fn selected(&mut self) -> &mut Container {
        &mut self.inner[self.selected]
    }
}