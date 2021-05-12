use anyhow::Result;
use nightmaregl::{Viewport, Context, VertexData, Renderer};

use crate::listener::{MessageCtx, Listener};
use crate::Message;

pub(super) struct Canvas {
    viewport: Viewport,
    pub(super) visible: bool,
    renderer: Renderer<VertexData>,
    // size: Size<i32>,
    // // pub cursor: Cursor,
    // sprite: Sprite<i32>,
    // current_layer: usize,
    // // layers: Vec<Layer>,
    // // save_buffer: SaveBuffer,
}

impl Canvas {
    fn render(&self, context: &mut Context) -> Result<()> {
        Ok(())
    }
}
