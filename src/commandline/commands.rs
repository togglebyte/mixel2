use nightmaregl::{Position, Size};
use nightmaregl::pixels::Pixel;

use crate::canvas::Orientation;
use crate::canvas::LayerId;

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
    Split(Orientation),
    CloseSelectedSplit,
    NewLayer,
    RemoveLayer,
    ChangeLayer(LayerId),
}

#[derive(Debug, Default)]
pub struct Extent {
    pub left: i32,
    pub right: i32,
    pub up: i32,
    pub down: i32,
}
