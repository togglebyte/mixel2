use nightmare::events::{ButtonState, MouseButton, Key};
use nightmare::Position;

use crate::application::Mode;
use crate::listener::{Listener, MessageCtx};
use crate::message::Message;
use crate::Mouse;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Char(char),
    Key(Key),
    Mouse(Mouse),
    Scroll(i32),
}

impl Input {
    pub fn from_char(c: char) -> Input {
        Input::Char(c)
    }

    pub fn from_key(k: Key) -> Option<Input> {
        match k {
            Key::Left => Some(Input::Key(k)),
            Key::Right => Some(Input::Key(k)),
            Key::Up => Some(Input::Key(k)),
            Key::Down => Some(Input::Key(k)),
            Key::Escape => Some(Input::Key(k)),
            Key::LControl => Some(Input::Key(k)),
            Key::RControl => Some(Input::Key(k)),
            Key::LShift => Some(Input::Key(k)),
            Key::RShift => Some(Input::Key(k)),
            Key::Back => Some(Input::Key(k)),
            Key::Return => Some(Input::Key(k)),
            _ => None,
        }
    }

    pub fn mouse(mouse: Mouse) -> Input {
        Input::Mouse(mouse)
    }
}

pub struct InputToAction(Mode);

impl InputToAction {
    pub fn new(mode: Mode) -> Self {
        Self(mode)
    }
}

impl Listener for InputToAction {
    fn message(&mut self, msg: &Message, context: &mut MessageCtx) -> Message {
        match (self.0, msg) {
            (Mode::Normal, Message::Input(input, modifiers)) => {
                Message::Action(context.config.key_map(*input, *modifiers))
            }
            (Mode::Insert, Message::Input(input, modifiers)) => {
                Message::Action(context.config.key_map(*input, *modifiers))
            }
            (_, Message::ModeChanged(mode)) => {
                self.0 = *mode;
                Message::Noop
            }
            _ => Message::Noop,
        }
    }
}
