use log::error;
use nightmaregl::{Context, Size};
use nightmaregl::events::Key;
use anyhow::Result;

use crate::canvas::Canvas;
use crate::commandline::{Command, CommandLine};

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
    pub close: bool,
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
            close: false,
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
            self.command_line.input_char(c);
        }
    }

    pub fn input(&mut self, key: Key, context: &mut Context) {
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
            Mode::Command => {
                match self.command_line.input(key) {
                    Some(Command::Quit) => self.close = true,
                    Some(command) => self.canvas.exec(command, context),
                    None => {}
                }
            }
            _ => self.canvas.input(key),
        }
    }

    pub fn render(&mut self, context: &mut Context) {
        if let Mode::Command = self.mode {
            self.command_line.render(context);
        }
        self.canvas.render(context);
    }
}
