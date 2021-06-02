use nightmaregl::events::{MouseButton, ButtonState, Modifiers};
use nightmaregl::{Position, Size};

use crate::Mouse;
use crate::application::Mode;
use crate::canvas::LayerId;
use crate::commandline::Command;
use crate::config::Action;
use crate::input::Input;

pub enum Message {
    Input(Input, Modifiers),
    CursorPos(Position<i32>),
    Resize(Size<i32>),
    ModeChanged(Mode),
    Command(Command),
    Action(Action),
    Mouse(Mouse),
    TranslatedMouse(Position<i32>),
    LayerChanged { layer: LayerId, total_layers: usize },
    Noop,
}

