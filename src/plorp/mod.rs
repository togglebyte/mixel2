use anyhow::Result;
use log::error;
use nightmaregl::events::Key;
use nightmaregl::{Context, Position, Renderer, Size, VertexData, Viewport};

use crate::commandline::commands::{Command, Extent};
use crate::config::Action;

mod cursor;
mod draw;
mod savebuffer;
mod undostack;
mod layer;

use border::Border;
use draw::{Direction, Draw};

pub enum DrawMode {
    Normal,
    Square,
    Circle,
}

// -----------------------------------------------------------------------------
//     - Canvas -
// -----------------------------------------------------------------------------
pub struct Canvas {
    viewport: Viewport,
    draw: Draw,
}

impl Canvas {
    pub fn new(window_size: Size<i32>, context: &mut Context) -> Result<Self> {
        // let mut application_renderer = Renderer::default(context)?;
        // application_renderer.pixel_size = 1;
        // let application_viewport = Viewport::new(Position::zero(), window_size);

        // -----------------------------------------------------------------------------
        //     - Canvas viewport -
        // -----------------------------------------------------------------------------
        let viewport = {
            let padding = 256 / application_renderer.pixel_size;
            let pos = application_viewport.position + Position::new(padding, padding);
            let size = *application_viewport.size() - Size::new(padding * 2, padding * 2);

            Viewport::new(pos, size)
        };

        // -----------------------------------------------------------------------------
        //     - Border -
        // -----------------------------------------------------------------------------
        let border = Border::new(
            viewport.position,
            *canvas_viewport.size(),
        )?;

        // -----------------------------------------------------------------------------
        //     - Draw -
        // -----------------------------------------------------------------------------
        let draw = Draw::new(Size::new(32, 32), context, canvas_viewport.centre())?;

        let inst = Self {
            border,
            viewport,
            draw,
        };

        Ok(inst)
    }

    pub fn input(&mut self, action: Action, repeat: u16) {
        for _ in 0..repeat {
            match action {
                Action::Left => self.draw.offset_cursor(Position::new(-1, 0)),
                Action::Right => self.draw.offset_cursor(Position::new(1, 0)),
                Action::Up => self.draw.offset_cursor(Position::new(0, -1)),
                Action::Down => self.draw.offset_cursor(Position::new(0, 1)),

                Action::UpLeft => self.draw.offset_cursor(Position::new(-1, -1)),
                Action::UpRight => self.draw.offset_cursor(Position::new(1, -1)),
                Action::DownLeft => self.draw.offset_cursor(Position::new(-1, 1)),
                Action::DownRight => self.draw.offset_cursor(Position::new(1, 1)),

                Action::CanvasLeft => self.draw.offset_canvas(Position::new(-1, 0)),
                Action::CanvasRight => self.draw.offset_canvas(Position::new(1, 0)),
                Action::CanvasUp => self.draw.offset_canvas(Position::new(0, 1)),
                Action::CanvasDown => self.draw.offset_canvas(Position::new(0, -1)),

                Action::NextXPixel => { self.draw.jump_cursor(Direction::Right); }
                Action::PrevXPixel => { self.draw.jump_cursor(Direction::Left); }
                Action::NextYPixel => { self.draw.jump_cursor(Direction::Down); }
                Action::PrevYPixel => { self.draw.jump_cursor(Direction::Up); }

                Action::Noop => {}
            }
        }
    }

    pub fn resize(&mut self, new_size: Size<i32>) {
        self.application_viewport.resize(new_size);
        // self.canvas_viewport.reszie(new_size);
    }

    pub fn render(&mut self, context: &mut Context) {
        // Borders and everything but the drawable area
        let res = self.application_renderer.render(
            &self.border.texture,
            &self.border.vertex_data,
            &self.application_viewport,
            context,
        );

        if let Err(e) = res {
            error!("Failed to render the application: {:?}", e);
        }

        // Render the drawable area
        self.draw.render(&self.canvas_viewport, context);
    }

    // Commands coming from the command line
    pub fn exec(&mut self, command: Command, context: &mut Context) -> Result<()> {
        match command {
            Command::Noop | Command::Quit => {}
            Command::Save { path, overwrite } => self.draw.write_to_disk(path, overwrite, context),
            Command::Extend(ext) => self.draw.resize_canvas(ext, context)?,
            Command::Put { x, y } => self.draw.put_pixel(Position::new(x, y)),
            _ => unimplemented!(),
        }

        Ok(())
    }

    pub fn cur_pos(&self) -> Position<i32> {
        self.draw.cursor.position
    }
}

impl Listener for Canvas {
}