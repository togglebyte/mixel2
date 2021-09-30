use std::path::Path;

use anyhow::Result;
use log::error;
use nightmare::texture::{Format, Texture};
use nightmare::{Context, Position, Size, Sprite, VertexData, Viewport, Transform};
use nightmare::framebuffer::Framebuffer;
use nightmare::pixels::Pixel;
use nightmare::render2d::{SimpleRenderer, Model};

use super::layer::Layer;
use super::Image;

pub struct SaveBuffer {
    renderer: SimpleRenderer<Model>,
    viewport: Viewport,
}

impl SaveBuffer {
    pub fn new(context: &mut Context, size: Size) -> Result<Self> {
        let viewport = Viewport::new(Position::zero(), size);
        let renderer = SimpleRenderer::new(context, viewport.view_projection())?;
        let inst = Self {
            renderer,
            viewport,
        };

        Ok(inst)
    }

    pub fn save(
        &mut self,
        path: impl AsRef<Path>,
        image: &Image,
        size: Size,
        context: &mut Context,
    ) -> Result<()> {
        self.viewport.resize(size);
        self.viewport.swap_y();

        let mut fb = Framebuffer::default();

        let texture = Texture::new()
            .with_format(Format::Rgba)
            .with_no_data(size);

        fb.attach_texture(&texture);
        fb.bind();

        let sprite = Sprite::from_size(size);

        image.render(
            &self.renderer, 
            sprite,
            &Transform::default(),
            &self.viewport,
            context
        )?;

        if let Err(e) = texture.write_to_disk::<Pixel, _>(path.as_ref()) {
            if let Some(path) = path.as_ref().to_str() {
                error!("Failed to save \"{}\" : {:?}", path, e);
            }
        }

        // Reset y coordinate
        self.viewport.swap_y();

        Ok(())
    }
}
