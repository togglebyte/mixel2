use anyhow::Result;
use nightmaregl::{Viewport, Context, VertexData, Renderer};

use crate::listener::{MessageCtx, Listener};
use crate::Message;

pub(super) struct Canvas {
    viewport: Viewport,
    pub(super) visible: bool,
    renderer: Renderer<VertexData>,
}

impl Canvas {
    fn render(&self, context: &mut Context) -> Result<()> {
        Ok(())
    }
}
