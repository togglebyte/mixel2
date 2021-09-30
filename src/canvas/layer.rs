use nightmare::{Context, Size, Viewport};
use nightmare::texture::Texture;
use nightmare::pixels::{Pixel, Pixels};
use nightmare::render2d::{SimpleRenderer, Model};
use nightmare::shaders::ShaderProgram;
use crate::Coords;

// -----------------------------------------------------------------------------
//     - Layer id -
// -----------------------------------------------------------------------------
/// LayerId wraps an index.
/// Index 0 is displayed as 1
#[derive(Debug, Copy, Clone)]
pub struct LayerId(usize);

impl LayerId {
    pub fn display(&self) -> usize {
        self.0 + 1
    }

    pub fn from_display(id: usize) -> Self {
        Self(id - 1)
    }

    pub fn from_index(id: usize) -> Self  {
        Self(id)
    }

    pub fn as_index(&self) -> usize {
        self.0
    }

    pub fn as_display(&self) -> usize {
        self.0 + 1
    }
}

// -----------------------------------------------------------------------------
//     - Layers -
// -----------------------------------------------------------------------------
pub struct Layer {
    pub texture: Texture,
    pub buffer: Pixels<Pixel>,
    pub(super) dirty: bool,
    pub shader: Option<ShaderProgram>,
    pub(super) renderer: SimpleRenderer<Model>,
}

impl Layer {
    pub fn new(size: Size, context: &mut Context, viewport: &Viewport) -> Self {
        let buffer = Pixels::from_pixel(Pixel::transparent(), size.cast());
        let texture = Texture::default_with_data(size.cast(), buffer.as_bytes());
        let renderer = SimpleRenderer::new(context, viewport.view_projection());
        Self { texture, buffer, dirty: false }
    }

    pub fn push_pixel(&mut self, pixel: Pixel, coords: Coords) {
        if coords.0.x < 0 || coords.0.y < 0 {
            return;
        }

        let size = self.buffer.size().cast();

        if coords.0.x >= size.width || coords.0.y >= size.height {
            return
        }

        self.buffer.insert_pixel(coords.0.cast(), pixel);
        self.dirty = true;
    }

    pub fn resize(&mut self, new_size: Size) {
        drop(new_size);
        todo!("oh no you don't!");
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|p| *p = Pixel::transparent());
        self.dirty = true;
    }

    // TODO: only draw the dirty region
    pub fn draw_to_texture(&mut self) {
        self.texture.write_region(
            nightmare::Position::zero(),
            self.buffer.size().cast(),
            self.buffer.as_bytes(),
        );

        self.dirty = false;
    }
}

