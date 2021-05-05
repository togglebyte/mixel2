use std::collections::VecDeque;
use anyhow::Result;
use nightmaregl::events::{Key, Modifiers};
use nightmaregl::{Context as GlContext, Size};

use crate::border::Border;
use crate::commandline::{Command, CommandLine};
use crate::config::Config;
use crate::input::{InputToAction, Input};
use crate::listener::{Message, MessageCtx, Listener};
use crate::status::Status;
// use crate::canvas::Canvas;

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
    window_size: Size<i32>,
    config: Config,
    // action_counter: String,
    listeners: Vec<Box<dyn Listener>>,
}

impl App {
    pub fn new(config: Config, window_size: Size<i32>, context: &mut GlContext) -> Result<Self> {
        let mut inst = Self {
            window_size,
            mode: Mode::Normal,
            close: false,
            config,
            // action_counter: String::new(),
            listeners: vec![],
        };

        inst.listeners.push(Box::new(Status::new(window_size, context)?));
        inst.listeners.push(Box::new(CommandLine::new(window_size, context)?));
        inst.listeners.push(Box::new(InputToAction::new(inst.mode)));
        inst.listeners.push(Box::new(Border::new(window_size, context)?));

        Ok(inst)
    }

    pub fn resize(&mut self, new_size: Size<i32>, context: &mut GlContext) {
        self.window_size = new_size;
        self.handle_messages(Message::Resize(new_size), context);
    }

    pub fn input(
        &mut self,
        input: Input,
        modifiers: Modifiers,
        context: &mut GlContext,
    ) -> Result<()> {
        let mode = match (self.mode, input) {
            (Mode::Insert,  Input::Key(Key::Escape)) => Some(Mode::Normal),
            (Mode::Visual,  Input::Key(Key::Escape)) => Some(Mode::Normal),
            (Mode::Command, Input::Key(Key::Escape)) => Some(Mode::Normal),
            (Mode::Normal,  Input::Char(':')) => Some(Mode::Command),
            (Mode::Normal,  Input::Char('i')) if modifiers.is_empty() => Some(Mode::Insert),
            (Mode::Visual,  Input::Char('i')) if modifiers.is_empty() => Some(Mode::Insert),
            (Mode::Normal,  Input::Char('v')) if modifiers.is_empty() => Some(Mode::Visual),
            _ => None
        };

        if let Some(mode) = mode {
            self.mode = mode;
            self.handle_messages(Message::ModeChanged(mode), context);
        }

        self.handle_messages(Message::Input(input, modifiers), context);

        match (self.mode, input) {
            (Mode::Command, Input::Key(Key::Return)) => {
                self.mode = Mode::Normal;
                self.handle_messages(Message::ModeChanged(self.mode), context);
            }
            _ => {}
        };

        //     _ => {
        //         match input {
        //             Input::Key(Key::Escape) => self.action_counter.clear(),
        //             Input::Char(c @ '0'..='9') => self.action_counter.push(c),
        //             _ => {
        //                 let count = self.action_counter.parse::<u16>().unwrap_or(1);
        //                 self.action_counter.clear();

        //                 // Make input nice again
        //                 let input = match input {
        //                     Input::Char(c) if (c as u8) < 26 => Input::Char((c as u8 + 96) as char),
        //                     _ => input,
        //                 };

        //                 let action = self.config.key_map(input, modifiers);
        //                 self.canvas.input(action, count);
        //             }
        //         }
        //     }
        // }

        Ok(())
    }

    pub fn render(&mut self, context: &mut GlContext) {
        self.listeners.iter_mut().for_each(|l| {
            l.render(context);
        });
    }

    fn handle_messages(&mut self, m: Message, context: &mut GlContext) {
        let ctx = MessageCtx { config : &self.config, context };
        let mut messages = VecDeque::new();
        messages.push_back(m);

        // Quit?
        let close = &mut self.close;

        while let Some(m) = messages.pop_front() {
            for l in self.listeners.iter_mut() {
                match l.message(&m, &ctx) {
                    Message::Noop => {}
                    Message::Command(Command::Quit) => *close = true,
                    Message::Command(Command::NewCanvas(size)) => {
                        // TODO: Message to create a new canvas
                        //       Something needs to track canvasses
                    }
                    msg => messages.push_back(msg),
                }
            }
        }
    }

}
