use anyhow::Result;
use nightmaregl::events::Key;
use nightmaregl::texture::{Texture, Wrap};
use nightmaregl::{Context, Pixel, Position, Renderer, Size, Sprite, VertexData, Viewport};

mod pixelbuffer;
mod border;

use pixelbuffer::PixelBuffer;
use border::Border;

// -----------------------------------------------------------------------------
//     - Layers -
// -----------------------------------------------------------------------------
struct Layer {
    texture: Texture<i32>,
    buffer: PixelBuffer,
}

impl Layer {
    pub fn new(size: Size<i32>) -> Self {
        let buffer = PixelBuffer::new(Pixel::transparent(), size.cast());
        let texture = Texture::default_with_data(size, buffer.0.as_bytes());
        Self { texture, buffer }
    }
}

// -----------------------------------------------------------------------------
//     - Canvas -
// -----------------------------------------------------------------------------
pub struct Canvas {
    background: Texture<i32>,
    border: Border,
    application_renderer: Renderer<VertexData>,
    application_viewport: Viewport,
    draw_area_sprite: Sprite<i32>,
    layers: Vec<Layer>,
    canvas_viewport: Viewport,
    current_layer: usize,
    pixel_renderer: Renderer<VertexData>,
}

impl Canvas {
    pub fn new(window_size: Size<i32>, context: &mut Context) -> Result<Self> {
        let mut application_renderer = Renderer::default(context)?;
        application_renderer.pixel_size = 2;
        let application_viewport = Viewport::new(Position::zero(), window_size);

        let mut pixel_renderer = Renderer::default(context)?;
        pixel_renderer.pixel_size = 8;
        
        let canvas_viewport = {
            let padding = 128 / application_renderer.pixel_size as i32;
            let pos = application_viewport.position + Position::new(padding, padding);
            let size = *application_viewport.size() - Size::new(padding * 2, padding * 2);
            
            Viewport::new(pos, size)
        };

        let border = Border::new(canvas_viewport.position, *canvas_viewport.size(), application_renderer.pixel_size)?;

        let size = Size::new(32, 32);
        let layer = Layer::new(size);

        let background = Texture::from_disk("background.png")?;

        let mut draw_area_sprite = Sprite::new(&background);
        draw_area_sprite.position = canvas_viewport.centre();
        draw_area_sprite.anchor -= (draw_area_sprite.size / 2).into();

        let inst = Self {
            application_viewport,
            application_renderer,
            background,
            border,
            draw_area_sprite,
            layers: vec![layer],
            canvas_viewport,
            current_layer: 0,
            pixel_renderer,
        };

        Ok(inst)
    }

    pub fn input(&mut self, key: Key) {
        match key {
            Key::H => self.draw_area_sprite.position.x -= 1 * self.pixel_renderer.pixel_size as i32,
            Key::L => self.draw_area_sprite.position.x += 1 * self.pixel_renderer.pixel_size as i32,
            Key::K => self.draw_area_sprite.position.y += 1 * self.pixel_renderer.pixel_size as i32,
            Key::J => self.draw_area_sprite.position.y -= 1 * self.pixel_renderer.pixel_size as i32,
            Key::A => self.pixel_renderer.pixel_size += 1,
            Key::S => self.pixel_renderer.pixel_size -= 1,
            _ => {}
        }
    }

    pub fn resize(&mut self, new_size: Size<i32>) {
        self.application_viewport.resize(new_size);
        // TODO: resize all layer viewports as well.
        // Move the sprite into the centre of the screen
    }

    pub fn render(&mut self, context: &mut Context) {
        // Use the same vertex data for the canvas and
        // the layers.

        // Canvas / Drawable area
        let vertex_data = [self
            .draw_area_sprite
            .vertex_data_scaled(self.pixel_renderer.pixel_size as f32)];

        self.pixel_renderer
            .render(&self.background, &vertex_data, &self.canvas_viewport, context);

        // Layers
        let errors = self
            .layers
            .iter()
            .map(|l| {
                self.pixel_renderer
                    .render(&l.texture, &vertex_data, &self.canvas_viewport, context)
            })
            .collect::<Vec<_>>();

        // Borders
        self.application_renderer.render(
            &self.border.texture,
            &self.border.vertex_data,
            &self.application_viewport,
            context,
        );

    }
}
