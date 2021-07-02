use std::path::Path;

use anyhow::Result;
use nightmaregl::{Size, VertexData, Context, Viewport, Renderer, Transform, Sprite};
use nightmaregl::pixels::Pixel;
use nightmaregl::texture::Texture;

use super::layer::{LayerId, Layer, Coords};

// -----------------------------------------------------------------------------
//     - Image -
// -----------------------------------------------------------------------------
pub struct Image {
    layers: Vec<Layer>,
    pub layer_id: LayerId,
    pub dirty: bool,
}

impl Image {
    pub(super) fn new(size: Size<i32>) -> Self {
        Self {
            layers: vec![Layer::new(size.cast())],
            layer_id: LayerId::from_index(0)t,
            dirty: false,
        }
    }

    pub(super) fn from_disk(path: impl AsRef<Path>) -> Result<Image> {
        // 1. Read a png file = 1 layer, 1 texture
        // 2. Mixel format: lots of layers and misc
        unimplemented!()
    }

    pub(super) fn put_pixel(&mut self, pixel: Pixel, coords: Coords) {
        self.layers[self.layer_id.as_index()].push_pixel(pixel, coords);
        self.dirty = true;
    }

    pub(super) fn clear_pixel(&mut self, coords: Coords) {
        self.layers[self.layer_id.as_index()].push_pixel(Pixel::transparent(), pos);
        self.dirty = true;
    }

    pub(super) fn new_layer(&mut self, size: Size<i32>) -> (LayerId, usize) {
        let new_layer_id = LayerId::from_index(self.layers.len());
        self.layers.push(Layer::new(size));
        self.layer_id = new_layer_id;
        (new_layer_id, self.layers.len())
    }

    pub(super) fn set_layer(&mut self, layer_id: LayerId) -> (LayerId, usize) { 
        match layer_id.as_index() >= self.layers.len() {
            true => {},
            false => self.layer_id = layer_id,
        }

        (layer_id, self.layers.len())
    }

    pub(super) fn remove_layer(&mut self) -> Option<(LayerId, usize)> {
        if self.layers.len() == 1 {
            self.clear_layer();
            return None;
        }

        self.layers.remove(self.layer_id.as_index());
        
        match self.layer_id.as_index() {
            0 => Some((self.layer_id, self.layers.len())),
            _ => {
                if self.layer_id.as_index() >= self.layers.len() {
                    self.layer_id = LayerId::from_index(self.layer_id.as_index() - 1);
                }
                Some((self.layer_id, self.layers.len()))
            }
        }
    }

    pub(super) fn clear_layer(&mut self) {
        self.layers[self.layer_id.as_index()].clear();
        self.dirty = true;
    }

    pub(super) fn redraw_layers(&mut self) {
        self.layers.iter_mut().filter(|l| l.dirty).for_each(Layer::draw_to_texture);
        self.dirty = false;
    }

    pub fn render(
        &self,
        renderer: &Renderer<VertexData>, 
        mut sprite: Sprite<i32>,
        transform: &Transform<i32>,
        viewport: &Viewport,
        context: &mut Context
    ) -> Result<()> {

        const SPRITE_Z: i32 = 150;

        // Do NOT reverse these AGAIN!
        // They are in the correct order now.
        for (mut z_index, layer) in self.layers.iter().enumerate() {
            sprite.z_index = SPRITE_Z - z_index as i32;
            let vertex_data = VertexData::new(&sprite, transform);
            renderer
                .render(&layer.texture, &[vertex_data], viewport, context)?;
        }

        Ok(())
    }

}
