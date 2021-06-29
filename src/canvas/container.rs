//! A container wraps the image being drawn.
//! It holds:
//! * viewport
//! * border
//! * position
//! * cursor
use std::convert::TryInto;

use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Renderer, Sprite, Transform, VertexData, Viewport};
use nightmaregl::pixels::Pixel;

use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use super::{Cursor, Image};
use crate::Node;
use crate::layout::Split;

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
            cursor: Cursor::new(Position::zero()),
            colour: Pixel::black(),
        };

        inst.renderer.pixel_size = 8 * 2;

        // Centre the canvas.
        let pos = (*inst.viewport.size() / inst.renderer.pixel_size) / 2 - inst.node.sprite.size / 2;
        let transform = Transform::new(pos.to_vector());
        inst.node.transform = transform;

        Ok(inst)
    }

    pub fn move_cursor_by(&mut self, pos: Position<i32>) -> Position<i32> {
        let new_pos = self.cursor.node.transform.translation + pos;
        self.cursor.node.transform.translate_mut(new_pos);
        new_pos
    }

    pub fn move_cursor(&mut self, pos: Position<i32>) {
        self.cursor.node.transform.translate_mut(pos);
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
        let transform = &self.node.transform;
        let vertex_data = VertexData::new(&sprite, transform);

        // Render the "transparent" background texture
        self.renderer.render(
            background_texture,
            &[vertex_data],
            &self.viewport,
            ctx.context,
        );


        // Render all layers
        image.render(&self.renderer, sprite.clone(), transform, &self.viewport, ctx.context)?;

        // Cursor
        if self.cursor.visible {
            self.renderer.render(
                &self.cursor.texture,
                &[self.cursor.node.relative_vertex_data(&self.node.transform)],
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
        let pixel_size = self.renderer.pixel_size as f32;
        let viewport_pos = ctx.canvas_viewport.position.cast::<f32>();
        let canvas_pos = self.node.transform.translation;
        let pos = mouse_pos.cast() - viewport_pos;
        let mut pos = (pos.cast::<f32>() / pixel_size) - canvas_pos.cast();
        let height = self.node.sprite.size.height as f32;
        // pos -= Position::new(0.5, 0.5);
        pos.floor().cast() 
    }

    pub fn set_colour(&mut self, colour: Pixel) {
        self.colour = colour;
        self.cursor.set_colour(colour);
    }

    pub fn set_alpha(&mut self, alpha: usize) {
        match alpha.try_into() {
            Ok(a) => {
                self.colour.a = a;
                self.set_colour(self.colour);
            }
            Err(_) => {}
        }
    }
}
