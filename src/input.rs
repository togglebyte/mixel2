use nightmaregl::events::{Modifiers, Key};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Char(char, Modifiers),
    Key(Key, Modifiers),
}

impl Input {
    pub fn from_char(c: char, m: Modifiers) -> Input {
        Input::Char(c, m)
    }

    pub fn from_key(k: Key, m: Modifiers) -> Option<Input> {
        match k {
            Key::Left => Some(Input::Key(k, m)),
            Key::Right => Some(Input::Key(k, m)),
            Key::Up => Some(Input::Key(k, m)),
            Key::Down => Some(Input::Key(k, m)),
            Key::Escape => Some(Input::Key(k, m)),
            Key::LControl => Some(Input::Key(k, m)),
            Key::RControl => Some(Input::Key(k, m)),
            Key::LShift => Some(Input::Key(k, m)),
            Key::RShift => Some(Input::Key(k, m)),
            Key::Back => Some(Input::Key(k, m)),
            Key::Return => Some(Input::Key(k, m)),
            _ => None,
        }
    }
}
