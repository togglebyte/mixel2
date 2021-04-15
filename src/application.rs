use log::error;
use nightmaregl::{Context, Size};
use nightmaregl::events::Key;
use anyhow::Result;

use crate::canvas::Canvas;
use crate::commandline::CommandLine;

// -----------------------------------------------------------------------------
//     - Mode -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Visual,
}

// -----------------------------------------------------------------------------
//     - App -
// -----------------------------------------------------------------------------
pub struct App {
    mode: Mode,
    canvas: Canvas,
    command_line: CommandLine,
    window_size: Size<i32>,
}

impl App {
    pub fn new(window_size: Size<i32>, context: &mut Context) -> Result<Self> {
        let inst = Self {
            canvas: Canvas::new(window_size, context)?,
            command_line: CommandLine::new(window_size, context)?,
            window_size,
            mode: Mode::Normal,
        };

        Ok(inst)
    }

    pub fn resize(&mut self, new_size: Size<i32>) {
        self.window_size = new_size;
        self.canvas.resize(new_size);
        // self.command_line.resize(new_size);
    }

    pub fn input_char(&mut self, c: char) {
        if let Mode::Command = self.mode {
            if let Err(e) = self.command_line.input_char(c) {
                error!("Command line input failed: {:?}", e);
            }
        }
    }

    pub fn input(&mut self, key: Key) -> Result<()> {
        match (self.mode, key) {
            (Mode::Normal, Key::Colon) => self.mode = Mode::Command,
            (Mode::Insert, Key::Escape) => self.mode = Mode::Normal,
            (Mode::Visual, Key::Escape) => self.mode = Mode::Normal,
            (Mode::Command, Key::Escape) => self.mode = Mode::Normal,
            (Mode::Normal, Key::I) => self.mode = Mode::Insert,
            (Mode::Normal, Key::V) => self.mode = Mode::Visual,
            _ => {}
        }

        match self.mode {
            Mode::Command => self.command_line.input(key)?,
            _ => self.canvas.input(key),
        }

        Ok(())
    }

    pub fn render(&mut self, context: &mut Context) {
        self.command_line.render(context);
        self.canvas.render(context);
    }
}
