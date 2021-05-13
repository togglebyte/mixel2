use std::path::Path;

use anyhow::Result;
use nightmaregl::{Position, Size, Sprite, VertexData};
use nightmaregl::pixels::Pixel;

use super::layer::Layer;
use crate::binarytree::NodeId;

// -----------------------------------------------------------------------------
//     - Images -
// -----------------------------------------------------------------------------
pub enum ImageEntry {
    Occupied { image: Image, nodes: Vec<NodeId> },
    Vacant(Option<usize>),
}

impl ImageEntry {
    pub fn new(image: Image, node_id: NodeId) -> Self {
        Self::Occupied {
            image,
            nodes: vec![node_id],
        }
    }

    fn remove_node(&mut self, node: NodeId) {
        if let ImageEntry::Occupied { nodes, .. } = self {
            if let Some(pos) = nodes.iter().position(|n| *n == node) {
                nodes.remove(pos);
            }
        }
    }

    fn attach_node(&mut self, node: NodeId) {
        if let ImageEntry::Occupied { nodes, .. } = self {
            nodes.push(node);
        }
    }
}

pub struct Images {
    inner: Vec<ImageEntry>,
    next: Option<usize>,
}

impl Images {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            next: None,
        }
    }

    pub fn get_id_by_node(&self, node: NodeId) -> Option<usize> {
        self.inner
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| match entry {
                ImageEntry::Occupied { nodes, .. } if nodes.contains(&node) => Some(index),
                _ => None,
            }).next()
    }

    pub fn get_by_node(&self, node: NodeId) -> Option<&Image> {
        self.inner
            .iter()
            .filter_map(|entry| match entry {
                ImageEntry::Occupied { nodes, image } if nodes.contains(&node) => Some(image),
                _ => None,
            }).next()
    }

    pub fn get_by_node_mut(&mut self, node: NodeId) -> Option<&mut Image> {
        self.inner
            .iter_mut()
            .filter_map(|entry| match entry {
                ImageEntry::Occupied { nodes, image } if nodes.contains(&node) => Some(image),
                _ => None,
            }).next()
    }

    pub fn attach(&mut self, image_index: usize, node: NodeId) {
        self.inner[image_index].attach_node(node);
    }

    pub fn images(&self) -> impl Iterator<Item=(&Image, &Vec<NodeId>)> {
        self.inner
            .iter()
            .filter_map(|entry| match entry {
                ImageEntry::Vacant(_) => None,
                ImageEntry::Occupied { nodes, image } => Some((image, nodes)),
            })
    }

    fn resize(&mut self, image_index: usize, new_size: Size<i32>) {
        todo!();
        // self.inner[image_index].resize(new_size);
    }

    fn remove_entry(&mut self, index: usize) {
        let mut entry = ImageEntry::Vacant(self.next.take());
        self.next = Some(index);
        std::mem::swap(&mut entry, &mut self.inner[index]);
    }

    fn remove_from_all(&mut self, node: NodeId) {
        self.inner.iter_mut().for_each(|i| i.remove_node(node));
    }

    pub fn push(&mut self, image: Image, node_id: NodeId) {
        let entry = ImageEntry::new(image, node_id);

        let index = match self.next.take() {
            Some(index) => {
                if let ImageEntry::Vacant(next) = self.inner[index] {
                    self.next = next;
                    self.inner[index] = entry;
                }
            },
            None => {
                self.inner.push(entry);
            }
        };
    }
}


// -----------------------------------------------------------------------------
//     - Image -
// -----------------------------------------------------------------------------
pub struct Image {
    pub layers: Vec<Layer>,
    pub selected_layer: usize,
}

impl Image {
    pub(super) fn new(size: Size<i32>) -> Self {
        Self {
            layers: vec![Layer::new(size)],
            selected_layer: 0,
        }
    }

    pub(super) fn from_disk(path: impl AsRef<Path>) -> Result<Image> {
        // 1. Read a png file = 1 layer, 1 texture
        // 2. Mixel format: lots of layers and misc
        unimplemented!()
    }

    pub(super) fn put_pixel(&mut self, pixel: Pixel, pos: Position<i32>) {
        self.layers[self.selected_layer].push_pixel(pixel, pos);
        self.layers[self.selected_layer].draw_to_texture();
    }
}
