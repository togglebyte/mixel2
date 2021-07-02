//! This works as a thin layer for the `Containers`,
//! as containers does not implement `Listener`.
//!
//! So in essence this just routes messages to the `Containers`.
use anyhow::Result;
use nightmaregl::{ Renderer, VertexData, Viewport, Transform, Position };
use nightmaregl::texture::Texture;

use crate::listener::{MessageCtx, Listener};
use crate::Message;
use crate::commandline::Command;
use crate::input::Input;
use crate::plugins::Plugin;

pub mod message;
mod containers;
mod layer;
mod image;
mod cursor;
mod container;
mod savebuffer;

pub use container::Container;
pub use containers::Containers;
pub use cursor::Cursor;
pub use image::Image;
pub use layer::LayerId;
pub use savebuffer::SaveBuffer;

/// Coords in canvas space
pub struct Coords(Position<i32>);

pub struct Canvas {
    /// All <whatevers> 
    containers: Containers,
    /// Background for transparency
    background: Texture<i32>,
    /// Plugin
    plugin: Plugin,
}

impl Canvas {
    pub fn new(viewport: Viewport, ctx: &mut MessageCtx) -> Result<Self> {
        // TODO: pfft, lazy! 
        //       Don't panic, return the error good and proper.
        // let plugin = match Plugin::new().map_err(|p| {}) {
        let plugin = match Plugin::new() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{:?}", e);
                panic!();
            }
        };

        let inst = Self {
            background: Texture::from_disk("background.png")?,
            containers: Containers::new(viewport, ctx)?,
            plugin,
        };

        Ok(inst)
    }
}

impl Listener for Canvas {
    fn message(&mut self, message: &Message, ctx: &mut MessageCtx) -> Message {
        match message {
            Message::Resize(new_size) => {
                self.containers.resize(new_size.cast());
            }
            Message::Command(Command::Split(dir)) => {
                self.containers.split(*dir, ctx);
            }
            Message::Command(Command::NewImage(size)) => {
                let image = Image::new(*size);
                self.containers.add_image(size.cast(), image);
            }
            Message::Command(Command::CloseSelectedSplit) => {
                self.containers.close_selected();
            }
            Message::Command(Command::Put(pos)) => {
                let coords = Coords(*pos);
                self.containers.draw(coords);
            }
            Message::Command(Command::Clear(pos)) => {
                let coords = Coords(*pos);
                self.containers.clear_pixel(coords);
            }
            Message::Command(Command::SetColour(colour)) => {
                self.containers.set_colour(*colour);
            }
            Message::Command(Command::SetAlpha(alpha)) => {
                self.containers.set_alpha(*alpha);
            }
            Message::Command(Command::NewLayer) => {
                match self.containers.new_layer() {
                    Some((layer, total_layers)) => return Message::LayerChanged { layer, total_layers },
                    None => {}
                }
            }
            Message::Command(Command::RemoveLayer) => {
                match self.containers.remove_layer() {
                    Some((layer, total_layers)) => return Message::LayerChanged { layer, total_layers },
                    None => {}
                }
            }
            Message::Command(Command::ChangeLayer(layer)) => {
                match self.containers.set_layer(*layer) {
                    Some((layer, total_layers)) => return Message::LayerChanged { layer, total_layers },
                    None => {}
                }
                
            }
            Message::Command(Command::Save { path, overwrite }) => {
                self.containers.save_current(path, *overwrite, ctx.context);
            }
            Message::ReloadPlugin(path) => {
                self.plugin.reload(path);
            }
            Message::Command(Command::Lua(code)) => {
                self.plugin.exec_code(code, &mut self.containers);
            }
            Message::Action(action) => {
                return self.containers.action(*action);
            }
            Message::Mouse(mouse) => {
                let pos = self.containers.mouse_input(*mouse, ctx);
                return Message::TranslatedCursor(pos);
            }
            Message::Input(Input::Scroll(delta), _) => {
                self.containers.change_scale(*delta);
            }

            // Unhandled messages
            Message::Input(_, _)
            | Message::CursorPos(_)
            | Message::ModeChanged(_)
            | Message::Command(_)
            | Message::TranslatedCursor(_)
            | Message::LayerChanged { .. }
            | Message::Noop => {}
        }

        Message::Noop
    }

    fn render(&mut self, ctx: &mut MessageCtx) -> Result<()> {
        self.containers.render(&self.background, ctx)
    }
}
