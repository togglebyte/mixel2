use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Renderer, Sprite, VertexData, Viewport, Transform};

use crate::border::{Border, BorderType};
use crate::listener::MessageCtx;

use crate::Node;
use super::{Direction, Cursor, Image};

// -----------------------------------------------------------------------------
//     - Container -
// -----------------------------------------------------------------------------
pub struct Container {
    dir: Direction,
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
        dir: Direction,
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

        inst.renderer.pixel_size = 8;

        // Centre the sprite
        // TODO: it doesn't quite look like it is in the centre
        //       is it the border? is it the viewport?
        let position = (*inst.viewport.size() / 2 / inst.renderer.pixel_size).to_vector();
        inst.node.transform.translate_mut(position);

        Ok(inst)
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
            &self.node.transform,
            ctx.textures,
            &self.viewport,
            ctx.border_renderer,
            ctx.context,
        );

        // Cursor
        self.renderer.render(
            &self.cursor.texture,
            &[self.cursor.node.relative_vertex_data(&self.node.transform)],
            &self.viewport,
            ctx.context,
        );

        // Images
        let vertex_data = self.node.vertex_data();

        // Render all layers
        image.layers.iter().for_each(|layer| {
            self.renderer
                .render(&layer.texture, &[vertex_data], &self.viewport, ctx.context);
        });

        // Render the "transparent" background texture
        self.renderer.render(
            background_texture,
            &[vertex_data],
            &self.viewport,
            ctx.context,
        );

        Ok(())
    }
}
