use nightmaregl::{Context, Size};
use nightmaregl::events::Key;
use anyhow::Result;

use crate::canvas::Canvas;
use crate::commandline::CommandLine;

pub struct App {
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
        };

        Ok(inst)
    }

    pub fn resize(&mut self, new_size: Size<i32>) {
        self.window_size = new_size;
        self.canvas.resize(new_size);
        // self.command_line.resizeknew_size);
    }

    pub fn input_char(&mut self, c: char) {
        // self.command_line.input_char(c);
    }

    pub fn input(&mut self, key: Key) {
        // self.command_line.input(key);
        self.canvas.input(key);
    }

    pub fn render(&mut self, context: &mut Context) {
        self.command_line.render(context);
        self.canvas.render(context);
    }
}
