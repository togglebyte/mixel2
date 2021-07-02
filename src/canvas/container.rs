//! A container wraps the image being drawn.
//! It holds:
//! * viewport
//! * border
//! * position
//! * cursor
use std::convert::TryInto;

use anyhow::Result;
use nightmaregl::pixels::Pixel;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Renderer, Size, Sprite, Transform, Vector, VertexData, Viewport};

use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use super::{Cursor, Image};
use crate::layout::Split;
use crate::Node;

const MAX_ZOOM: i32 = 60;

// -----------------------------------------------------------------------------
//     - Container -
// -----------------------------------------------------------------------------
pub struct Container {
    dir: Split,
    pub viewport: Viewport,
    pub renderer: Renderer<VertexData>,
    pub(super) border: Border,
    pub node: Node<i32>,
    pub image_id: Option<usize>,
    cursor: Cursor,
    pub colour: Pixel,
    pub scale: Vector<i32>,
}

impl Container {
    pub fn new(
        viewport: Viewport,
        dir: Split,
        ctx: &mut MessageCtx,
        sprite: Sprite<i32>,
    ) -> Result<Self> {
        let border_type = BorderType::Inactive;

        let mut inst = Self {
            border: Border::new(border_type, ctx.textures, &viewport),
            viewport,
            renderer: Renderer::default(ctx.context)?,
            node: Node::from_sprite(sprite),
            dir,
            image_id: None,
            cursor: Cursor::new(Position::zero(), sprite.anchor),
            colour: Pixel::black(),
            scale: Vector::new(4, 4),
        };

        // Centre the canvas.
        let pos = (*inst.viewport.size() / 2).cast();
        let mut transform = Transform::new(pos.to_vector());
        transform.scale_mut(inst.scale);
        inst.node.transform = transform;

        Ok(inst)
    }

    pub fn move_cursor_by(&mut self, pos: Position<i32>) -> Position<i32> {
        let translation = self.cursor.node.transform.translation;
        let new_pos = Position::new(translation.x + pos.x, translation.y + pos.y);
        self.cursor.node.transform.translate_mut(new_pos.cast());
        new_pos
    }

    pub fn move_cursor(&mut self, pos: Position<i32>) {
        self.cursor.node.transform.translate_mut(pos.cast());
    }

    pub fn render(
        &self,
        background_texture: &Texture<i32>,
        ctx: &mut MessageCtx,
        image: &Image,
    ) -> Result<()> {
        // Border
        self.border.render(
            &Transform::default(),
            ctx.textures,
            &self.viewport,
            ctx.border_renderer,
            ctx.context,
        )?;

        let mut sprite = self.node.sprite;
        sprite.z_index = 999;
        let mut transform = self.node.transform;
        // transform.scale_mut(self.scale);
        let vertex_data = VertexData::new(&sprite, &transform);

        // Render the "transparent" background texture
        self.renderer.render(
            background_texture,
            &[vertex_data],
            &self.viewport,
            ctx.context,
        );

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

        Ok(())
    }

    pub fn resize(&mut self) {
        self.border.resize(&self.viewport);
    }

    pub fn translate_mouse(&self, mouse_pos: Position<i32>, ctx: &MessageCtx) -> Position<i32> {
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
