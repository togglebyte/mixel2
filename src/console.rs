use anyhow::Result;
use nightmaregl::{Position, Context, Renderer, VertexData, Viewport, Sprite, Texture, Size};
use nightmaregl::pixels::{Pixels, Pixel};
use nightmaregl::text::{WordWrap, Text};

use crate::commandline::Command;
use crate::listener::{Listener, MessageCtx};
use crate::{Message, Node};

pub struct Console {
    visible: bool,
    lines: Vec<String>,
    renderer: Renderer<VertexData>,
    text_renderer: Renderer<VertexData>,
    viewport: Viewport,
    node: Node<i32>,
    texture: Texture<i32>,
    text: Text,
}

impl Console {
    pub fn new(ctx: &mut MessageCtx) -> Result<Self> {
        let text_renderer = Renderer::default_font(ctx.context)?;
        let renderer = Renderer::default(ctx.context)?;
        let mut size = *ctx.app_viewport.size();
        size.height /= 2;
        let pos = Position::new(0, size.height);

        let pixels = Pixels::from_pixel(Pixel { a: 128, ..Pixel::black() }, Size::new(1, 1));
        let texture = Texture::default_with_data(Size::new(1, 1), pixels.as_bytes());
        let mut node = Node::new(&texture);
        node.sprite.size = size;

        let font_size = 22.0;

        let mut text = Text::from_path(
            "/usr/share/fonts/TTF/Hack-Regular.ttf",
            font_size,
            WordWrap::Normal(size.width as u32),
            ctx.context,
        )?;

        let inst = Self {
            visible: true,
            lines: Vec::new(),
            renderer,
            text_renderer,
            viewport: Viewport::new(pos, size),
            node,
            texture,
            text,
        };

        Ok(inst)
    }

    fn add_line(&mut self, line: &str) {
        self.lines.push(line.to_owned());
        // self.text.set_text(self.lines.join("\n"));
        // self.text.position(Position::new(0.0, y));
    }
}

impl Listener for Console {
    fn message(&mut self, message: &Message, ctx: &mut MessageCtx) -> Message {
        match message {
            Message::Resize(new_size) => {
                self.viewport.resize(*new_size);
                // TODO: resize texture
            }
            Message::Command(Command::Log(line)) => self.add_line(line),
            // Message::ReloadPlugin(path) => self.add_line(&format!("Reloaded \"{}\"", path)),
            Message::ReloadPlugin(path) => self.add_line("Plugins reloaded"),
            Message::Command(_)
            | Message::Input(_, _)
            | Message::Action(_)
            | Message::CursorPos(_)
            | Message::ModeChanged(_)
            | Message::Command(_)
            | Message::CursorCoords(_)
            | Message::LayerChanged { .. }
            | Message::Noop => {}
        }

        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) -> Result<()> {
        if !self.visible {
            return Ok(());
        }

        self.text_renderer.render(
            self.text.texture(),
            &self.text.vertex_data(),
            &self.viewport,
            ctx.context,
        );

        self.renderer.render(
            &self.texture,
            &[self.node.vertex_data()],
            &self.viewport,
            ctx.context,
        );

        Ok(())
    }
}
