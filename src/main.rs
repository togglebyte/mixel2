use pretty_env_logger;
use nightmaregl::events::{Event, KeyState, LoopAction, Modifiers};
use nightmaregl::{Pixel, Context};
use anyhow::Result;

mod application;
mod canvas;
mod commandline;
mod config;

use application::App;
use config::Config;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let config = Config::from_path("config")?;

    let (eventloop, mut context) = Context::builder("Mixel: the modal pixel editor")
        .vsync(false)
        .build()?;

    let window_size = context.window_size();
    let mut app = App::new(config, window_size, &mut context)?;

    let mut modifiers = Modifiers::empty();
    eventloop.run(move |event| {
        match event {
            Event::Char(c) => app.input_char(c),
            Event::Modifier(m) => modifiers = m,
            Event::Key { key, state: KeyState::Pressed } => { 
                app.input(key, modifiers, &mut context);
                if app.close {
                    return LoopAction::Quit;
                }
            }
            Event::Draw(_dt) => {
                // Clear the background with Nypsiee blue
                context.clear(Pixel { r: 12, g: 34, b: 56, a: 255 }.into());
                app.render(&mut context);
                context.swap_buffers();
            }
            Event::Resize(new_size) => app.resize(new_size.cast()),
            _ => {}
        }

        LoopAction::Continue
    });
}
