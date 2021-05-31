use nightmaregl::{Position, Size};
use crate::application::Mode;
use crate::commandline::Command;
use crate::input::Input;
use crate::config::Action;
use nightmaregl::events::Modifiers;

pub enum Message {
    Input(Input, Modifiers),
    CursorPos(Position<i32>),
    Resize(Size<i32>),
    ModeChanged(Mode),
    Command(Command),
    Action(Action),
    MouseMove(Position<i32>),
    TranslatedMouse(Position<i32>),
    Noop,
}

