use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Size, Sprite, VertexData, Point, Rect, FillMode};

// TODO: A Viewport border:
// * Eight sprites (each corner, each side)
// * Sprite sheet as texture
// * Repeat x,y texture for the sides
//
// Border colours:
// #081726 Dark
// #18426d Mid
// #2d639a Light
//
// Note: add a black pixel border around the actual canvas sprite as well
pub struct Border {
    top_left: Sprite<i32>,
    top: Sprite<i32>,
    top_right: Sprite<i32>,
    pub texture: Texture<i32>,
    pub vertex_data: [VertexData; 8],
}

impl Border {
    pub fn new(mut position: Position<i32>, size: Size<i32>, pixel_size: u16) -> Result<Self> {
        let size = size / pixel_size as i32;
        let texture = Texture::from_disk("border2.png")?;

        // Fixed sizes for the corners and the sides
        let corner_size = Size::new(3, 3);
        let vert_size = Size::new(3, size.height - corner_size.height * 2 - position.y);
        let horz_size = Size::new(size.width - corner_size.width * 2 - position.x, 3);

        // -----------------------------------------------------------------------------
        //     - Bottom left -
        // -----------------------------------------------------------------------------
        let mut bottom_left = Sprite::new(&texture);
        bottom_left.texture_rect = Rect::new(Point::new(0, 4), corner_size);
        bottom_left.position = position;
        bottom_left.size = corner_size;

        position.y += bottom_left.size.height;

        // -----------------------------------------------------------------------------
        //     - Left -
        // -----------------------------------------------------------------------------
        let mut left = Sprite::new(&texture);
        left.texture_rect = Rect::new(Point::new(0, 4), Size::new(3, 1));
        left.position = position;
        left.size = vert_size;

        position.y += left.size.height;

        // -----------------------------------------------------------------------------
        //     - Top left -
        // -----------------------------------------------------------------------------
        let mut top_left = Sprite::new(&texture);
        top_left.texture_rect = Rect::new(Point::new(0, 0), corner_size);
        top_left.position = position;
        top_left.size = corner_size;

        // Offset the position by the top left corner
        position.x += top_left.size.width;

        // -----------------------------------------------------------------------------
        //     - Top -
        // -----------------------------------------------------------------------------
        let mut top = Sprite::new(&texture);
        top.texture_rect = Rect::new(Point::new(3, 0), Size::new(1, 3));
        top.position = position;
        top.size = horz_size;

        position.x += top.size.width;

        // -----------------------------------------------------------------------------
        //     - Top right -
        // -----------------------------------------------------------------------------
        let mut top_right = Sprite::new(&texture);
        top_right.texture_rect = Rect::new(Point::new(4, 0), corner_size);
        top_right.position = position;
        top_right.size = corner_size;

        position.y -= vert_size.height;
        
        // -----------------------------------------------------------------------------
        //     - Right -
        // -----------------------------------------------------------------------------
        let mut right = Sprite::new(&texture);
        right.texture_rect = Rect::new(Point::new(4, 4), Size::new(3, 1));
        right.position = position;
        right.size = vert_size;

        position.y -= corner_size.height;

        // -----------------------------------------------------------------------------
        //     - Bottom right -
        // -----------------------------------------------------------------------------
        let mut bottom_right = Sprite::new(&texture);
        bottom_right.texture_rect = Rect::new(Point::new(4, 4), corner_size );
        bottom_right.position = position;
        bottom_right.size = corner_size;

        position.x -= horz_size.width;

        // -----------------------------------------------------------------------------
        //     - Bottom -
        // -----------------------------------------------------------------------------
        let mut bottom = Sprite::new(&texture);
        bottom.texture_rect = Rect::new(Point::new(3, 4), Size::new(1, 3));
        bottom.position = position;
        bottom.size = horz_size;

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

        let inst = Self {
            top_left,
            top,
            top_right,
            texture,
            vertex_data,
        };

        Ok(inst)
    }
}
