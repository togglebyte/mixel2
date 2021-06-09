//! This works as a thin layer for the `Containers`,
//! as containers does not implement `Listener`.
use anyhow::Result;
use nightmaregl::{ Renderer, VertexData, Viewport, Transform };
use nightmaregl::texture::Texture;

use crate::listener::{MessageCtx, Listener};
use crate::Message;
use crate::border::{Border, BorderType};
use crate::commandline::Command;

pub mod message;
mod containers;
mod layer;
mod image;
mod cursor;
mod container;
mod savebuffer;

pub use container::Container;
pub use containers::Containers;
pub use cursor::Cursor;
pub use image::Image;
pub use layer::LayerId;
pub use savebuffer::SaveBuffer;

pub struct Canvas {
    /// All container viewports should be relative 
    /// to this one.
    viewport: Viewport,
    /// All <whatevers> 
    containers: Containers,
    /// Background for transparency
    background: Texture<i32>,
    /// Border around all containers
    border: Border,
    /// Render the border
    renderer: Renderer<VertexData>,
}

impl Canvas {
    pub fn new(viewport: Viewport, ctx: &mut MessageCtx) -> Result<Self> {
        let inst = Self {
            background: Texture::from_disk("background.png")?,
            viewport,
            border: Border::new(BorderType::Canvas, ctx.textures, &viewport),
            renderer: Renderer::default(ctx.context)?,
            containers: Containers::new(viewport, ctx)?,
        };

        Ok(inst)
    }
}

impl Listener for Canvas {
    fn message(&mut self, message: &Message, ctx: &mut MessageCtx) -> Message {
        match message {
            Message::Resize(new_size) => {
                self.viewport.resize(*new_size);
                self.border.resize(&self.viewport);
            }
            Message::Command(Command::Split(dir)) => {
                self.containers.split(*dir, ctx);
            }
            Message::Command(Command::NewImage(size)) => {
                let image = Image::new(*size);
                self.containers.add_image(*size, image);
            }
            Message::Command(Command::CloseSelectedSplit) => {
                self.containers.close_selected();
            }
            Message::Command(Command::Split(split)) => {
                self.containers.split(*split, ctx);
            }
            Message::Command(Command::Put(pos)) => {
                self.containers.draw(*pos);
            }
            Message::Command(Command::Clear(pos)) => {
                self.containers.clear_pixel(*pos);
            }
            Message::Command(Command::SetColour(colour)) => {
                self.containers.set_colour(*colour);
            }
            Message::Command(Command::SetAlpha(alpha)) => {
                self.containers.set_alpha(*alpha);
            }
            Message::Command(Command::NewLayer) => {
                match self.containers.new_layer() {
                    Some((layer, total_layers)) => return Message::LayerChanged { layer, total_layers },
                    None => {}
                }
            }
            Message::Command(Command::RemoveLayer) => {
                match self.containers.remove_layer() {
                    Some((layer, total_layers)) => return Message::LayerChanged { layer, total_layers },
                    None => {}
                }
            }
            Message::Command(Command::ChangeLayer(layer)) => {
                match self.containers.set_layer(*layer) {
                    Some((layer, total_layers)) => return Message::LayerChanged { layer, total_layers },
                    None => {}
                }
                
            }
            Message::Command(Command::Save { path, overwrite }) => {
                self.containers.save_current(path, *overwrite, ctx.context);
            }
            Message::Action(action) => {
                self.containers.action(*action);
            }
            Message::Mouse(mouse) => {
                let pos = self.containers.mouse_input(*mouse, ctx);
                return Message::TranslatedMouse(pos);
            }

            // Unhandled messages
            Message::Input(_, _)
            | Message::CursorPos(_)
            | Message::ModeChanged(_)
            | Message::Command(_)
            | Message::TranslatedMouse(_)
            | Message::LayerChanged { .. }
            | Message::Noop => {}
        }

        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) -> Result<()> {
        // let parent_transform = Transform::new();
        // self.border.render(
        //     &parent_transform,
        //     ctx.textures,
        //     &self.viewport,
        //     &self.renderer,
        //     ctx.context,
        // );

        self.containers.render(&self.background, ctx);

        Ok(())
    }
}
