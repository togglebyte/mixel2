use anyhow::Result;
use nightmaregl::{Texture, Position, Pixels, Pixel, VertexData, Context, Size, Renderer, Viewport};
use nightmaregl::text::{Text, WordWrap};

use crate::application::Mode;

pub struct Status {
    text: Text,
    cursor: Position<i32>,
    mode: Mode,
    cur_pos: Position<i32>,
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
            context
        )?;

        text.position(position.cast());

        let renderer = Renderer::default_font(context)?;

        let inst = Self {
            text,
            cursor: Position::zero(),
            cur_pos: Position::zero(),
            mode: Mode::Normal,
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

    fn update_text(&mut self) {
        let text = format!("x: {} y: {} | mode: {:?}", self.cur_pos.x, self.cur_pos.y, self.mode);
        self.text.set_text(text);
    }

    pub fn render(&self, context: &mut Context) {
        self.renderer.render(
            self.text.texture(),
            &self.text.vertex_data(),
            &self.viewport,
            context,
        );
    }
}
