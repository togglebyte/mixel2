use anyhow::Result;
use nightmare::events::{ButtonState, MouseButton};
use nightmare::{Context, Texture, Position, VertexData};
use nightmare::render2d::SimpleRenderer;

use crate::listener::{Listener, MessageCtx};
use crate::message::Message;
use crate::node::Node;
use crate::input::Input;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Mouse {
    pub state: ButtonState,
    pub button: Option<MouseButton>,
    pub pos: Position<i32>,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            state: ButtonState::Released,
            button: None,
            pos: Position::zero(),
        }
    }
}

pub struct MouseCursor {
    node: Node<i32>,
    texture: Texture<i32>,
    renderer: Renderer<VertexData>,
}

impl MouseCursor {
    pub fn new(ctx: &mut MessageCtx, viewport: Viewport) -> Result<Self> {
        let texture = Texture::from_disk("cursor.png")?;
        let renderer = SimpleRenderer::new(ctx.context, viewport.view_projection())?;
        let mut node = Node::new(&texture);
        node.sprite.z_index = 10;
        node.sprite.anchor = (node.sprite.size / 2).to_vector();

        let inst = Self {
            node,
            texture,
            renderer,
        };

        Ok(inst)
    }

}

impl Listener for MouseCursor {
    fn message(&mut self, msg: &Message, ctx: &mut MessageCtx) -> Message {
        if let Message::Input(Input::Mouse(mouse), _) = msg {
            self.node.transform.translate_mut(mouse.pos);
        }
        
        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) -> Result<()> {
        self.renderer.render(
           &self.texture,
           &[self.node.vertex_data()],
           ctx.app_viewport,
           ctx.context,
        );
        Ok(())
    }
}
