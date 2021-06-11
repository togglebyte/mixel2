//! Shows and renders all containers.
//! A `Container` holds the viewport
use std::path::Path;

use anyhow::Result;
use nightmaregl::events::{ButtonState, MouseButton};
use nightmaregl::pixels::Pixel;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Point, Size, Sprite, Transform, Viewport, Rect, Context};

use crate::Mouse;
use crate::border::BorderType;
use crate::canvas::LayerId;
use crate::config::Action;
use crate::layout::{Split, Layout};
use crate::listener::MessageCtx;
use crate::message::Message;

use super::{Container, Image, SaveBuffer};

// -----------------------------------------------------------------------------
//     - Containers -
// -----------------------------------------------------------------------------
pub struct Containers {
    /// All container viewports should be relative 
    /// to this one.
    pub(super) viewport: Viewport,
    /// Layout
    layout: Layout,
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
        let container = Container::new(
            viewport.clone(),
            Split::Horz,
            ctx,
            Sprite::from_size(Size::new(32, 32)),
            // Transform::new(Position::new(10, 10)),
        )?;

        let mut inst = Self {
            layout: Layout::Leaf { id: 0, size: *viewport.size(), pos: container.viewport.position },
            selected: 0,
            inner: vec![container],
            images: Vec::new(),
            viewport,
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

    pub fn resize(&mut self, mut new_size: Size<i32>) {
        // HACK! remove this dirt
        // Do this because of the padding
        new_size.width -= 128 * 2;
        new_size.height -= 128 * 2;
        // End of hack

        self.layout.set_size(new_size);
        self.layout.rebuild();
        self.layout.layout(&mut self.inner);
        self.inner.iter_mut().for_each(Container::resize);
    }

    // This is horrid: 
    // TODO: removing a split will ruin everything,
    // because of the indexing.
    //
    // But rather than using an arena we can just rebuild it
    pub fn split(&mut self, dir: Split, ctx: &mut MessageCtx) -> Result<()> {
        let new_id = self.inner.len();
        self.layout.split(self.selected, new_id, dir);

        let selected = self.selected();
        let viewport = selected.viewport.clone();
        let sprite = selected.node.sprite.clone();

        let mut container = Container::new(
            viewport,
            Split::Horz,
            ctx,
            sprite,
        )?;

        container.image_id = selected.image_id;
        self.inner.push(container);

        self.layout.layout(&mut self.inner);
        self.inner
            .iter_mut()
            .for_each(|cont| {
                cont.border.resize(&cont.viewport);
                let mut cur_pos = cont.node.transform.translation;
                match dir {
                    Split::Horz => cur_pos.y /= 2,
                    Split::Vert => cur_pos.x /= 2,
                };
                cont.node.transform.translate_mut(cur_pos);
            });

        // Set the active border.
        // Ignore the previous active border as that
        // is not rendered since it now has children.
        let mut selected = &mut self.inner[self.selected];
        selected.border.border_type = BorderType::Active;

        Ok(())
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
                Some(id) => &mut self.images[id],
                None => continue,
            };

            if image.dirty {
                image.redraw_layers();
            }

            container.render(background, ctx, image)?;
        }

        Ok(())
    }

    pub fn draw(&mut self, pos: Position<i32>) {
        let container = &mut self.inner[self.selected];
        let image = match container.image_id {
            Some(id) => &mut self.images[id],
            None => return,
        };

        image.put_pixel(container.colour, pos);
    }

    pub fn clear_pixel(&mut self, pos: Position<i32>) {
        let container = &mut self.inner[self.selected];

        let image = match container.image_id {
            Some(id) => &mut self.images[id],
            None => return,
        };

        image.clear_pixel(pos);
    }

    pub fn action(&mut self, action: Action) -> Message {
        use Action::*;
        match action {
            Left => {
                let pos = self.selected().move_cursor_by(Position::new(-1, 0));
                self.update_positions(pos);
                return Message::TranslatedCursor(pos);
            }
            Right => {
                let pos = self.selected().move_cursor_by(Position::new(1, 0));
                self.update_positions(pos);
                return Message::TranslatedCursor(pos);
            }
            Up => {
                let pos = self.selected().move_cursor_by(Position::new(0, 1));
                self.update_positions(pos);
                return Message::TranslatedCursor(pos);
            }
            Down => {
                let pos = self.selected().move_cursor_by(Position::new(0, -1));
                self.update_positions(pos);
                return Message::TranslatedCursor(pos);
            }
            CanvasZoomIn => self.selected().renderer.pixel_size += 1,
            CanvasZoomOut => self.selected().renderer.pixel_size -= 1,
            _ => {}
        }

        Message::Noop
    }

    fn update_positions(&mut self, pos: Position<i32>) {
        let image_id = self.selected().image_id;
        let selected = self.selected;
        self.inner
            .iter_mut()
            .enumerate()
            .filter(|(i, cont)| cont.image_id == image_id && *i != selected) 
            .for_each(|(_, cont)| cont.move_cursor(pos));
    }

    pub fn mouse_input(&mut self, mouse: Mouse, ctx: &MessageCtx) -> Position<i32> {
        let pos = self.selected().translate_mouse(mouse.pos, ctx);
        self.selected().move_cursor(pos);
        self.update_positions(pos);

        let size = self.selected().node.sprite.size;
        let pos = Position::new(pos.x, size.height - pos.y - 1);

        if let ButtonState::Pressed = mouse.state {
            let bounding_box = Rect::new(Point::zero(), size);
            if !bounding_box.contains(pos.to_point()) {
                return pos;
            }


            if let Some(MouseButton::Left) = mouse.button {
                self.draw(pos);
            }

            if let Some(MouseButton::Right) = mouse.button {
                self.clear_pixel(pos);
            }
        }

        pos
    }

    pub fn set_colour(&mut self, colour: Pixel) {
        let container = self.selected();
        container.set_colour(colour);
    }

    pub fn set_alpha(&mut self, alpha: usize) {
        let container = self.selected();
        container.set_alpha(alpha);
    }

    pub(super) fn selected(&mut self) -> &mut Container {
        &mut self.inner[self.selected]
    }

    pub(super) fn selected_image(&mut self) -> Option<&mut Image> {
        let id = self.inner[self.selected].image_id?;
        self.images.get_mut(id)
    }

    pub(super) fn new_layer(&mut self) -> Option<(LayerId, usize)> {
        let size = self.selected().node.sprite.size;
        self.selected_image().map(|image| image.new_layer(size))
    }

    pub(super) fn set_layer(&mut self, mut layer_id: LayerId) -> Option<(LayerId, usize)> {
        if layer_id.as_display() == 0 {
            return None;
        }

        self.selected_image().map(|image| image.set_layer(layer_id))
    }

    pub(super) fn remove_layer(&mut self) -> Option<(LayerId, usize)> {
        self.selected_image().and_then(Image::remove_layer)
    }

    pub(super) fn save_current(&mut self, path: impl AsRef<Path>, overwrite: bool, context: &mut Context) {
        if !overwrite && path.as_ref().exists() {
            return
        }
        let size = self.selected().node.sprite.size.clone();
        let image = self.selected_image().unwrap();
        let mut save_buf = SaveBuffer::new(context, size).unwrap();
        save_buf.save(path, image, size, context);
    }

    pub(super) fn change_pixel_size(&mut self, offset: i32) {
        eprintln!("{:?}", offset);
        self.selected().renderer.pixel_size += offset;
    }
}
