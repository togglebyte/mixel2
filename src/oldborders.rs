use std::path::Path;

use anyhow::Result;
use nightmare::texture::Texture;
use nightmare::{Renderer, Position, Size, Sprite, VertexData, Point, Rect, Context, Viewport, RelativeViewport};

use crate::listener::{Listener, MessageCtx};
use crate::message::Message;

const VERTEX_DATA_SIZE: usize = 8;

// -----------------------------------------------------------------------------
//     - Viewport -
// -----------------------------------------------------------------------------
fn viewport(win_size: Size<i32>, pixel_size: i32) -> Viewport {
    let padding = 128 * pixel_size;
    let position = Position::new(padding, padding);
    let size = Size::new(win_size.width - padding * 2, win_size.height - padding * 2);
    Viewport::new(position, size)
}

// -----------------------------------------------------------------------------
//     - Vertex data -
// -----------------------------------------------------------------------------
fn vertex_data(texture_size: Size<i32>, viewport: &Viewport, pixel_size: i32) -> [VertexData; VERTEX_DATA_SIZE] {
    let z_index = 10;

    // Fixed sizes for the corners and the sides
    let corner_size = Size::new(3, 3);
    let (width, height) = {
        let size = viewport.size();
        (size.width / pixel_size, size.height / pixel_size)
    };

    // -----------------------------------------------------------------------------
    //     - Bottom left -
    // -----------------------------------------------------------------------------
    let mut bottom_left = Sprite::from_size(texture_size);
    bottom_left.z_index = z_index;
    bottom_left.texture_rect = Rect::new(Point::new(0, 4), corner_size);
    bottom_left.position = Position::zeros();
    bottom_left.size = corner_size;

    // -----------------------------------------------------------------------------
    //     - Left -
    // -----------------------------------------------------------------------------
    let mut left = Sprite::from_size(texture_size);
    left.z_index = z_index;
    left.texture_rect = Rect::new(Point::new(0, 4), Size::new(3, 1));
    left.position = Position::new(0, corner_size.height); 
    left.size = Size::new(corner_size.width, height - corner_size.height * 2);

    // -----------------------------------------------------------------------------
    //     - Top left -
    // -----------------------------------------------------------------------------
    let mut top_left = Sprite::from_size(texture_size);
    top_left.texture_rect = Rect::new(Point::new(0, 0), corner_size);
    top_left.position = Position::new(0, height - corner_size.height);
    top_left.size = corner_size;

    // -----------------------------------------------------------------------------
    //     - Top -
    // -----------------------------------------------------------------------------
    let mut top = Sprite::from_size(texture_size);
    top.z_index = z_index;
    top.texture_rect = Rect::new(Point::new(3, 0), Size::new(1, 3));
    top.position = Position::new(corner_size.width, height - corner_size.height);
    top.size = Size::new(width - corner_size.width * 2, corner_size.height);  

    // -----------------------------------------------------------------------------
    //     - Top right -
    // -----------------------------------------------------------------------------
    let mut top_right = Sprite::from_size(texture_size);
    top_right.texture_rect = Rect::new(Point::new(4, 0), corner_size);
    top_right.position = Position::new(width - corner_size.width, height - corner_size.height);
    top_right.size = corner_size;
    
    // -----------------------------------------------------------------------------
    //     - Right -
    // -----------------------------------------------------------------------------
    let mut right = Sprite::from_size(texture_size);
    right.z_index = z_index;
    right.texture_rect = Rect::new(Point::new(4, 4), Size::new(3, 1));
    right.position = Position::new(width - corner_size.width, corner_size.height); 
    right.size = Size::new(corner_size.width, height - corner_size.height * 2);

    // -----------------------------------------------------------------------------
    //     - Bottom right -
    // -----------------------------------------------------------------------------
    let mut bottom_right = Sprite::from_size(texture_size);
    bottom_right.z_index = z_index;
    bottom_right.texture_rect = Rect::new(Point::new(4, 4), corner_size );
    bottom_right.position = Position::new(width - corner_size.width, 0);
    bottom_right.size = corner_size;

    // -----------------------------------------------------------------------------
    //     - Bottom -
    // -----------------------------------------------------------------------------
    let mut bottom = Sprite::from_size(texture_size);
    bottom.z_index = z_index;
    bottom.texture_rect = Rect::new(Point::new(3, 4), Size::new(1, 3));
    bottom.position = Position::new(corner_size.width, 0);
    bottom.size = Size::new(width - corner_size.width * 2, corner_size.height);  

    let vertex_data = [
        bottom_left.vertex_data(),
        left.vertex_data(),
        top_left.vertex_data(),
        top.vertex_data(),
        top_right.vertex_data(),
        right.vertex_data(),
        bottom_right.vertex_data(),
        bottom.vertex_data(),
    ];

    vertex_data
}

// -----------------------------------------------------------------------------
//     - Border -
// -----------------------------------------------------------------------------
pub struct Border {
    vertex_data: [VertexData; VERTEX_DATA_SIZE],
    renderer: Renderer<VertexData>,
    viewport: RelativeViewport,
}

impl Border {
    pub fn new(
        viewport: &Viewport,
        context: &mut Context,
        texture_size: Size<i32>,
    ) -> Result<Self> {
        // -----------------------------------------------------------------------------
        //     - Renderer and viewport -
        // -----------------------------------------------------------------------------
        let win_size = *viewport.size();
        let renderer = Renderer::default(context)?;

        let vertex_data = vertex_data(texture_size, &viewport, renderer.pixel_size);

        // let viewport = viewport.sub_viewport(viewport.position, viewport.position);
        let viewport = viewport.relative(Position::zeros(), Position::zeros());

        let inst = Self {
            vertex_data,
            renderer,
            viewport,
        };

        Ok(inst)
    }

    pub fn render(&mut self, texture: &Texture<i32>, context: &mut Context) -> Result<()> {
        self.renderer.render(
            texture,
            &self.vertex_data,
            self.viewport.viewport(),
            context
        )?;
        Ok(())
    }
}
