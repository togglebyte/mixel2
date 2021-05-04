use anyhow::Result;
use nightmaregl::text::{Text, WordWrap};
use nightmaregl::pixels::{Pixel, Pixels};
use nightmaregl::{
    Context, Position, Renderer, Size, Texture, VertexData, Viewport,
};

use crate::config::Config;
use crate::listener::{Listener, Message};
use crate::application::Mode;

pub struct Status {
    text: Text,
    mode: Mode,
    cur_pos: Position<i32>,
    layer: usize,
    renderer: Renderer<VertexData>,
    viewport: Viewport,
}

impl Status {
    pub fn new(size: Size<i32>, context: &mut Context) -> Result<Self> {
        let font_size = 18.0;
        let position = {
            let size = size.cast::<f32>();
            Position::new(10.0, size.height - 10.0 - font_size * 2.0)
        };

        let mut text = Text::from_path(
            "/usr/share/fonts/TTF/Hack-Regular.ttf",
            font_size,
            WordWrap::NoWrap,
            context,
        )?;

        text.position(position.cast());

        let renderer = Renderer::default_font(context)?;

        let inst = Self {
            text,
            cur_pos: Position::new(-1, -1),
            mode: Mode::Normal,
            layer: 0,
            viewport: Viewport::new(Position::zero(), size),
            renderer,
        };

        Ok(inst)
    }

    pub fn set_mode(&mut self, mode: Mode) {
        if mode != self.mode {
            self.mode = mode;
            self.update_text();
        }
    }

    pub fn set_cur_pos(&mut self, pos: Position<i32>) {
        if pos != self.cur_pos {
            self.cur_pos = pos;
            self.update_text();
        }
    }

    pub fn set_layer(&mut self, layer: usize) {
        if layer != self.layer {
            self.layer = layer;
            self.update_text();
        }
    }

    fn update_text(&mut self) {
        let text = format!(
            "x: {} y: {} | mode: {:?} | layer: {}",
            self.cur_pos.x, self.cur_pos.y, self.mode, self.layer
        );
        self.text.set_text(text);
    }
}

// -----------------------------------------------------------------------------
//     - Listener -
// -----------------------------------------------------------------------------
impl Listener for Status {
    fn message(&mut self, message: &Message, _: &Config) -> Message {
        match message {
            Message::ModeChanged(mode) => {
                self.mode = *mode;
                self.update_text();
            }
            Message::CursorPos(pos) => {
                self.cur_pos = *pos;
                self.update_text();
            }
            Message::Resize(size) => self.viewport.resize(*size),
            | Message::Input(_, _)
            | Message::Action(_)
            | Message::Noop
            | Message::Command(_) => {}
        }

        Message::Noop
    }

    fn render(&mut self, context: &mut Context) {
        self.renderer.render(
            self.text.texture(),
            &self.text.vertex_data(),
            &self.viewport,
            context,
        );
    }
}
