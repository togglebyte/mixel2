use anyhow::Result;
use nightmare::events::{ButtonState, MouseButton};
use nightmare::{Context, Texture, Position, VertexData, Viewport};
use nightmare::render2d::{SimpleRenderer, Model};

use crate::listener::{Listener, MessageCtx};
use crate::message::Message;
use crate::node::Node;
use crate::input::Input;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Mouse {
    pub state: ButtonState,
    pub button: Option<MouseButton>,
    pos: (i32, i32),
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            state: ButtonState::Released,
            button: None,
            pos: (0, 0),
        }
    }

    pub fn pos(&self) -> Position {
        Position::new(self.pos.0 as f32, self.pos.1 as f32)
    }

    pub fn set_pos(&mut self, pos: Position) {
        self.pos = (pos.x as i32, pos.y as i32)
    }
}


pub struct MouseCursor {
    node: Node,
    texture: Texture,
    renderer: SimpleRenderer<Model>,
}

impl MouseCursor {
    pub fn new(ctx: &mut MessageCtx) -> Result<Self> {
        let texture = Texture::from_disk("cursor.png")?;
        let renderer = SimpleRenderer::new(ctx.context, ctx.app_viewport.view_projection())?;
        let mut node = Node::new(&texture);
        node.sprite.z_index = 10;
        node.sprite.anchor = node.sprite.size / 2.0;

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
            self.node.transform.isometry.translation = mouse.pos().into();
            let model = self.node.model();
            self.renderer.load_data(&[model], ctx.context);
        }
        
        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) {
        self.renderer.render_instanced(ctx.context, 1);
    }
}
