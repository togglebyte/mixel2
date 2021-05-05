use anyhow::Result;
use nightmaregl::{Position, Size, Context as GlContext};
use nightmaregl::events::Modifiers;

use crate::input::Input;
use crate::application::Mode;
use crate::commandline::Command;
use crate::config::{Config, Action};

pub trait Listener {
    fn message(&mut self, _: &Message, _: &MessageCtx) -> Message {
        Message::Noop
    }

    fn render(&mut self, _: &mut GlContext) -> Result<()> {
        Ok(())
    }
}

pub struct MessageCtx<'a> {
    pub config: &'a Config,
    pub context: &'a mut GlContext,
}

pub enum Message {
    Input(Input, Modifiers),
    CursorPos(Position<i32>),
    Resize(Size<i32>),
    ModeChanged(Mode),
    Command(Command),
    Action(Action),
    // Render(&'a mut Context),
    Noop,
}
