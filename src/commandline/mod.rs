use anyhow::Result;
use log::error;
use nightmaregl::events::Key;
use nightmaregl::text::{Text, WordWrap};
use nightmaregl::pixels::{Pixel, Pixels};
use nightmaregl::{
    Context, Position, Renderer, Size, Sprite, Texture, VertexData, Viewport,
};

use crate::config::Config;
use crate::input::Input;
use crate::listener::{Listener, Message};
use crate::application::Mode;

mod commands;
mod parser;

pub use commands::Command;
use parser::Parser;

// -----------------------------------------------------------------------------
//     - Command line -
// -----------------------------------------------------------------------------
pub struct CommandLine {
    text_renderer: Renderer<VertexData>,
    font_size: f32,
    cursor: Cursor,
    viewport: Viewport,
    text: Text,
    input_buffer: String,
    visible_buffer: String,
    mode: Mode,
}

impl CommandLine {
    pub fn new(size: Size<i32>, context: &mut Context) -> Result<Self> {
        let text_renderer = Renderer::default_font(context)?;
        let font_size = 18.0;
        let viewport = Viewport::new(Position::new(0, 0), viewport_size(size, font_size));

        let mut text = Text::from_path(
            "/usr/share/fonts/TTF/Hack-Regular.ttf",
            font_size,
            WordWrap::NoWrap,
            context,
        )?;

        text.position(Position::new(0.0, font_size / 1.7));

        let inst = Self {
            text_renderer,
            font_size,
            cursor: Cursor::new(font_size, context)?,
            viewport,
            text,
            input_buffer: String::new(),
            visible_buffer: String::new(),
            mode: Mode::Normal,
        };

        Ok(inst)
    }

    fn render(&mut self, context: &mut Context) {
    }

    fn resize(&mut self, new_size: Size<i32>) {
        self.viewport
            .resize(viewport_size(new_size, self.font_size))
    }

    fn is_empty(&self) -> bool {
        self.input_buffer.is_empty()
    }

    fn input(&mut self, input: Input) -> Option<Command> {
        match self.mode {
            Mode::Command => {}
            _ => return None,
        }

        match input {
            Input::Char(c) if c.is_control() => {}
            Input::Char(c) => {
                self.input_buffer.push(c);
                self.visible_buffer.push(c);
                self.update_text();
            }
            Input::Key(key) => match key {
                Key::Back => {
                    self.visible_buffer.pop();
                    self.input_buffer.pop();
                    self.update_text();
                }
                Key::Return => {
                    let input = self.input_buffer.drain(..).collect::<String>();
                    let parser = Parser::new(&input);
                    let command = parser.parse();
                    self.visible_buffer.clear();
                    self.update_text();
                    return Some(command);
                }
                _ => {}
            },
        }

        None
    }

    fn update_text(&mut self) {
        if let Err(e) = self.text.set_text(&self.visible_buffer) {
            error!("Failed to set text: {:?}", e);
        }

        while self.text.caret().x + self.cursor.sprite.size.width
            > self.viewport.size().width as f32
        {
            if self.visible_buffer.is_empty() {
                break;
            }
            self.visible_buffer.drain(..1);
            if let Err(e) = self.text.set_text(&self.visible_buffer) {
                error!("Failed to set text: {:?}", e);
            }
        }

        self.cursor.sprite.position = Position::new(self.text.caret().x, self.font_size / 3.0);
    }
}

// -----------------------------------------------------------------------------
//     - Listener -
// -----------------------------------------------------------------------------
impl Listener for CommandLine {
    fn message(&mut self, message: &Message, _: &Config) -> Message {
        match message {
            Message::Input(input, modifiers) => return self.input(*input).map(Message::Command).unwrap_or(Message::Noop),
            Message::Resize(new_size) => self.resize(*new_size),
            Message::ModeChanged(mode) => {
                self.mode = *mode;
                self.visible_buffer.clear();
                self.input_buffer.clear();
            }
            Message::CursorPos(_)
            | Message::Command(_) => { }
            _ => {}
        }

        Message::Noop
    }

    fn render(&mut self, context: &mut Context) {
        match self.mode {
            Mode::Command => {}
            _ => return
        }

        let texture = self.text.texture();
        let text_vertex_data = self.text.vertex_data();

        let res = self
            .text_renderer
            .render(texture, &text_vertex_data, &self.viewport, context);

        if let Err(e) = res {
            error!("Failed to render text: {:?}", e);
        }

        self.cursor.render(context, &self.viewport);
    }
}

// -----------------------------------------------------------------------------
//     - Cursor -
// -----------------------------------------------------------------------------
struct Cursor {
    renderer: Renderer<VertexData>,
    texture: Texture<f32>,
    sprite: Sprite<f32>,
}

impl Cursor {
    pub fn new(font_size: f32, context: &mut Context) -> Result<Self> {
        let renderer = Renderer::default_font(context)?;

        let cursor_size = Size::new(font_size, font_size * 2.0);
        let cursor_pixels = Pixels::from_pixel(Pixel::white(), Size::new(1, 1));

        let texture = Texture::default_with_data(Size::new(1.0, 1.0), cursor_pixels.as_bytes());
        let mut sprite = Sprite::new(&texture);
        sprite.size = cursor_size;

        let inst = Self {
            renderer,
            texture,
            sprite,
        };

        Ok(inst)
    }

    fn render(&mut self, context: &mut Context, viewport: &Viewport) {
        let res = self.renderer.render(
            &self.texture,
            &[self.sprite.vertex_data()],
            viewport,
            context,
        );

        if let Err(e) = res {
            error!("cursor renderer failed: {:?}", e);
        }
    }
}

// -----------------------------------------------------------------------------
//     - Viewport size -
//     Used when resizing
// -----------------------------------------------------------------------------
fn viewport_size(size: Size<i32>, font_size: f32) -> Size<i32> {
    Size::new(size.width, (font_size * 2.0) as i32)
}
