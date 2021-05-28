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

pub use containers::{Orientation, Containers};
pub use image::Image;
pub use cursor::Cursor;
pub use container::Container;

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
            Message::Command(Command::Put(pos)) => {
                self.containers.draw(*pos);
            }
            Message::Action(action) => {
                self.containers.action(*action);
            }

            // Unhandled messages
            Message::Input(_, _)
            | Message::CursorPos(_)
            | Message::ModeChanged(_)
            | Message::Command(_)
            | Message::Noop => {}
        }
        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) -> Result<()> {
        let parent_transform = Transform::new();
        self.border.render(
            &parent_transform,
            ctx.textures,
            &self.viewport,
            &self.renderer,
            ctx.context,
        );

        self.containers.render(&self.background, ctx);

        Ok(())
    }
}
