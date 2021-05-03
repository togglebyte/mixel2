use nightmaregl::events::Key;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Char(char),
    Key(Key),
}

impl Input {
    pub fn from_char(mut c: char) -> Input {
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
}
