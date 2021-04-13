use nightmaregl::events::{Event, KeyState, LoopAction};
use nightmaregl::{Pixel, Context};
use anyhow::Result;

mod application;
mod canvas;

use application::App;

fn main() -> Result<()> {
    let (eventloop, mut context) = Context::builder("Mixel: the modal pixel editor").build()?;

    let window_size = context.window_size();
    let mut app = App::new(window_size, &mut context)?;

    eventloop.run(move |event| {
        match event {
            Event::Char('q') => return LoopAction::Quit,
            Event::Char(c) => app.input_char(c),
            Event::Key { key, state: KeyState::Pressed } => app.input(key),
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
