use anyhow::Result;
use log::error;
use nightmaregl::events::{Key, Modifiers};
use nightmaregl::{Context, Size, Position};

use crate::canvas::Canvas;
use crate::commandline::CommandLine;
use crate::commandline::commands::Command;
use crate::config::Config;
use crate::input::Input;
use crate::status::Status;

// -----------------------------------------------------------------------------
//     - Mode -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq)]
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
    status: Status, 
}

impl App {
    pub fn new(config: Config, window_size: Size<i32>, context: &mut Context) -> Result<Self> {
        let mut status = Status::new(window_size, context)?;

        let inst = Self {
            canvas: Canvas::new(window_size, context)?,
            command_line: CommandLine::new(window_size, context)?,
            window_size,
            mode: Mode::Normal,
            close: false,
            config,
            status,
        };

        Ok(inst)
    }

    pub fn resize(&mut self, new_size: Size<i32>) {
        self.window_size = new_size;
        self.canvas.resize(new_size);
        // self.command_line.resize(new_size);
    }

    pub fn input(&mut self, input: Input, modifiers: Modifiers, context: &mut Context) -> Result<()> {
        match (self.mode, input) {
            (Mode::Normal, Input::Char(':')) => self.mode = Mode::Command,
            (Mode::Insert, Input::Key(Key::Escape)) => self.mode = Mode::Normal,
            (Mode::Visual, Input::Key(Key::Escape)) => self.mode = Mode::Normal,
            (Mode::Command, Input::Key(Key::Escape)) => self.mode = Mode::Normal,
            (Mode::Normal, Input::Char('i')) if modifiers.is_empty() => self.mode = Mode::Insert,
            (Mode::Visual, Input::Char('i')) if modifiers.is_empty() => self.mode = Mode::Insert,
            (Mode::Normal, Input::Char('v')) if modifiers.is_empty() => self.mode = Mode::Visual,
            _ => {}
        }

        match self.mode {
            Mode::Command => {
                match self.command_line.input(input) {
                    Some(Command::Quit) => self.close = true,
                    Some(command) => self.canvas.exec(command, context)?,
                    None => {}
                }

                if let Input::Key(Key::Return) = input {
                    self.mode = Mode::Normal;
                }

                if let Input::Key(Key::Back) = input {
                    if self.command_line.is_empty() {
                        self.mode = Mode::Normal;
                    }
                }
            }
            _ => {
                let action = self.config.key_map(input, modifiers);
                self.canvas.input(action);
            }
        }

        Ok(())
    }

    pub fn render(&mut self, context: &mut Context) {
        if let Mode::Command = self.mode {
            self.command_line.render(context);
        }

        self.canvas.render(context);
        self.status.set_mode(self.mode);
        self.status.set_cur_pos(self.canvas.cur_pos());
        self.status.render(context);
        
    }
}
