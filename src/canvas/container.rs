use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Renderer, Sprite, Transform, VertexData, Viewport};

use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use super::{Cursor, Orientation, Image};
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
        };

        inst.renderer.pixel_size = 8 * 3;

        // Centre the sprite
        // TODO: it doesn't quite look like it is in the centre
        //       is it the border? is it the viewport?
        // let position = (*inst.viewport.size() / 2 / inst.renderer.pixel_size).to_vector();
        // inst.node.transform.translate_mut(position);

        Ok(inst)
    }

    pub fn move_cursor(&mut self, pos: Position<i32>) {
        let transform = &mut self.cursor.node.transform;
        let pos = transform.translation + pos;
        transform.translate_mut(pos);
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
        if render_cursor {
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
        for layer in &image.layers {
            self.renderer.render(
                &layer.texture,
                &[vertex_data], 
                &self.viewport, 
                ctx.context
            )?;
        }

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
        let pos: Position<f32> = (mouse_pos.cast::<f32>() - ctx.viewport.position.cast()) / self.renderer.pixel_size as f32;
        let pos = pos - Position::new(0.5, 0.5);
        pos.floor().cast()

        // self.node.transform.translation * self.renderer.pixel_size
            // + mouse_pos
    }
}
