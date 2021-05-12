use std::path::Path;

use anyhow::Result;
use log::error;
use nalgebra::Vector3;
use nightmaregl::framebuffer::{Framebuffer, FramebufferTarget};
use nightmaregl::pixels::{Pixel, Pixels};
use nightmaregl::texture::Texture;
use nightmaregl::{
    Context, FillMode, Position, Rect, Renderer, Size, Sprite, VertexData, Viewport,
};

// use crate::commandline::commands::Extent;
use crate::listener::{MessageCtx, Listener};
use crate::Message;

// use super::cursor::Cursor;
// use super::savebuffer::SaveBuffer;

// pub use super::layer::Layer;

pub mod message;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// -----------------------------------------------------------------------------
//     - Canvas -
// -----------------------------------------------------------------------------
pub struct Canvas {
    size: Size<i32>,
    background: Texture<i32>,
    // pub cursor: Cursor,
    sprite: Sprite<i32>,
    current_layer: usize,
    // layers: Vec<Layer>,
    renderer: Renderer<VertexData>,
    // save_buffer: SaveBuffer,
}

impl Canvas {
    pub fn new(size: Size<i32>, context: &mut Context, position: Position<i32>) -> Result<Self> {

        // // -----------------------------------------------------------------------------
        // //     - Cursor -
        // // -----------------------------------------------------------------------------
        // let cursor = Cursor::new(Position::new(size.width / 2, size.height / 2));

        // -----------------------------------------------------------------------------
        //     - Renderer -
        // -----------------------------------------------------------------------------
        let mut renderer = Renderer::default(context)?;
        renderer.pixel_size = 22;

        // -----------------------------------------------------------------------------
        //     - Drawable area (sprite) -
        // -----------------------------------------------------------------------------
        let mut sprite = Sprite::new(&background);
        sprite.position = position;
        sprite.fill = FillMode::Repeat;
        sprite.anchor -= (sprite.size / 2).into();

        // -----------------------------------------------------------------------------
        //     - Default layer -
        // -----------------------------------------------------------------------------
        // let layer = Layer::new(size);

        // -----------------------------------------------------------------------------
        //     - Instance -
        // -----------------------------------------------------------------------------
        let inst = Self {
            size,
            background,
            // cursor,
            sprite,
            current_layer: 0,
            // layers: vec![layer],
            renderer,
        };

        Ok(inst)
    }

    // pub fn new_layer(&mut self) {
    //     let layer = Layer::new(self.size);
    //     self.current_layer = self.layers.len();
    //     self.layers.push(layer);
    // }

    pub fn render(&self, viewport: &Viewport, context: &mut Context) {
        let pixel_size = self.renderer.pixel_size as f32;

        // Canvas / Drawable area
        let mut vertex_data = self.sprite.vertex_data_scaled(pixel_size);

        let res = self
            .renderer
            .render(&self.background, &[vertex_data], viewport, context);

        if let Err(e) = res {
            error!("Failed to render the background: {:?}", e);
        }

        // Decrease the z_index,
        vertex_data
            .model
            .append_translation_mut(&Vector3::from([0.0, 0.0, -1.0]));

        // Layers
        // self.layers
        //     .iter()
        //     .enumerate()
        //     .map(|(z_index, layer)| {
        //         let z_index = z_index as f32;
        //         vertex_data
        //             .model
        //             .append_translation_mut(&Vector3::from([0.0, 0.0, -z_index]));

        //         self.renderer
        //             .render(&layer.texture, &[vertex_data], viewport, context)
        //     })
        //     .filter(Result::is_err)
        //     .for_each(|e| error!("Failed to render layer: {:?}", e));

        let x = vertex_data.model[(0, 3)];
        let y = vertex_data.model[(1, 3)];

        // let mut cursor_vd = self.cursor.sprite.vertex_data_scaled(pixel_size);
        // cursor_vd.model[(0, 3)] = x + self.cursor.position.x as f32;
        // cursor_vd.model[(1, 3)] = y + (self.size.height - 1 - self.cursor.position.y) as f32;

        // // Cursor
        // let res = self
        //     .renderer
        //     .render(&self.cursor.texture, &[cursor_vd], &viewport, context);

        // if let Err(e) = res {
        //     error!("Failed to render the cursor: {:?}", e);
        // }
    }

    // pub fn draw(&mut self) {
    //     self.put_pixel(self.cursor.position)
    // }

    // pub fn put_pixel(&mut self, position: Position<i32>) {
    //     let mut layer = &mut self.layers[self.current_layer];

    //     layer
    //         .buffer
    //         .insert_pixel(self.cursor.color, position.cast());

    //     layer.texture.write_region(
    //         Position::zero(),
    //         layer.buffer.size().cast(),
    //         layer.buffer.as_bytes(),
    //     );
    // }

