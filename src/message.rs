use nightmaregl::{Position, Size};
use crate::application::Mode;
use crate::commandline::Command;
use crate::input::Input;
use crate::config::Action;
use crate::canvas::message::Canvas;
use nightmaregl::events::Modifiers;

pub enum Message {
    Input(Input, Modifiers),
    CursorPos(Position<i32>),
    Resize(Size<i32>),
    ModeChanged(Mode),
    Command(Command),
    Action(Action),
    Canvas(Canvas),
    Noop,
}

