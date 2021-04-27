use nightmaregl::{Rect, Position};
use nightmaregl::pixels::{Pixel, Pixels};

use super::draw::Layer;

pub struct Undo<'a> {
    inner: Vec<Snapshot<'a>>,
    max_size: usize,
}

impl<'a> Undo<'a> {
}

pub struct Snapshot<'a> {
    rect: Rect<i32>,
    pos: Position<i32>,
    pixels: Pixels<Pixel>,
    layer: &'a Layer,
}
