use anyhow::Result;
use nightmare::{Context, VertexData, Viewport};
use nightmare::render2d::{SimpleRenderer, Model};

// use crate::border::Textures;
use crate::config::Config;
use crate::message::Message;

pub trait Listener {
    fn message(&mut self, _: &Message, _: &mut MessageCtx) -> Message {
        Message::Noop
    }

    fn render(&mut self, _: &mut MessageCtx) {
    }
}

pub struct MessageCtx<'a> {
    pub config: &'a Config,
    pub context: &'a mut Context,
    pub canvas_viewport: &'a Viewport,
    pub app_viewport: &'a Viewport,
    // pub textures: &'a Textures, // TODO: if these are the border textures, why aren't they called border_textures
    // pub border_renderer: &'a SimpleRenderer<Model>,
}
