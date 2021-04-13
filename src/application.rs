use nightmaregl::{Context, Size};
use nightmaregl::events::Key;
use anyhow::Result;

use crate::canvas::Canvas;

pub struct App {
    canvas: Canvas,
    window_size: Size<i32>,
}

impl App {
    pub fn new(window_size: Size<i32>, context: &mut Context) -> Result<Self> {
        let inst = Self {
            canvas: Canvas::new(window_size, context)?,
            window_size,
        };

        Ok(inst)
    }

    pub fn resize(&mut self, new_size: Size<i32>) {
        self.window_size = new_size;
        self.canvas.resize(new_size);
        // self.command_line.resize(new_size);
    }

    pub fn input_char(&self, c: char) {
        // self.command_line.input_char(c);
    }

    pub fn input(&mut self, key: Key) {
        self.canvas.input(key);
    }

    pub fn render(&mut self, context: &mut Context) {
        // self.command_line.render(context)
        self.canvas.render(context);
    }
}
