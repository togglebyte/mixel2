use anyhow::Result;
use nightmaregl::texture::Texture;
use nightmaregl::{Position, Size, Sprite, VertexData, Point, Rect, FillMode};

pub struct Border {
    pub texture: Texture<i32>,
    pub vertex_data: [VertexData; 8],
}

impl Border {
    pub fn new(mut position: Position<i32>, size: Size<i32>, pixel_size: i32) -> Result<Self> {
        let z_index = 10;

        let size = size / pixel_size;
        let texture = Texture::from_disk("border.png")?;

        // Fixed sizes for the corners and the sides
        let corner_size = Size::new(3, 3);
        let vert_size = Size::new(3, size.height - corner_size.height * 2);
        let horz_size = Size::new(size.width - corner_size.width * 2, 3);

        // -----------------------------------------------------------------------------
        //     - Bottom left -
        // -----------------------------------------------------------------------------
        let mut bottom_left = Sprite::new(&texture);
        bottom_left.z_index = z_index;
        bottom_left.texture_rect = Rect::new(Point::new(0, 4), corner_size);
        bottom_left.position = position;
        bottom_left.size = corner_size;

        position.y += bottom_left.size.height;

        // -----------------------------------------------------------------------------
        //     - Left -
        // -----------------------------------------------------------------------------
        let mut left = Sprite::new(&texture);
        left.z_index = z_index;
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
        top.z_index = z_index;
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
        right.z_index = z_index;
        right.texture_rect = Rect::new(Point::new(4, 4), Size::new(3, 1));
        right.position = position;
        right.size = vert_size;

        position.y -= corner_size.height;

        // -----------------------------------------------------------------------------
        //     - Bottom right -
        // -----------------------------------------------------------------------------
        let mut bottom_right = Sprite::new(&texture);
        bottom_right.z_index = z_index;
        bottom_right.texture_rect = Rect::new(Point::new(4, 4), corner_size );
        bottom_right.position = position;
        bottom_right.size = corner_size;

        position.x -= horz_size.width;

        // -----------------------------------------------------------------------------
        //     - Bottom -
        // -----------------------------------------------------------------------------
        let mut bottom = Sprite::new(&texture);
        bottom.z_index = z_index;
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
            texture,
            vertex_data,
        };

        Ok(inst)
    }
}
