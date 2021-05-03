use nightmaregl::{Position, Size, Context};
use crate::input::Input;
use crate::application::{Render, Mode};
use crate::commandline::Command;

pub trait Listener : Render {
    fn message(&mut self, m: &Message) -> Option<Message> {
        None
    }
}

#[derive(Debug)]
pub enum Message {
    Input(Input),
    CursorPos(Position<i32>),
    Resize(Size<i32>),
    ModeChanged(Mode),
    Command(Command),
}
