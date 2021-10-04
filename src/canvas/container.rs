//! A container wraps the image being drawn.
//! It holds:
//! * viewport
//! * border
//! * position
//! * cursor
use std::convert::TryInto;

use anyhow::Result;
use nightmare::pixels::Pixel;
use nightmare::texture::Texture;
use nightmare::{Position, Size, Sprite, Transform, Vector, VertexData, Viewport, create_model_matrix};
use nightmare::text::{Text, WordWrap};
use nightmare::render2d::{SimpleRenderer, Model};

use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use super::{Cursor, Image, Coords};
use crate::layout::Split;
use crate::Node;

const MAX_ZOOM: i32 = 60;

// -----------------------------------------------------------------------------
//     - Container -
// -----------------------------------------------------------------------------
pub struct Container {
    pub(super) border: Border,
    pub viewport: Viewport,
    pub node: Node,
    pub image_id: Option<usize>,
    pub colour: Pixel,
    pub(super) scale: u32,
    pub container_id: usize,

    dir: Split,
    cursor: Cursor,
    renderer: SimpleRenderer<Model>,
}

impl Container {
    pub fn new(
        container_id: usize,
        viewport: Viewport,
        dir: Split,
        ctx: &mut MessageCtx,
        sprite: Sprite,
    ) -> Result<Self> {

        let border_type = BorderType::Inactive;

        let font_size = 22.0;

        let renderer = SimpleRenderer::new(ctx.context, viewport.view_projection())?;

        let mut inst = Self {
            border: Border::new(border_type, ctx.textures, &viewport),
            viewport,
            renderer,
            node: Node::from_sprite(sprite),
            dir,
            image_id: None,
            cursor: Cursor::new(Coords::zeros(), sprite.anchor),
            colour: Pixel::black(),
            scale: 8,
        };

        // Centre the canvas.
        let pos = (*inst.viewport.size() / 2).cast();
        let mut transform = Transform::new(pos.to_vector());
        transform.scale_mut(inst.scale);
        inst.node.transform = transform;

        inst.text.set_text(format!("id: {}", inst.container_id));

        Ok(inst)
    }

    pub fn move_cursor_by(&mut self, coords: Coords) -> Coords {
        let translation = self.cursor.node.transform.translation;
        let height = self.node.sprite.size.height - 1;
        let mut current_coords = Coords::from_translation(translation, height);
        current_coords + coords
    }

    pub fn move_cursor(&mut self, coords: Coords) {
        let height = self.node.sprite.size.height - 1;
        self.cursor.node.transform.translate_mut(coords.to_translation(height));
    }

    pub fn render(
        &self,
        background_texture: &Texture,
        ctx: &mut MessageCtx,
        image: &Image,
    ) -> Result<()> {
        // Border
        // self.border.render(
        //     &Transform::default(),
        //     ctx.textures,
        //     &self.viewport,
        //     ctx.border_renderer,
        //     ctx.context,
        // )?;

        let mut sprite = self.node.sprite;
        sprite.z_index = 999;
        let mut transform = self.node.transform;
        // transform.scale_mut(self.scale);
        let model = create_model_matrix(&sprite, &transform);

        // Render the "transparent" background texture
        background_texture.bind();
        self.renderer.load_data(&[model], ctx.context);
        self.renderer.render_instanced(ctx.context, 1);

        // Render all layers
        image.render(
            &self.renderer,
            sprite.clone(),
            &transform,
            &self.viewport,
            ctx.context,
        )?;

        // Cursor
        if self.cursor.visible {
            self.renderer.render(
                &self.cursor.texture,
                &[self.cursor.node.relative_vertex_data(&transform)],
                &self.viewport,
                ctx.context,
            )?;
        }

        // Container id
        self.text_renderer.render(
            &self.text.texture(),
            &self.text.vertex_data(),
            &self.viewport,
            ctx.context,
        );

        Ok(())
    }

    pub fn resize(&mut self) {
        self.border.resize(&self.viewport);
    }

    pub fn translate_mouse(&self, mouse_pos: Position, ctx: &MessageCtx) -> Position {
        let transform = self.node.transform;
        let canvas_pos = transform.translation;
        let mut pos = Position::new(mouse_pos.x - canvas_pos.x, mouse_pos.y - canvas_pos.y);

        pos.x /= self.scale.x;
        pos.y /= self.scale.y;

        // let viewport_pos = ctx.canvas_viewport.position;
        // let canvas_pos = self.node.transform.translation.cast::<i32>();
        // let pos = mouse_pos - viewport_pos;
        // let mut pos = pos - canvas_pos;

        pos
    }

    pub fn set_colour(&mut self, colour: Pixel) {
        self.colour = colour;
        self.cursor.set_colour(colour);
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        self.colour.a = alpha;
        self.set_colour(self.colour);
    }

    pub fn scale(&mut self, diff: i32) {
        if diff > 0 && self.scale.x < MAX_ZOOM {
            self.scale += Vector::new(diff, diff);
        }
        if diff < 0 && self.scale.x > 1 {
            self.scale += Vector::new(diff, diff);
        }

        self.node.transform.scale_mut(self.scale);
    }
}
