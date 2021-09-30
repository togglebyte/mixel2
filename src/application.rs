use std::path::PathBuf;
use std::collections::VecDeque;

use anyhow::Result;
use nightmare::events::{Key, Modifiers};
use nightmare::{Viewport, Context, Size, Position};
use nightmare::texture::Texture;
use nightmare::pixels::Pixel;
use nightmare::render2d::{SimpleRenderer, Model};

use crate::border::{BorderType, Textures};
use crate::canvas::Canvas;
use crate::commandline::{Command, CommandLine};
use crate::config::Config;
use crate::console::Console;
use crate::input::{InputToAction, Input};
use crate::listener::{MessageCtx, Listener};
use crate::message::Message;
use crate::mouse::MouseCursor;
use crate::status::Status;

const VIEWPORT_PADDING:i32 = 128;

fn canvas_viewport(viewport: &Viewport) -> Viewport {
    let pad = VIEWPORT_PADDING;
    Viewport::new(
        Position::new(pad, pad),
        *viewport.size() - Size::new(pad * 2, pad * 2),
    )
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
    win_size: Size,
    config: Config,
    listeners: Vec<Box<dyn Listener>>,
    app_viewport: Viewport,
    canvas_viewport: Viewport,
    textures: Textures,
    renderer: SimpleRenderer<Model>,
}

impl App {
    pub fn new(config: Config, win_size: Size, context: &mut Context) -> Result<Self> {
        let app_viewport = Viewport::new(
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
        let canvas_viewport = canvas_viewport(&app_viewport);
        let renderer = SimpleRenderer::new(context, app_viewport.view_projections())?;

        let mut inst = Self {
            win_size,
            mode: Mode::Normal,
            close: false,
            config,
            listeners: vec![],
            app_viewport,
            canvas_viewport,
            textures,
            renderer,
        };

        let mut ctx = MessageCtx { 
            config: &inst.config,
            canvas_viewport: &inst.canvas_viewport,
            app_viewport: &inst.app_viewport,
            textures: &inst.textures,
            renderer: &inst.renderer,
            context,
        };

        inst.listeners.push(Box::new(Canvas::new(inst.canvas_viewport.clone(), &mut ctx)?));
        inst.listeners.push(Box::new(Status::new(win_size, ctx.context)?));
        inst.listeners.push(Box::new(CommandLine::new(win_size, ctx.context)?));
        inst.listeners.push(Box::new(MouseCursor::new(&mut ctx)?));
        inst.listeners.push(Box::new(InputToAction::new(inst.mode)));
        inst.listeners.push(Box::new(Console::new(&mut ctx)?));

        Ok(inst)
    }

    pub fn resize(&mut self, new_size: Size, context: &mut Context) {
        self.win_size = new_size;
        self.app_viewport.resize(new_size);
        self.canvas_viewport = canvas_viewport(&self.app_viewport);
        self.handle_messages(Message::Resize(new_size), context);
    }

    pub fn input(
        &mut self,
        mut input: Input,
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

        if let Input::Mouse(ref mut mouse) = input {
            // Flip the y coords
            let max_y = self.app_viewport.size().height;
            mouse.pos.y = max_y - mouse.pos.y;
            // self.handle_messages(Message::Mouse(mouse), context);
        }

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
            canvas_viewport: &self.canvas_viewport,
            app_viewport: &self.app_viewport,
            textures: &self.textures,
            border_renderer: &self.border_renderer,
            context,
        };

        self.listeners.iter_mut().for_each(|l| {
            l.render(&mut ctx);
        });
    }

    pub fn reload_plugins(&mut self, path: PathBuf, context: &mut Context) {
        // Window size needs to be known at `message handling`
        self.handle_messages(Message::ReloadPlugin(path), context);
        
    }

    fn handle_messages(&mut self, m: Message, context: &mut Context) {
        let mut messages = VecDeque::new();
        messages.push_back(m);

        let mut ctx = MessageCtx { 
            config: &self.config,
            canvas_viewport: &self.canvas_viewport,
            app_viewport: &self.app_viewport,
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
