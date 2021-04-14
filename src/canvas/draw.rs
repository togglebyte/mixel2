use anyhow::Result;
use nalgebra::Vector3;
use nightmaregl::events::Key;
use nightmaregl::texture::Texture;
use nightmaregl::{Context, Pixel, Pixels, Position, Renderer, Size, Sprite, VertexData, Viewport};

use super::cursor::Cursor;
use super::pixelbuffer::PixelBuffer;

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
}

impl Draw {
    pub fn new(size: Size<i32>, context: &mut Context, position: Position<i32>) -> Result<Self> {
        let background = Texture::from_disk("background.png")?;

        // -----------------------------------------------------------------------------
        //     - Cursor -
        // -----------------------------------------------------------------------------
        let mut cursor = Cursor::new();

        // cursor.anchor = (cursor.sprite.size / 2).into();

        // -----------------------------------------------------------------------------
        //     - Renderer -
        // -----------------------------------------------------------------------------
        let mut renderer = Renderer::default(context)?;
        renderer.pixel_size = 23;

        // -----------------------------------------------------------------------------
        //     - Drawable area -
        // -----------------------------------------------------------------------------
        let position = (position / renderer.pixel_size).into();
        let mut sprite = Sprite::new(&background);
        sprite.position = position;
        sprite.anchor -= (sprite.size / 2).into();

        let layer = Layer::new(size);

        let inst = Self {
            cursor_pos: Position::new(0, 0),
            size,
            background,
            cursor,
            sprite,
            current_layer: 0,
            layers: vec![layer],
            renderer,
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
        // let mut vertex_data = self.sprite.vertex_data_scaled(pixel_size);
        let mut vertex_data = self.sprite.vertex_data();

        self.renderer.render(
            &self.background,
            &[vertex_data],
            viewport,
            context,
        );

        // Decrease the z_index, 
        vertex_data
            .model
            .append_translation_mut(&Vector3::from([0.0, 0.0, -1.0]));

        // Layers
        let errors = self
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
            .collect::<Vec<_>>();

        let x = vertex_data.model[(0, 3)];
        let y = vertex_data.model[(1, 3)];

        let mut cursor_vd = self.cursor.sprite.vertex_data_scaled(pixel_size);
        cursor_vd.model[(0, 3)] = x + self.cursor_pos.x as f32;
        cursor_vd.model[(1, 3)] = y + (self.size.height - 1 - self.cursor_pos.y) as f32;

        // Cursor
        self.renderer.render(
            &self.cursor.texture,
            &[cursor_vd],
            &viewport,
            context,
        );
    }

    pub fn draw(&mut self) {
        let position = self.cursor_pos;
        let layer = &self.layers[self.current_layer];
        let pixel = Pixel {
            r: 255,
            ..Default::default()
        };
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
        self.sprite.position += offset * self.renderer.pixel_size;
    }

    pub fn resize_pixel(&mut self, size: i32) {
        self.renderer.pixel_size += size;
    }
}


