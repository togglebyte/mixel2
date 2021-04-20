use std::path::Path;

use log::error;
use anyhow::Result;
use nalgebra::Vector3;
use nightmaregl::texture::Texture;
use nightmaregl::{Context, Pixel, Pixels, Position, Renderer, Size, Sprite, FillMode, VertexData, Viewport};
use nightmaregl::framebuffer::{Framebuffer, FramebufferTarget};

use crate::commandline::commands::Extent;

use super::cursor::Cursor;
use super::pixelbuffer::PixelBuffer;
use super::savebuffer::SaveBuffer;

// -----------------------------------------------------------------------------
//     - Layers -
// -----------------------------------------------------------------------------
pub struct Layer {
    pub texture: Texture<i32>,
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
//     - Draw -
// -----------------------------------------------------------------------------
pub struct Draw {
    size: Size<i32>,
    background: Texture<i32>,
    cursor: Cursor,
    sprite: Sprite<i32>,
    current_layer: usize,
    layers: Vec<Layer>,
    renderer: Renderer<VertexData>,
    cursor_pos: Position<i32>,
    save_buffer: SaveBuffer,
}

impl Draw {
    pub fn new(size: Size<i32>, context: &mut Context, position: Position<i32>) -> Result<Self> {
        let background = Texture::from_disk("background.png")?;

        // -----------------------------------------------------------------------------
        //     - Cursor -
        // -----------------------------------------------------------------------------
        let cursor = Cursor::new();

        // -----------------------------------------------------------------------------
        //     - Renderer -
        // -----------------------------------------------------------------------------
        let mut renderer = Renderer::default(context)?;
        renderer.pixel_size = 22;

        // -----------------------------------------------------------------------------
        //     - Drawable area (sprite) -
        // -----------------------------------------------------------------------------
        // let position = (position / renderer.pixel_size).into();
        let mut sprite = Sprite::new(&background);
        sprite.size = size;
        sprite.position = position;
        sprite.fill = FillMode::Repeat;
        sprite.anchor -= (sprite.size / 2).into();

        // -----------------------------------------------------------------------------
        //     - Default layer -
        // -----------------------------------------------------------------------------
        let layer = Layer::new(size);

        // -----------------------------------------------------------------------------
        //     - Instance -
        // -----------------------------------------------------------------------------
        let inst = Self {
            cursor_pos: Position::new(0, 0),
            size,
            background,
            cursor,
            sprite,
            current_layer: 0,
            layers: vec![layer],
            renderer,
            save_buffer: SaveBuffer::new(context)?
        };

        Ok(inst)
    }

    pub fn new_layer(&mut self) {
        let layer = Layer::new(self.size);
        self.current_layer = self.layers.len();
        self.layers.push(layer);
    }

    pub fn render(&self, viewport: &Viewport, context: &mut Context) {
        let pixel_size = self.renderer.pixel_size as f32;

        // Canvas / Drawable area
        let mut vertex_data = self.sprite.vertex_data_scaled(pixel_size);

        let res = self.renderer.render(
            &self.background,
            &[vertex_data],
            viewport,
            context,
        );

        if let Err(e) = res {
            error!("Failed to render the background: {:?}", e);
        }

        // Decrease the z_index, 
        vertex_data
            .model
            .append_translation_mut(&Vector3::from([0.0, 0.0, -1.0]));

        // Layers
        self
            .layers
            .iter()
            .enumerate()
            .map(|(z_index, layer)| {
                let z_index = z_index as f32;
                vertex_data
                    .model
                    .append_translation_mut(&Vector3::from([0.0, 0.0, -z_index]));

                self.renderer.render(
                    &layer.texture,
                    &[vertex_data],
                    viewport,
                    context,
                )
            })
            .filter(Result::is_err)
            .for_each(|e| error!("Failed to render layer: {:?}", e));

        let x = vertex_data.model[(0, 3)];
        let y = vertex_data.model[(1, 3)];

        let mut cursor_vd = self.cursor.sprite.vertex_data_scaled(pixel_size);
        cursor_vd.model[(0, 3)] = x + self.cursor_pos.x as f32;
        cursor_vd.model[(1, 3)] = y + (self.size.height - 1 - self.cursor_pos.y) as f32;

        // Cursor
        let res = self.renderer.render(
            &self.cursor.texture,
            &[cursor_vd],
            &viewport,
            context,
        );

        if let Err(e) = res {
            error!("Failed to render the cursor: {:?}", e );
        }
    }

    pub fn draw(&mut self) {
        let position = self.cursor_pos;
        let layer = &self.layers[self.current_layer];
        let pixel = self.cursor.color;
        let size = Size::new(1, 1);
        let pixels = Pixels::from_pixel(pixel, size);
        layer
            .texture
            .write_region(position, size.cast(), pixels.as_bytes());
    }

    pub fn offset_cursor(&mut self, offset: Position<i32>) { 
        self.cursor_pos += offset;
    }

    pub fn offset_canvas(&mut self, offset: Position<i32>) {
        self.sprite.position += offset * 32;
    }

    pub fn resize_pixel(&mut self, size: i32) {
        self.renderer.pixel_size += size;
    }

    pub fn write_to_disk(&mut self, path: impl AsRef<Path>, overwrite: bool, context: &mut Context) {
        if !overwrite && path.as_ref().exists() {
            error!("File exists. Use ! to overwrite");
            return;
        }

        self.save_buffer.save(
            path,
            &self.sprite,
            &self.layers,
            context
        );
    }

    pub fn resize_canvas(&mut self, extent: Extent, context: &mut Context) -> Result<()> {
        // Maybe use framebuffer blitting:
        // https://www.khronos.org/opengl/wiki/Framebuffer#Blitting
        //
        // Create two frame buffers, then blit to copy the 
        // pixels from one texture to another.
        //
        // Create one framebuffer for reading.
        // Attach texture from the layer to that framebuffer.
        //
        // Create another framebuffer for writing
        // that has the new size in a new texture.
        //
        // Resize the sprite.
        //
        // Render... 

        let new_size = Size::new(48, 32);

        // Create a new sprite
        let mut sprite = Sprite::from_size(new_size);
        // sprite.fill = FillMode::Repeat;
        // sprite.position = self.sprite.position;
        // sprite.anchor = self.sprite.anchor;

        // self.sprite = sprite;
        self.sprite.size = new_size;
        let vertex_data = [self.sprite.vertex_data()];
        let viewport = Viewport::new(Position::zero(), new_size);

        // Setup renderer
        let renderer = Renderer::default(context)?;

        for layer in &mut self.layers {
            // Read from
            let mut read_buffer = Framebuffer::new(FramebufferTarget::Read);
            read_buffer.attach_texture(&layer.texture);
            read_buffer.bind();

            // Write to
            let mut write_buffer = Framebuffer::new(FramebufferTarget::Draw);
            let pixels = Pixels::from_pixel(Pixel::transparent(), new_size.cast());
            let texture = Texture::default_with_data(new_size, pixels.as_bytes());
            write_buffer.attach_texture(&texture);
            write_buffer.bind();

            // Blit
            renderer.render(
                &texture,
                &vertex_data,
                &viewport,
                context,
            );
        }

        Ok(())
    }
}


