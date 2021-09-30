//! This works as a thin layer for the `Containers`,
//! as containers does not implement `Listener`.
//!
//! So in essence this just routes messages to the `Containers`.
use anyhow::Result;
use nightmare::{ VertexData, Viewport, Transform, Position };
use nightmare::texture::Texture;
use nightmare::events::{ButtonState, MouseButton};

use crate::commandline::Command;
use crate::input::Input;
use crate::listener::{MessageCtx, Listener};
use crate::plugins::Plugin;
use crate::{Coords, Message};

pub mod message;
mod containers;
mod layer;
mod image;
mod cursor;
mod container;
mod savebuffer;

use crate::config::Action;

pub use container::Container;
pub use containers::Containers;
pub use cursor::Cursor;
pub use image::Image;
pub use layer::LayerId;
pub use savebuffer::SaveBuffer;

pub struct Canvas {
    /// All <whatevers> 
    containers: Containers,
    /// Background for transparency
    background: Texture,
    /// Plugin
    plugin: Plugin,

    drag_pos: Option<Position>,
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
            drag_pos: None,
        };

        Ok(inst)
    }

    fn change_cursor_coords(&mut self, coords: Coords) -> Message {
        let coords = self.containers.selected().move_cursor_by(coords);
        eprintln!("{:?}", coords);
        self.containers.update_coords(coords);
        Message::CursorCoords(coords)
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
                use Action::*;
                match action {
                    Left => return self.change_cursor_coords(Coords::new(-1, 0)),
                    Right => return self.change_cursor_coords(Coords::new(1, 0)),
                    Up => return self.change_cursor_coords(Coords::new(0, -1)),
                    Down => return self.change_cursor_coords(Coords::new(0, 1)),
                    CanvasZoomIn => self.containers.selected().scale += 1,
                    CanvasZoomOut => self.containers.selected().scale -= 1,
                    _ => {}
                }
            }
            Message::Input(Input::Mouse(mouse), _) => {
                // Convert the mouse position to image coords
                // for the selected canvas.

                let node = self.containers.selected().node;
                let anchor = node.sprite.anchor.cast::<f32>();
                let node_pos = node.transform.translation;

                let padding = self.containers.viewport.position;
                let mut offset_pos = mouse.pos.cast::<f32>() - node_pos.cast::<f32>() - padding.cast::<f32>();

                let scale = self.containers.selected().scale.cast::<f32>();
                offset_pos.x /= scale.x;
                offset_pos.y /= scale.y;
                offset_pos += anchor;
                offset_pos.y = node.sprite.size.height as f32 - offset_pos.y;

                let coords = Coords::from(offset_pos);

                self.containers.update_coords(coords);
                match mouse.state {
                    ButtonState::Pressed => {
                        if let Some(MouseButton::Left) = mouse.button {
                            self.containers.draw(coords);
                        }

                        if let Some(MouseButton::Right) = mouse.button {
                            self.containers.clear_pixel(coords);
                        }

                        if let Some(MouseButton::Middle) = mouse.button {
                            match self.drag_pos {
                                None => self.drag_pos = Some(mouse.pos),
                                Some(pos) => {
                                    let diff = mouse.pos - pos;
                                    self.drag_pos = Some(mouse.pos);
                                    self.containers.move_canvas(diff);
                                }
                            }
                            // Check difference and move canvas accordingly
                            // start tracking the mouse down point
                        }
                    }
                    ButtonState::Released => {
                        if let Some(MouseButton::Middle) = mouse.button {
                            self.drag_pos = None;
                        }
                    }
                }

                return Message::CursorCoords(coords);
            }
            Message::Input(Input::Scroll(delta), _) => {
                self.containers.change_scale(*delta);
            }
            // Unhandled messages
            Message::Input(_, _)
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
        self.containers.render(&self.background, ctx)
    }

}
