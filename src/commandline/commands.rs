use nightmaregl::{Position, Size};
use nightmaregl::pixels::Pixel;

use crate::layout::Split;
use crate::canvas::LayerId;
use crate::plugins::PluginCall;

#[derive(Debug)]
pub enum Command {
    Noop,
    Quit,
    Save { path: String, overwrite: bool },
    Put(Position<i32>),
    SetColour(Pixel),
    SetAlpha(u8),
    Clear(Position<i32>),
    NewImage(Size<i32>),
    Split(Split),
    CloseSelectedSplit,
    NewLayer,
    RemoveLayer,
    ChangeLayer(LayerId),
    Lua(String),
}
