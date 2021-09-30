use nightmare::{Position, Size};
use nightmare::pixels::Pixel;

use crate::layout::Split;
use crate::canvas::LayerId;
use crate::plugins::PluginCall;

#[derive(Debug)]
pub enum Command {
    Noop,
    Quit,
    Save { path: String, overwrite: bool },
    Put(Position),
    SetColour(Pixel),
    SetAlpha(u8),
    Clear(Position),
    NewImage(Size),
    Split(Split),
    CloseSelectedSplit,
    NewLayer,
    RemoveLayer,
    ChangeLayer(LayerId),
    Lua(String),
    Log(String),
}
