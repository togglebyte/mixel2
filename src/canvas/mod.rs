//! This works as a thin layer for the `Containers`,
//! as containers does not implement `Listener`.
use anyhow::Result;
use nightmaregl::{
    Context, FillMode, Position, Rect, Renderer, Size, Sprite, VertexData, Viewport, RelativeViewport
};
use nightmaregl::texture::Texture;

use crate::listener::{MessageCtx, Listener};
use crate::Message;
use crate::border::{Textures, Border, BorderType};
use crate::commandline::Command;
use crate::input::Input;

pub mod message;
mod container;
mod layer;
mod image;
mod cursor;

use container::Containers;
pub use image::Image;
pub use cursor::Cursor;

pub use container::Direction;

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
            },
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
            Message::Command(Command::Put(pos)) => {
                self.containers.draw(*pos);
            }

            // Unhandled messages
            Message::Input(_, _)
            | Message::CursorPos(_)
            | Message::ModeChanged(_)
            | Message::Command(_)
            | Message::Action(_)
            | Message::Noop => {}
        }
        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) -> Result<()> {
        self.border.render(
            ctx.textures,
            &self.viewport,
            &self.renderer,
            ctx.context,
        );

        self.containers.render(&self.background, ctx);

        Ok(())
    }
}
