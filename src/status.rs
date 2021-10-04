use anyhow::Result;
use log::error;
use nightmare::text::{Text, WordWrap};
use nightmare::{Context, Position, Size, VertexData, Viewport};
use nightmare::render2d::{SimpleRenderer, Model};

use crate::application::Mode;
use crate::canvas::LayerId;
use crate::commandline::Command;
use crate::listener::{Listener, MessageCtx};
use crate::message::Message;
use crate::Coords;

pub struct Status {
    dirty: bool,
    text: Text,
    mode: Mode,
    cursor_coords: Coords,
    layer: LayerId,
    total_layers: usize,
    renderer: SimpleRenderer<Model>,
    viewport: Viewport,
}

impl Status {
    pub fn new(size: Size, context: &mut Context) -> Result<Self> {
        let font_size = 18.0;
        let position = {
            Position::new(10.0, size.height as f32 - 10.0 - font_size * 2.0)
        };

        let mut text = Text::from_path(
            "/usr/share/fonts/TTF/Hack-Regular.ttf",
            font_size,
            WordWrap::NoWrap,
            context,
        )?;

        text.position(position);
        text.z_index(9999);

        let viewport = Viewport::new(Position::zeros(), size);
        let renderer = SimpleRenderer::new(context, viewport.view_projection())?;

        let mut inst = Self {
            dirty: true,
            text,
            cursor_coords: Coords::zeros(),
            mode: Mode::Normal,
            layer: LayerId::from_display(1),
            total_layers: 1,
            viewport,
            renderer,
        };

        Ok(inst)
    }

    fn update_text(&mut self) {
        let text = format!(
            "x: {} y: {} | mode: {:?} | layer: {}/{}",
            self.cursor_coords.0.x,
            self.cursor_coords.0.y,
            self.mode,
            self.layer.as_display(),
            self.total_layers,
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
                self.dirty = true;
            }
            Message::Resize(ref size) => self.viewport.resize(*size),
            Message::CursorCoords(coords) => {
                self.cursor_coords = *coords;
                self.dirty = true;
            }
            Message::LayerChanged { layer, total_layers } => {
                self.layer = *layer;
                self.total_layers = *total_layers;
                self.dirty = true;
            }
            Message::Input(_, _)
            | Message::CursorPos(_)
            | Message::Action(_)
            | Message::Command(_)
            | Message::ReloadPlugin(_)
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
