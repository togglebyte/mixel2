use anyhow::Result;
use log::error;
use nightmaregl::events::Key;
use nightmaregl::text::{Text, WordWrap};
use nightmaregl::pixels::{Pixel, Pixels};
use nightmaregl::{
    Context, Position, Renderer, Size, Texture, VertexData, Viewport,
};

use crate::Node;
use crate::application::Mode;
use crate::input::Input;
use crate::listener::{Listener, MessageCtx};
use crate::message::Message;

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
    caret: Caret,
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
        text.z_index(9999);

        let inst = Self {
            text_renderer,
            font_size,
            caret: Caret::new(font_size, context)?,
            viewport,
            text,
            input_buffer: String::new(),
            visible_buffer: String::new(),
            mode: Mode::Normal,
        };

        Ok(inst)
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
            Input::Mouse(_) => {}
            Input::Scroll(_) => {}
        }

        None
    }

    fn update_text(&mut self) {
        if let Err(e) = self.text.set_text(&self.visible_buffer) {
            error!("Failed to set text: {:?}", e);
        }

        while self.text.caret().x + self.caret.node.sprite.size.width
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

        self.caret.node.transform.translate_mut(Position::new(self.text.caret().x, self.font_size / 3.0));
    }
}

// -----------------------------------------------------------------------------
//     - Listener -
// -----------------------------------------------------------------------------
impl Listener for CommandLine {
    fn message(&mut self, message: &Message, _: &mut MessageCtx) -> Message {
        match message {
            Message::Input(input, _) => return self.input(*input).map(Message::Command).unwrap_or(Message::Noop),
            Message::Resize(new_size) => {
                self.viewport.resize(viewport_size(*new_size, self.font_size));
            }
            Message::ModeChanged(mode) => {
                self.mode = *mode;
                self.visible_buffer.clear();
                self.input_buffer.clear();
            }
            Message::CursorPos(_)
            | Message::Action(_)
            | Message::Command(_)
            | Message::Mouse(_)
            | Message::TranslatedCursor(_)
            | Message::LayerChanged { .. }
            | Message::ReloadPlugin(_)
            | Message::Noop => {}
        }

        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) -> Result<()> {
        match self.mode {
            Mode::Command => {}
            _ => return Ok(())
        }

        let texture = self.text.texture();
        let text_vertex_data = self.text.vertex_data();

        self.text_renderer.render(
            texture, 
            &text_vertex_data,
            &self.viewport,
            ctx.context
        )?;

        self.caret.render(ctx.context, &self.viewport);
        Ok(())
    }
}

// -----------------------------------------------------------------------------
//     - Caret -
// -----------------------------------------------------------------------------
struct Caret {
    renderer: Renderer<VertexData>,
    texture: Texture<f32>,
    node: Node<f32>,
}

impl Caret {
    pub fn new(font_size: f32, context: &mut Context) -> Result<Self> {
        let renderer = Renderer::default_font(context)?;

        let caret_size = Size::new(font_size, font_size * 2.0);
        let caret_pixels = Pixels::from_pixel(Pixel::white(), Size::new(1, 1));

        let texture = Texture::default_with_data(Size::new(1.0, 1.0), caret_pixels.as_bytes());
        let mut node = Node::new(&texture);
        node.sprite.size = caret_size;

        let inst = Self {
            renderer,
            texture,
            node,
        };

        Ok(inst)
    }

    fn render(&mut self, context: &mut Context, viewport: &Viewport) {
        let res = self.renderer.render(
            &self.texture,
            &[self.node.vertex_data()],
            viewport,
            context,
        );

        if let Err(e) = res {
            error!("caret renderer failed: {:?}", e);
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
