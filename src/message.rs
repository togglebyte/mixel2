use std::path::PathBuf;

use nightmare::events::{MouseButton, ButtonState, Modifiers};
use nightmare::{Position, Size};

use crate::application::Mode;
use crate::canvas::LayerId;
use crate::commandline::Command;
use crate::config::Action;
use crate::input::Input;
use crate::{Mouse, Coords};

#[derive(Debug)]
pub enum Message {
    Input(Input, Modifiers),
    CursorPos(Position),
    Resize(Size),
    ModeChanged(Mode),
    Command(Command),
    Action(Action),
    CursorCoords(Coords),
    LayerChanged { layer: LayerId, total_layers: usize },
    ReloadPlugin(PathBuf),
    Noop,
}

