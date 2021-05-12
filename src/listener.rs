use anyhow::Result;
use nightmaregl::{Position, Size, Context, Viewport, Renderer, VertexData,};

use crate::input::Input;
use crate::application::Mode;
use crate::commandline::Command;
use crate::config::{Config, Action};
use crate::message::Message;
use crate::border::Textures;

pub trait Listener {
    fn message(&mut self, _: &Message, _: &mut MessageCtx) -> Message {
        Message::Noop
    }

    fn render(&mut self, _: &mut MessageCtx) -> Result<()> {
        Ok(())
    }
}

pub struct MessageCtx<'a> {
    pub config: &'a Config,
    pub context: &'a mut Context,
    pub viewport: &'a Viewport,
    pub textures: &'a Textures,
    pub border_renderer: &'a Renderer<VertexData>,
}

