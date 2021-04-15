use log::error;
use anyhow::Result;
use nightmaregl::events::Key;
use nightmaregl::{Context, Position, Renderer, Size, VertexData, Viewport};

mod border;
mod cursor;
mod pixelbuffer;
mod draw;

use border::Border;
use draw::Draw;

// -----------------------------------------------------------------------------
//     - Canvas -
// -----------------------------------------------------------------------------
pub struct Canvas {
    border: Border,
    application_renderer: Renderer<VertexData>,
    application_viewport: Viewport,
    canvas_viewport: Viewport,
    draw: Draw,
}

impl Canvas {
    pub fn new(window_size: Size<i32>, context: &mut Context) -> Result<Self> {
        let mut application_renderer = Renderer::default(context)?;
        application_renderer.pixel_size = 1;
        let application_viewport = Viewport::new(Position::zero(), window_size);

        // -----------------------------------------------------------------------------
        //     - Canvas viewport -
        // -----------------------------------------------------------------------------
        let canvas_viewport = {
            let padding = 256 / application_renderer.pixel_size;
            let pos = application_viewport.position + Position::new(padding, padding);
            let size = *application_viewport.size() - Size::new(padding * 2, padding * 2);

            Viewport::new(pos, size)
        };

        // -----------------------------------------------------------------------------
        //     - Border -
        // -----------------------------------------------------------------------------
        let border = Border::new(
            canvas_viewport.position,
            *canvas_viewport.size(),
            application_renderer.pixel_size,
        )?;

        // -----------------------------------------------------------------------------
        //     - Draw -
        // -----------------------------------------------------------------------------
        let draw = Draw::new(Size::new(32, 32), context, canvas_viewport.centre())?;

        let inst = Self {
            border,
            application_renderer,
            application_viewport,
            canvas_viewport,
            draw,
        };

        Ok(inst)
    }

    pub fn input(&mut self, key: Key) {
        // let pixel_size = self.pixel_renderer.pixel_size as i32;

        match key {
            // Move the cursor  
            Key::H => self.draw.offset_cursor(Position::new(-1, 0)),
            Key::L => self.draw.offset_cursor(Position::new(1, 0) ),
            Key::K => self.draw.offset_cursor(Position::new(0, -1) ),
            Key::J => self.draw.offset_cursor(Position::new(0, 1)),

            // Draw a pixel
            Key::Space => self.draw.draw(),

            // Move
            Key::Left  => self.draw.offset_canvas(Position::new(-1, 0)),
            Key::Right => self.draw.offset_canvas(Position::new(1, 0)),
            Key::Up    => self.draw.offset_canvas(Position::new(0, 1)),
            Key::Down  => self.draw.offset_canvas(Position::new(0, -1)),

            // Scale up / down the pixel
            Key::A => self.draw.resize_pixel(1),
            Key::S => self.draw.resize_pixel(-1),
            _ => {}
        }
    }

    pub fn resize(&mut self, new_size: Size<i32>) {
        self.application_viewport.resize(new_size);
        // self.canvas_viewport.reszie(new_size);
    }

    pub fn render(&mut self, context: &mut Context) {
        // Borders and everything but the drawable area
        let res = self.application_renderer.render(
            &self.border.texture,
            &self.border.vertex_data,
            &self.application_viewport,
            context,
        );

        if let Err(e) = res { 
            error!("Failed to render the application: {:?}", e);
        }

        // Render the drawable area
        self.draw.render(&self.canvas_viewport, context);
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use nightmaregl::*;

    #[test]
    fn moving_cursor() {
        let viewport = Viewport::new(Position::zero(), Size::new(800, 600));
        let mut canvas = Sprite::from_size(Size::new(32, 32));
        let mut cursor = Sprite::from_size(Size::new(1, 1));
        cursor.position = Position::new(0, 0);
        canvas.position = Position::new(0, 0);

        assert_eq!(expected, actual);
    }
}
