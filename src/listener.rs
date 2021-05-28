use anyhow::Result;
use nightmaregl::{Context, Renderer, VertexData, Viewport};

use crate::border::Textures;
use crate::config::Config;
use crate::message::Message;

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
