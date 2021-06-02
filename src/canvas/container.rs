use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Renderer, Sprite, Transform, VertexData, Viewport};
use nightmaregl::pixels::Pixel;

use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use super::{Cursor, Image, Orientation};
use crate::Node;

// -----------------------------------------------------------------------------
//     - Container -
// -----------------------------------------------------------------------------
pub struct Container {
    dir: Orientation,
    pub viewport: Viewport,
    pub renderer: Renderer<VertexData>,
    border: Border,
    pub node: Node<i32>,
    pub image_id: Option<usize>,
    cursor: Cursor,
    pub colour: Pixel,
}

impl Container {
    pub fn new(
        viewport: Viewport,
        dir: Orientation,
        ctx: &mut MessageCtx,
        sprite: Sprite<i32>,
        transform: Transform<i32>,
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

        inst.renderer.pixel_size = 7 * 3;

        // Centre the sprite
        // TODO: it doesn't quite look like it is in the centre
        //       is it the border? is it the viewport?
        // let position = (*inst.viewport.size() / 2 / inst.renderer.pixel_size).to_vector();
        // inst.node.transform.translate_mut(position);

        Ok(inst)
    }

    pub fn move_cursor_by(&mut self, pos: Position<i32>) {
        let new_pos = self.cursor.node.transform.translation + pos;
        self.cursor.node.transform.translate_mut(new_pos);
    }

    pub fn move_cursor(&mut self, pos: Position<i32>) {
        self.cursor.node.transform.translate_mut(pos);
    }

    pub fn render(
        &self,
        background_texture: &Texture<i32>,
        ctx: &mut MessageCtx,
        image: &Image,
        render_cursor: bool,
    ) -> Result<()> {
        // Border
        self.border.render(
            &self.node.transform,
            ctx.textures,
            &self.viewport,
            ctx.border_renderer,
            ctx.context,
        );

        // Cursor
        if render_cursor && self.cursor.visible {
            self.renderer.render(
                &self.cursor.texture,
                &[self.cursor.node.relative_vertex_data(&self.node.transform)],
                &self.viewport,
                ctx.context,
            )?;
        }

        // Images
        let vertex_data = self.node.vertex_data();

        // Render all layers
        image.render(&self.renderer, &[vertex_data], &self.viewport, ctx.context)?;

        // Render the "transparent" background texture
        self.renderer.render(
            background_texture,
            &[vertex_data],
            &self.viewport,
            ctx.context,
        );

        Ok(())
    }

    pub fn translate_mouse(&self, mouse_pos: Position<i32>, ctx: &MessageCtx) -> Position<i32> {
        let pixel_size = self.renderer.pixel_size as f32;
        let viewport_pos = ctx.canvas_viewport.position.cast::<f32>();
        let pos = mouse_pos.cast() - viewport_pos;
        let mut pos = (pos.cast::<f32>() / pixel_size);
        let height = self.node.sprite.size.height as f32;
        // pos.y = height - pos.y + 1.0;
        pos -= Position::new(0.5, 0.5);
        pos.floor().cast() 
    }

    pub fn set_colour(&mut self, colour: Pixel) {
        self.colour = colour;
        self.cursor.set_colour(colour);
    }
}
