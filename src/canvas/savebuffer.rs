use std::path::Path;

use anyhow::Result;
use log::error;
use nightmaregl::texture::{Format, Texture};
use nightmaregl::{Context, Position, Renderer, Size, Sprite, VertexData, Viewport, Transform};
use nightmaregl::framebuffer::Framebuffer;
use nightmaregl::pixels::Pixel;

use super::layer::Layer;
use super::Image;

pub struct SaveBuffer {
    renderer: Renderer<VertexData>,
    viewport: Viewport,
}

impl SaveBuffer {
    pub fn new(context: &mut Context, size: Size<i32>) -> Result<Self> {
        let renderer = Renderer::default(context)?;
        let inst = Self {
            renderer,
            viewport: Viewport::new(Position::zero(), size),
        };

        Ok(inst)
    }

    pub fn save(
        &mut self,
        path: impl AsRef<Path>,
        image: &Image,
        size: Size<i32>,
        context: &mut Context,
    ) -> Result<()> {
        self.viewport.resize(size);
        self.viewport.swap_y();

        let mut fb = Framebuffer::default();

        let texture = Texture::<i32>::new()
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