    // pub fn offset_cursor(&mut self, offset: Position<i32>) {
    //     let p = (self.cursor.position + offset).to_point();
    //     let rect = Rect::from_size(self.sprite.size);
    //     if rect.contains(p) {
    //         self.cursor.position += offset;
    //     }
    // }

    pub fn offset_canvas(&mut self, offset: Position<i32>) {
        self.sprite.position += offset * 32;
    }

    pub fn resize_pixel(&mut self, size: i32) {
        self.renderer.pixel_size += size;
    }

    // pub fn write_to_disk(
    //     &mut self,
    //     path: impl AsRef<Path>,
    //     overwrite: bool,
    //     context: &mut Context,
    // ) {
    //     if !overwrite && path.as_ref().exists() {
    //         error!("File exists. Use ! to overwrite");
    //         return;
    //     }

    //     self.save_buffer
    //         .save(path, &self.sprite, &self.layers, context);
    // }

    // pub fn resize_canvas(&mut self, extent: Extent, context: &mut Context) -> Result<()> {
    //     let new_size = Size::new(48, 32);

    //     // self.sprite = sprite;
    //     self.sprite.size = new_size;
    //     let vertex_data = [self.sprite.vertex_data()];
    //     let viewport = Viewport::new(Position::zero(), new_size);

    //     // Setup renderer
    //     let renderer = Renderer::default(context)?;

    //     for layer in &mut self.layers {
    //         // Read from
    //         let mut read_buffer = Framebuffer::new(FramebufferTarget::Read);
    //         read_buffer.attach_texture(&layer.texture);
    //         read_buffer.bind();

    //         // Write to
    //         let mut write_buffer = Framebuffer::new(FramebufferTarget::Draw);
    //         let pixels = Pixels::from_pixel(Pixel::transparent(), new_size.cast());
    //         let texture = Texture::default_with_data(new_size, pixels.as_bytes());
    //         write_buffer.attach_texture(&texture);
    //         write_buffer.bind();

    //         // Blit
    //         renderer.render(&texture, &vertex_data, &viewport, context);
    //     }

    //     Ok(())
    // }

    // pub fn jump_cursor(&mut self, direction: Direction) -> Option<()> {
    //     let layer = &self.layers[self.current_layer];
    //     let (pos, size) = {
    //         let size = self.sprite.size;
    //         let pos = self.cursor.position;

    //         match direction {
    //             Direction::Left => (
    //                 Position::new(0, pos.y),
    //                 Size::new(pos.x, 1),
    //             ),
    //             Direction::Right => (
    //                 Position::new(pos.x, pos.y),
    //                 Size::new(size.width - pos.x, 1)
    //             ),
    //             Direction::Up => (
    //                 Position::new(pos.x, 0),
    //                 Size::new(1, pos.y),
    //             ),
    //             Direction::Down => (
    //                 Position::new(pos.x, pos.y),
    //                 Size::new(1, size.height - pos.y),
    //             )
    //         }
    //     };

    //     let region = layer.buffer.region(pos.cast(), size.cast());

    //     match direction {
    //         Direction::Right => {
    //             let row = region.rows().next().unwrap();
    //             let steps = row.into_iter().position(|p| p.a > 0).unwrap_or(1) - 1;
    //             self.cursor.position.x += steps as i32;
    //         }
    //         Direction::Left => {
    //             let row = region.rows().next().unwrap();
    //             let steps = row.into_iter().rev().position(|p| p.a > 0).unwrap_or(0);
    //             self.cursor.position.x -= steps as i32;
    //         }
    //         Direction::Up => {
    //             let steps = region.rows().rev().position(|c| c.first().unwrap().a > 0).unwrap_or(0);
    //             self.cursor.position.y -= steps as i32;
    //         }
    //         Direction::Down => {
    //             let steps = region.rows().position(|c| c.first().unwrap().a > 0).unwrap_or(1) - 1;
    //             self.cursor.position.y += steps as i32;
    //         }
    //     }

    //     None
    // }
}

impl Listener for Canvas {
    fn message(&mut self, message: &Message, ctx: &mut MessageCtx) -> Message {
        Message::Noop
    }

    fn render(&mut self, context: &mut Context) -> Result<()> {
        let pixel_size = self.renderer.pixel_size as f32;

        // Canvas / Drawable area
        let mut vertex_data = self.sprite.vertex_data_scaled(pixel_size);

        let res = self
            .renderer
            .render(&self.background, &[vertex_data], viewport, context);

        if let Err(e) = res {
            error!("Failed to render the background: {:?}", e);
        }

        // Decrease the z_index,
        vertex_data
            .model
            .append_translation_mut(&Vector3::from([0.0, 0.0, -1.0]));
        Ok(())
    }
}