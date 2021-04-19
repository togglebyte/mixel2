use anyhow::Result;
use log::error;
use nightmaregl::events::{Key, Modifiers};
use nightmaregl::{Context, Size};

use crate::canvas::Canvas;
use crate::commandline::{Command, CommandLine};
use crate::config::Config;

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
    config: Config,
}

impl App {
    pub fn new(config: Config, window_size: Size<i32>, context: &mut Context) -> Result<Self> {
        let inst = Self {
            canvas: Canvas::new(window_size, context)?,
            command_line: CommandLine::new(window_size, context)?,
            window_size,
            mode: Mode::Normal,
            close: false,
            config,
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

    pub fn input(&mut self, key: Key, modifiers: Modifiers, context: &mut Context) {
        match (self.mode, key) {
            (Mode::Normal, Key::Colon) => self.mode = Mode::Command,
            (Mode::Insert, Key::Escape) => self.mode = Mode::Normal,
            (Mode::Visual, Key::Escape) => self.mode = Mode::Normal,
            (Mode::Command, Key::Escape) => self.mode = Mode::Normal,
            (Mode::Normal, Key::I) if modifiers.is_empty() => self.mode = Mode::Insert,
            (Mode::Normal, Key::V) if modifiers.is_empty() => self.mode = Mode::Visual,
            _ => {}
        }

        match self.mode {
            Mode::Command => {
                match self.command_line.input(key) {
                    Some(Command::Quit) => self.close = true,
                    Some(command) => self.canvas.exec(command, context),
                    None => {}
                }

                if let Key::Return = key {
                    self.mode = Mode::Normal;
                }

                if let Key::Back = key {
                    if self.command_line.is_empty() {
                        self.mode = Mode::Normal;
                    }
                }
            }
            _ => {
                let action = self.config.key_map(key, modifiers);
                self.canvas.input(action);
            }
        }
    }

    pub fn render(&mut self, context: &mut Context) {
        if let Mode::Command = self.mode {
            self.command_line.render(context);
        }
        self.canvas.render(context);
    }
}
