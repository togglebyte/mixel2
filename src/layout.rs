use std::mem::replace;
use std::fmt;

use nightmaregl::{Position, Size, Rect, Point};
use crate::canvas::Container;

#[derive(Debug, Copy, Clone)]
pub enum Split {
    Horz,
    Vert,
}

fn placeholder() -> Layout {
    Layout::Leaf { id: std::usize::MAX, size: Size::new(1, 1), pos: Position::zero() }
}

#[derive(Debug)]
pub enum Layout {
    Leaf { id: usize, size: Size<i32>, pos: Position<i32> },
    Branch { left: Box<Layout>, right: Box<Layout>, size: Size<i32>, pos: Position<i32> },
}

impl Layout {
    fn find_node(&mut self, node_id: usize) -> Option<&mut Layout> {
        match self {
            Layout::Leaf{ id, .. } if *id == node_id => Some(self),
            Layout::Leaf{ id, .. } => None,
            Layout::Branch { left, right, .. } => {
                match left.find_node(node_id) {
                    Some(node) => Some(node),
                    None => match right.find_node(node_id) {
                        Some(node) => Some(node),
                        None => None,
                    }
                }
            }
        }
    }

    fn pos(&self) -> Position<i32> {
        match self {
            Self::Leaf { pos, .. } => *pos,
            Self::Branch { pos, .. } => *pos,
        }
    }

    fn size(&self) -> Size<i32> {
        match self {
            Self::Leaf { size, .. } => *size,
            Self::Branch { size, .. } => *size,
        }
    }

    pub fn set_size(&mut self, new_size: Size<i32>) {
        match self {
            Self::Leaf { size, .. } => *size = new_size,
            Self::Branch { size, .. } => *size = new_size,
        }
    }

    fn set_pos(&mut self, new_pos: Position<i32>) {
        match self {
            Self::Leaf { pos, .. } => *pos = new_pos,
            Self::Branch { pos, .. } => *pos = new_pos,
        }
    }

    pub fn split(&mut self, left_id: usize, right_id: usize, split: Split) {
        if let Some(node) = self.find_node(left_id) {
            match node {
                Layout::Leaf { size, pos, .. } => {
                    let new_size = match split {
                        Split::Horz => Size::new(size.width, size.height / 2),
                        Split::Vert => Size::new(size.width / 2, size.height),
                    };

                    let left_pos = *pos;
                    let mut right_pos = left_pos;

                    match split {
                        Split::Vert => right_pos.x += new_size.width,
                        Split::Horz => right_pos.y += new_size.height,
                    }

                    *node = Layout::Branch {
                        left: Box::new(Layout::Leaf { id: left_id, size: new_size, pos: left_pos }),
                        right: Box::new(Layout::Leaf { id: right_id, size: new_size, pos: right_pos }),
                        size: *size,
                        pos: *pos,
                    };

                }
                Layout::Branch { left, right, .. } => unreachable!(),
            }
        }
    }

    pub fn resize(&mut self, node_id: usize, new_size: Size<i32>) {
        if let Some(n) = self.find_node(node_id) {
            n.set_size(new_size);
            self.rebuild();
        }
    }

    pub fn collapse(&mut self, node_id: usize) -> bool {
        // If this is a leaf then return false as we can't
        // progress down this path anymore.
        let (left, right) = match self {
            Layout::Leaf { .. } => return false,
            Layout::Branch { left, right, .. } => {
                (left, right)
            }
        };

        // If the left node id matches the `node_id` then
        // swap the parent for the right node
        if matches!(left.as_mut(), Layout::Leaf {id, ..} if *id == node_id) {
            let new_parent = replace(right.as_mut(), placeholder());
            *self = new_parent;
            return true;
        }
    
        // If the right node id matches the `node_id` then
        // swap the parent for the left node
        if matches!(right.as_mut(), Layout::Leaf {id, ..} if *id == node_id) {
            let new_parent = replace(left.as_mut(), placeholder());
            *self = new_parent;
            return true;
        }

        match left.collapse(node_id) {
            true => true,
            false => right.collapse(node_id)
        }
    }

    pub fn rebuild(&mut self) {
        let parent_pos = self.pos();

        let size = match self {
            Layout::Leaf { size, .. } => { *size },
            Layout::Branch { left, right, size, pos } => {
                *pos = parent_pos;

                left.set_pos(parent_pos);
                let (right_pos, right_size) = {
                    let mut right_pos = parent_pos;
                    let mut right_size = *size;
                    if left.pos().x != right.pos().x {
                        let width = left.size().width;
                        right_pos.x += width;
                        right_size.width -= width;
                    } else {
                        let height = left.size().height;
                        right_pos.y += height;
                        right_size.height -= height;
                    }
                    (right_pos, right_size)
                };

                right.set_pos(right_pos);
                right.set_size(right_size);

                left.rebuild();
                right.rebuild();

                *size
            }
        };

        self.set_size(size);
    }

    pub fn layout(&self, containers: &mut [Container]) {
        match self {
            Layout::Leaf { id, size, pos } => {
                let vp = &mut containers[*id].viewport;
                vp.position = *pos;
                vp.resize(*size);
            }
            Layout::Branch { left, right, .. } => {
                left.layout(containers);
                right.layout(containers);
            }
        }
    }

}

// -----------------------------------------------------------------------------
//     - Dispay -
// -----------------------------------------------------------------------------
enum LeafType {
    Root,
    Left,
    Right,
}

impl Layout {
    fn display(&self, level: usize, f: &mut fmt::Formatter<'_>, leaf_type: LeafType) {
        let side = match leaf_type {
            LeafType::Root => "root",
            LeafType::Left => "left",
            LeafType::Right => "right"
        };

        let spacer = " ".repeat(level * 2);

        match self {
            Layout::Leaf { id, size, pos } => {
                write!(f, "{} {}{} leaf: {} ({:?} | {:?})\n", level, spacer, side, id, pos, size);
            },
            Layout::Branch { left, right, size, pos } => {
                write!(f, "{} {}{} branch ({:?} | {:?})\n", level, spacer, side, pos, size); 
                left.display(level + 1, f, LeafType::Left);
                right.display(level + 1, f, LeafType::Right);
            }
        }

    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(0, f, LeafType::Root);
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use nightmaregl::*;

    #[test]
    fn split_horz_and_resize() {
        let mut tree = Layout::Leaf { id: 0, pos: Position::zero(), size: Size::new(20, 20) };
        tree.split(0, 100, Split::Horz);
        tree.resize(0, Size::new(20, 5));

        // Left branch
        let expected = (0, Size::new(20, 5), Position::zero());
        let actual = tree.layout()[0];
        assert_eq!(expected, actual);

        // Right branch
        let expected = (100, Size::new(20, 15), Position::new(0, 5));
        let actual = tree.layout()[1];
        assert_eq!(expected, actual);
    }

    #[test]
    fn double_split() {
        let mut tree = Layout::Leaf { id: 0, pos: Position::zero(), size: Size::new(20, 20) };
        tree.split(0, 100, Split::Horz);
        tree.split(100, 200, Split::Vert);
        tree.resize(100, Size::new(3, 10));

        let layout = tree.layout();
        let expected = (200, Size::new(17, 10), Position::new(3, 10));
        let actual = layout[2];
        assert_eq!(expected, actual);
    }
}

