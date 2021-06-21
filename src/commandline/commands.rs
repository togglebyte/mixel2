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
    Extend(Extent),
    Put(Position<i32>),
    SetColour(Pixel),
    SetAlpha(usize),
    Clear(Position<i32>),
    NewImage(Size<i32>),
    Split(Split),
    CloseSelectedSplit,
    NewLayer,
    RemoveLayer,
    ChangeLayer(LayerId),
    Lua(String),
}

#[derive(Debug, Default)]
pub struct Extent {
    pub left: i32,
    pub right: i32,
    pub up: i32,
    pub down: i32,
}
