use anyhow::Result;
use nightmaregl::{
    Context, FillMode, Position, Rect, Renderer, Size, Sprite, VertexData, Viewport, RelativeViewport
};
use nightmaregl::texture::Texture;

use crate::listener::{MessageCtx, Listener};
use crate::Message;
use crate::border::{Textures, Border, BorderType};
use crate::binarytree::{Tree, Node};
use crate::commandline::Command;

pub mod message;
mod container;
mod layer;

use container::Containers;
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
            Message::Canvas(_) => panic!("oh noes, forgot to do this"),
            Message::Resize(new_size) => {
                self.viewport.resize(*new_size);
                self.border.resize(&self.viewport);
            }
            Message::Command(Command::Split(dir)) => {
                self.containers.split(*dir, ctx);
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

        // for canvas in self.containers.iter_mut().filter(|c| c.visible) {
        // }

        self.containers.render(ctx);
        

        // let pixel_size = self.renderer.pixel_size as f32;

        // // Canvas / Drawable area
        // let mut vertex_data = self.sprite.vertex_data_scaled(pixel_size);

        // let res = self
        //     .renderer
        //     .render(&self.background, &[vertex_data], viewport, context)?;

        // // Decrease the z_index,
        // vertex_data
        //     .model
        //     .append_translation_mut(&Vector3::from([0.0, 0.0, -1.0]));

        Ok(())
    }
}
