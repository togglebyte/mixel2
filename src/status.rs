use anyhow::Result;
use log::error;
use nightmaregl::text::{Text, WordWrap};
use nightmaregl::{
    Context, Position, Renderer, Size, VertexData, Viewport,
};

use crate::listener::{Listener, MessageCtx};
use crate::message::Message;
use crate::application::Mode;

pub struct Status {
    dirty: bool,
    text: Text,
    mode: Mode,
    cur_pos: Position<i32>,
    raw_mouse: Position<i32>,
    translated_mouse: Position<i32>,
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

        let mut inst = Self {
            dirty: true,
            text,
            cur_pos: Position::new(0, 0),
            raw_mouse: Position::new(0, 0),
            translated_mouse: Position::new(0, 0),
            mode: Mode::Normal,
            layer: 0,
            viewport: Viewport::new(Position::zero(), size),
            renderer,
        };

        Ok(inst)
    }

    fn update_text(&mut self) {
        let text = format!(
            "x: {} y: {} | mode: {:?} | layer: {} | mouse x: {} y: {} (raw) mouse x: {} y: {}",
            self.cur_pos.x, self.cur_pos.y, 
            self.mode, self.layer, 
            self.translated_mouse.x, self.translated_mouse.y,
            self.raw_mouse.x, self.raw_mouse.y
        );
        if let Err(e) = self.text.set_text(text) {
            error!("Failed to update text: {:?}", e);
        }
    }
}

// -----------------------------------------------------------------------------
//     - Listener -
// -----------------------------------------------------------------------------
impl Listener for Status {
    fn message(&mut self, message: &Message, ctx: &mut MessageCtx) -> Message {
        match message {
            Message::ModeChanged(mode) => {
                self.mode = *mode;
                self.dirty = true
            }
            Message::CursorPos(pos) => {
                self.cur_pos = *pos;
                self.dirty = true
            }
            Message::Resize(size) => self.viewport.resize(*size),
            Message::TranslatedMouse(pos) => {
                self.translated_mouse = *pos;
                self.dirty = true
            }
            Message::Mouse(mouse) => {
                self.raw_mouse = mouse.pos;
                self.dirty = true
            }
            | Message::Input(_, _)
            | Message::Action(_)
            | Message::Command(_)
            | Message::Noop => {}
        }

        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) -> Result<()> {
        if self.dirty {
            self.dirty = false;
            self.update_text();
        }
        self.renderer.render(
            self.text.texture(),
            &self.text.vertex_data(),
            &self.viewport,
            ctx.context,
        )?;
        Ok(())
    }
}
