use std::collections::VecDeque;
use anyhow::Result;
use nightmaregl::events::{Key, Modifiers};
use nightmaregl::{Renderer, VertexData, Viewport, RelativeViewport, Context, Size, Position};
use nightmaregl::texture::Texture;

use crate::commandline::{Command, CommandLine};
use crate::config::Config;
use crate::input::{InputToAction, Input};
use crate::listener::{MessageCtx, Listener};
use crate::message::Message;
use crate::status::Status;
use crate::canvas::Canvas;
use crate::border::{BorderType, Textures};

const VIEWPORT_PADDING:i32 = 128;

const fn pad_pos() -> Position<i32> {
    Position::new(VIEWPORT_PADDING, VIEWPORT_PADDING)
}

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
    win_size: Size<i32>,
    config: Config,
    listeners: Vec<Box<dyn Listener>>,
    viewport: Viewport,
    canvas_viewport: RelativeViewport,
    textures: Textures,
    border_renderer: Renderer<VertexData>,
}

impl App {
    pub fn new(config: Config, win_size: Size<i32>, context: &mut Context) -> Result<Self> {
        let viewport = Viewport::new(
            Position::zero(),
            win_size
        );

        // -----------------------------------------------------------------------------
        //     - Border textures -
        // -----------------------------------------------------------------------------
        let textures = {
            let mut textures = Textures::new();

            let canvas = Texture::from_disk("border-canvas.png")?;
            let active = Texture::from_disk("border-active.png")?;
            let inactive = Texture::from_disk("border-inactive.png")?;

            textures.insert(BorderType::Canvas, canvas);
            textures.insert(BorderType::Active, active);
            textures.insert(BorderType::Inactive, inactive);

            textures
        };

        // -----------------------------------------------------------------------------
        //     - Canvas viewport -
        // -----------------------------------------------------------------------------
        let canvas_viewport = viewport.relative(pad_pos(), pad_pos());

        let mut inst = Self {
            win_size,
            mode: Mode::Normal,
            close: false,
            config,
            listeners: vec![],
            viewport,
            canvas_viewport,
            textures,
            border_renderer: Renderer::default(context)?,
        };

        let mut ctx = MessageCtx { 
            config: &inst.config,
            viewport: &inst.canvas_viewport.viewport(),
            textures: &inst.textures,
            border_renderer: &inst.border_renderer,
            context,
        };

        inst.listeners.push(Box::new(Status::new(win_size, ctx.context)?));
        inst.listeners.push(Box::new(CommandLine::new(win_size, ctx.context)?));
        inst.listeners.push(Box::new(InputToAction::new(inst.mode)));
        inst.listeners.push(Box::new(Canvas::new(*inst.canvas_viewport.viewport(), &mut ctx)?));

        Ok(inst)
    }

    pub fn resize(&mut self, new_size: Size<i32>, context: &mut Context) {
        self.win_size = new_size;
        self.viewport.resize(new_size);
        self.canvas_viewport.resize(&self.viewport);
        self.handle_messages(Message::Resize(new_size), context);
    }

    pub fn input(
        &mut self,
        input: Input,
        modifiers: Modifiers,
        context: &mut Context,
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

        Ok(())
    }

    pub fn render(&mut self, context: &mut Context) {
        let mut ctx = MessageCtx { 
            config: &self.config,
            viewport: &self.canvas_viewport.viewport(),
            textures: &self.textures,
            border_renderer: &self.border_renderer,
            context,
        };

        self.listeners.iter_mut().for_each(|l| {
            l.render(&mut ctx);
        });
    }

    fn handle_messages(&mut self, m: Message, context: &mut Context) {
        let mut messages = VecDeque::new();
        messages.push_back(m);

        let mut ctx = MessageCtx { 
            config: &self.config,
            viewport: &self.canvas_viewport.viewport(),
            textures: &self.textures,
            border_renderer: &self.border_renderer,
            context,
        };

        // Quit?
        let close = &mut self.close;

        while let Some(m) = messages.pop_front() {
            for l in self.listeners.iter_mut() {
                match l.message(&m, &mut ctx) {
                    Message::Noop => {}
                    Message::Command(Command::Quit) => *close = true,
                    msg => messages.push_back(msg),
                }
            }
        }
    }

}
