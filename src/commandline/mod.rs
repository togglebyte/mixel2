use anyhow::Result;
use log::error;
use nightmare::events::Key;
use nightmare::pixels::{Pixel, Pixels};
use nightmare::render2d::{Model, SimpleRenderer};
use nightmare::text::{default_font_shader, Text, WordWrap};
use nightmare::{Context, Position, Size, Texture, VertexData, Viewport};

use crate::application::Mode;
use crate::input::Input;
use crate::listener::{Listener, MessageCtx};
use crate::message::Message;
use crate::Node;

mod commands;
mod parser;

pub use commands::Command;
use parser::Parser;

// -----------------------------------------------------------------------------
//     - Command line -
// -----------------------------------------------------------------------------
pub struct CommandLine {
    text_renderer: SimpleRenderer<Model>,
    font_size: f32,
    caret: Caret,
    viewport: Viewport,
    text: Text,
    input_buffer: String,
    visible_buffer: String,
    mode: Mode,
}

impl CommandLine {
    pub fn new(size: Size, context: &mut Context) -> Result<Self> {
        let font_size = 18.0;

        let viewport = Viewport::new(Position::new(0.0, 0.0), viewport_size(size, font_size));

        let shader = default_font_shader()?;
        let mut text_renderer = SimpleRenderer::new(context, viewport.view_projection())?;
        text_renderer.set_shader(shader, viewport.view_projection(), context);

        let mut text = Text::from_path("/usr/share/fonts/TTF/Hack-Regular.ttf", font_size, WordWrap::NoWrap, context)?;

        text.position(Position::new(0.0, font_size / 1.7));
        text.z_index(9999);

        let inst = Self {
            text_renderer,
            font_size,
            caret: Caret::new(font_size, context, &viewport)?,
            viewport,
            text,
            input_buffer: String::new(),
            visible_buffer: String::new(),
            mode: Mode::Normal,
        };

        Ok(inst)
    }

    fn input(&mut self, input: Input, context: &mut Context) -> Option<Command> {
        match self.mode {
            Mode::Command => {}
            _ => return None,
        }

        match input {
            Input::Char(c) if c.is_control() => {}
            Input::Char(c) => {
                self.input_buffer.push(c);
                self.visible_buffer.push(c);
                self.update_text(context);
            }
            Input::Key(key) => match key {
                Key::Back => {
                    self.visible_buffer.pop();
                    self.input_buffer.pop();
                    self.update_text(context);
                }
                Key::Return => {
                    let input = self.input_buffer.drain(..).collect::<String>();
                    let parser = Parser::new(&input);
                    let command = parser.parse();
                    self.visible_buffer.clear();
                    self.update_text(context);
                    return Some(command);
                }
                _ => {}
            },
            Input::Mouse(_) => {}
            Input::Scroll(_) => {}
        }

        None
    }

    fn update_text(&mut self, context: &mut Context) {
        if let Err(e) = self.text.set_text(&self.visible_buffer) {
            error!("Failed to set text: {:?}", e);
        }

        while self.text.caret().x + self.caret.node.sprite.size.x > self.viewport.size().x {
            if self.visible_buffer.is_empty() {
                break;
            }
            self.visible_buffer.drain(..1);
            if let Err(e) = self.text.set_text(&self.visible_buffer) {
                error!("Failed to set text: {:?}", e);
            }
        }

        self.caret.node.transform.isometry.translation = Position::new(self.text.caret().x, self.font_size / 3.0).into();

        let models = self.text.models();
        self.text_renderer.load_data(&models, context);
    }
}

// -----------------------------------------------------------------------------
//     - Listener -
// -----------------------------------------------------------------------------
impl Listener for CommandLine {
    fn message(&mut self, message: &Message, ctx: &mut MessageCtx) -> Message {
        match message {
            Message::Input(input, _) => {
                return self
                    .input(*input, ctx.context)
                    .map(Message::Command)
                    .unwrap_or(Message::Noop)
            }
            Message::Resize(new_size) => {
                self.viewport
                    .resize(viewport_size(*new_size, self.font_size));
            }
            Message::ModeChanged(mode) => {
                self.mode = *mode;
                self.visible_buffer.clear();
                self.input_buffer.clear();
            }
            Message::CursorPos(_)
            | Message::Action(_)
            | Message::Command(_)
            | Message::CursorCoords(_)
            // | Message::LayerChanged { .. }
            | Message::ReloadPlugin(_)
            | Message::Noop => {}
        }

        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) {
        match self.mode {
            Mode::Command => {}
            _ => return,
        }

        self.text.texture().bind();

        self.text_renderer.render_instanced(ctx.context, 1);
        self.caret.render(ctx.context, &self.viewport);
    }
}

// -----------------------------------------------------------------------------
//     - Caret -
// -----------------------------------------------------------------------------
struct Caret {
    renderer: SimpleRenderer<Model>,
    texture: Texture,
    node: Node,
}

impl Caret {
    pub fn new(font_size: f32, context: &mut Context, viewport: &Viewport) -> Result<Self> {
        let shader = default_font_shader()?;
        let mut renderer = SimpleRenderer::new(context, viewport.view_projection())?;
        renderer.set_shader(shader, viewport.view_projection(), context);

        let caret_size = Size::new(font_size, font_size * 2.0);
        let caret_pixels = Pixels::from_pixel(Pixel::white(), Size::new(1.0, 1.0));

        let texture = Texture::default_with_data(Size::new(1.0, 1.0), caret_pixels.as_bytes());
        let mut node = Node::new(&texture);
        node.sprite.size = caret_size;

        let inst = Self { renderer, texture, node };

        Ok(inst)
    }

    fn render(&mut self, context: &mut Context, viewport: &Viewport) {
        self.renderer.load_data(&[self.node.model()], context);
        self.renderer.render_instanced(context, 1);
    }
}

// -----------------------------------------------------------------------------
//     - Viewport size -
//     Used when resizing
// -----------------------------------------------------------------------------
fn viewport_size(size: Size, font_size: f32) -> Size {
    Size::new(size.x, font_size * 2.0)
}
