use std::path::Path;

use anyhow::Result;
use log::error;
use nightmaregl::texture::{Format, Texture};
use nightmaregl::{Context, Framebuffer, Position, Renderer, Size, Sprite, VertexData, Viewport};

use super::draw::Layer;

pub struct SaveBuffer {
    renderer: Renderer<VertexData>,
    viewport: Viewport,
}

impl SaveBuffer {
    pub fn new(context: &mut Context) -> Result<Self> {
        let renderer = Renderer::default(context)?;
        let inst = Self {
            renderer,
            viewport: Viewport::new(Position::zero(), Size::new(32, 32)),
        };

        Ok(inst)
    }

    pub fn save(
        &mut self,
        path: impl AsRef<Path>,
        sprite: &Sprite<i32>,
        layers: &[Layer],
        context: &mut Context,
    ) {
        self.viewport.resize(sprite.size);
        eprintln!("{:?}", "let's do swapsies");
        self.viewport.swap_y();

        let fb = Framebuffer::new();

        let texture = Texture::<i32>::new()
            .with_format(Format::Rgba)
            .with_no_data(sprite.size);

        fb.attach_texture(&texture);
        fb.bind();

        let sprite = Sprite::from_size(Size::new(32, 32));

        let vertex_data = [sprite.vertex_data()];

        layers.into_iter().for_each(|layer| {
            let res = self
                .renderer
                .render(&layer.texture, &vertex_data, &self.viewport, context);

            if let Err(e) = res {
                error!("Failed to save layer: {:?}", e);
            }
        });

        if let Err(e) = texture.write_to_disk(path.as_ref()) {
            if let Some(path) = path.as_ref().to_str() {
                error!("Failed to save \"{}\" : {:?}", path, e);
            }
        }
    }
}
