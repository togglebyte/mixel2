use nightmaregl::{Position, Size, Context};
use nightmaregl::events::Modifiers;
use crate::input::Input;
use crate::application::Mode;
use crate::commandline::Command;
use crate::config::{Config, Action};

pub trait Listener {
    fn message(&mut self, m: &Message, config: &Config) -> Message;
    fn render(&mut self, context: &mut Context) {
    }
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
