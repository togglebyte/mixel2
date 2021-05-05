use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Renderer, Position, Size, Sprite, VertexData, Point, Rect, Context, Viewport};

use crate::listener::{Listener, Message, MessageCtx};

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
fn vertex_data(texture: &Texture<i32>, viewport: &Viewport, pixel_size: i32) -> [VertexData; VERTEX_DATA_SIZE] {
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
    let mut bottom_left = Sprite::new(&texture);
    bottom_left.z_index = z_index;
    bottom_left.texture_rect = Rect::new(Point::new(0, 4), corner_size);
    bottom_left.position = Position::zero();
    bottom_left.size = corner_size;

    // -----------------------------------------------------------------------------
    //     - Left -
    // -----------------------------------------------------------------------------
    let mut left = Sprite::new(&texture);
    left.z_index = z_index;
    left.texture_rect = Rect::new(Point::new(0, 4), Size::new(3, 1));
    left.position = Position::new(0, corner_size.height); 
    left.size = Size::new(corner_size.width, height - corner_size.height * 2);

    // -----------------------------------------------------------------------------
    //     - Top left -
    // -----------------------------------------------------------------------------
    let mut top_left = Sprite::new(&texture);
    top_left.texture_rect = Rect::new(Point::new(0, 0), corner_size);
    top_left.position = Position::new(0, height - corner_size.height);
    top_left.size = corner_size;

    // -----------------------------------------------------------------------------
    //     - Top -
    // -----------------------------------------------------------------------------
    let mut top = Sprite::new(&texture);
    top.z_index = z_index;
    top.texture_rect = Rect::new(Point::new(3, 0), Size::new(1, 3));
    top.position = Position::new(corner_size.width, height - corner_size.height);
    top.size = Size::new(width - corner_size.width * 2, corner_size.height);  

    // -----------------------------------------------------------------------------
    //     - Top right -
    // -----------------------------------------------------------------------------
    let mut top_right = Sprite::new(&texture);
    top_right.texture_rect = Rect::new(Point::new(4, 0), corner_size);
    top_right.position = Position::new(width - corner_size.width, height - corner_size.height);
    top_right.size = corner_size;
    
    // -----------------------------------------------------------------------------
    //     - Right -
    // -----------------------------------------------------------------------------
    let mut right = Sprite::new(&texture);
    right.z_index = z_index;
    right.texture_rect = Rect::new(Point::new(4, 4), Size::new(3, 1));
    right.position = Position::new(width - corner_size.width, corner_size.height); 
    right.size = Size::new(corner_size.width, height - corner_size.height * 2);

    // -----------------------------------------------------------------------------
    //     - Bottom right -
    // -----------------------------------------------------------------------------
    let mut bottom_right = Sprite::new(&texture);
    bottom_right.z_index = z_index;
    bottom_right.texture_rect = Rect::new(Point::new(4, 4), corner_size );
    bottom_right.position = Position::new(width - corner_size.width, 0);
    bottom_right.size = corner_size;

    // -----------------------------------------------------------------------------
    //     - Bottom -
    // -----------------------------------------------------------------------------
    let mut bottom = Sprite::new(&texture);
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
    texture: Texture<i32>,
    vertex_data: [VertexData; VERTEX_DATA_SIZE],
    renderer: Renderer<VertexData>,
    viewport: Viewport,
}

impl Border {
    pub fn new(win_size: Size<i32>, context: &mut Context) -> Result<Self> {
        // -----------------------------------------------------------------------------
        //     - Renderer and viewport -
        // -----------------------------------------------------------------------------
        let renderer = Renderer::default(context)?;
        let viewport = viewport(win_size, renderer.pixel_size);

        let texture = Texture::from_disk("border.png")?;
        let vertex_data = vertex_data(&texture, &viewport, renderer.pixel_size);

        let inst = Self {
            texture,
            vertex_data,
            renderer,
            viewport,
        };

        Ok(inst)
    }
}

impl Listener for Border {
    fn message(&mut self, message: &Message, _: &MessageCtx) -> Message {
        if let Message::Resize(new_size) = message {
            self.viewport = viewport(*new_size, self.renderer.pixel_size);
            self.vertex_data = vertex_data(
                &self.texture,
                &self.viewport,
                self.renderer.pixel_size
            );
        }

        Message::Noop
    }

    fn render(&mut self, context: &mut Context) -> Result<()> {
        self.renderer.render(
            &self.texture,
            &self.vertex_data,
            &self.viewport,
            context
        )?;
        Ok(())
    }
}
