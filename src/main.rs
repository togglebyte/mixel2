use anyhow::Result;
use log::error;
use nightmaregl::{Context, Size, Position};
use nightmaregl::events::{Event, ButtonState, LoopAction, Modifiers, EventLoop};
use nightmaregl::pixels::Pixel;
use pretty_env_logger;

mod application;
mod border;
mod canvas;
mod commandline;
mod config;
mod input;
mod listener;
mod message;
mod node;
mod status;

use application::App;
use config::Config;
use input::Input;
use message::Message;
pub use node::Node;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let config = Config::from_path("config")?;

    let (el, mut context) = Context::builder("Mixel: the modal pixel editor")
        .vsync(false)
        .resizable(false)
        .with_size(Size::new(1880, 1024))
        .build()?;

    let eventloop = EventLoop::new(el);

    let window_size = context.window_size();
    let mut app = App::new(config, window_size, &mut context)?;

    let mut modifiers = Modifiers::empty();
    eventloop.run(move |event| {
        match event {
            Event::MouseMoved { x, y } => {
                app.input(Input::mouse(x, y), modifiers, &mut context);
            }
            Event::Modifier(m) => modifiers = m,
            Event::Char(c) => {
                if let Err(e) = app.input(Input::from_char(c), modifiers, &mut context) {
                    error!("Failed to handle input: {:?}", e);
                }
            }
            Event::Key {
                key,
                state: ButtonState::Pressed,
            } => {
                if let Some(input) = Input::from_key(key) {
                    if let Err(e) = app.input(input, modifiers, &mut context) {
                        error!("Failed to handle input: {:?}", e);
                    }

                    if app.close {
                        return LoopAction::Quit;
                    }
                }
            }
            Event::Draw(_dt) => {
                // Clear the background with Nypsiee blue
                context.clear(
                    Pixel {
                        r: 12,
                        g: 34,
                        b: 56,
                        a: 255,
                    }
                    .into(),
                );
                app.render(&mut context);
                context.swap_buffers();
            }
            Event::Resize(new_size) => app.resize(new_size.cast(), &mut context),
            _ => {}
        }

        LoopAction::Continue
    });
}
