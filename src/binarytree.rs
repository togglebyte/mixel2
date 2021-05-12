use std::mem;
use std::ptr::NonNull;
use std::fmt;
use std::ops::{Index, IndexMut, Deref, DerefMut};

use nightmaregl::pixels::{Pixels, Pixel};
use nightmaregl::texture::{Format, Texture};
use nightmaregl::{ Position, Size, Sprite };

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct NodeId(usize);

// -----------------------------------------------------------------------------
//     - Tree entry -
// -----------------------------------------------------------------------------
enum Entry<T> {
    Occupied(Node<T>),
    Vacant(Option<NodeId>),
}

// -----------------------------------------------------------------------------
//     - Tree -
// -----------------------------------------------------------------------------
pub struct Tree<T> {
    inner: Vec<Entry<T>>,
    next: Option<NodeId>,
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        let root_node = Node::new(root, NodeId(0), None);
        Self {
            inner: vec![Entry::Occupied(root_node)],
            next: None,
        }
    }

    pub fn root_id(&self) -> NodeId {
        NodeId(0)
    }

    // TODO: do we perhaps want to move the inner value to the parent
    //       and remove the remaining branch?
    pub fn remove(&mut self, index: NodeId) -> Node<T> {
        if index.0 == 0 {
            panic!("Can not remove the root node");
        }

        if let Entry::Vacant(_) = self.inner[index.0] {
            panic!("tried to remove vacand entry");
        }

        let mut ret_val = Entry::Vacant(self.next.take());
        mem::swap(&mut ret_val, &mut self.inner[index.0]);
        self.next = Some(index);

        // Get the node to remove
        let node = match ret_val {
            Entry::Vacant(_) => unreachable!(),
            Entry::Occupied(node) => node,
        };

        // Remove the children
        if let Some(left) = node.left {
            self.remove(left);
        }

        if let Some(right) = node.right {
            self.remove(right);
        }

        // Remove the node from the parent
        if let Some(parent_id) = node.parent {
            let parent = match self.inner.get_mut(parent_id.0) {
                Some(Entry::Occupied(p)) => p,
                _ => return node,
            };

            if parent.left == Some(node.id) {
                parent.left = None;
            }

            if parent.right == Some(node.id) {
                parent.right = None;
            }

        }

        node
    }

    fn push(&mut self, val: T, parent: NodeId) -> NodeId {
        let index = match self.next {
            Some(ref id) => *id,
            None => {
                let index = self.inner.len();
                self.inner.push(Entry::Vacant(None));
                NodeId(index)
            }
        };

        self.inner[index.0] = Entry::Occupied(Node::new(val, index, Some(parent)));
        index
    }

    pub fn insert_left(&mut self, parent_id: NodeId, val: T) -> NodeId {
        let id = self.push(val, parent_id);
        if let Entry::Occupied(ref mut parent) = self.inner[parent_id.0] {
            parent.left = Some(id);
        }
        id
    }

    pub fn insert_right(&mut self, parent_id: NodeId, val: T) -> NodeId {
        let id = self.push(val, parent_id);
        if let Entry::Occupied(ref mut parent) = self.inner[parent_id.0] {
            parent.right = Some(id);
        }
        id
    }

    pub fn iter(&self) -> TreeIter<T> {
        TreeIter {
            index: 0,
            tree: self
        }
    }
}

pub struct TreeIter<'a, T> {
    index: usize,
    tree: &'a Tree<T>,
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.tree.inner.get(self.index) {
            Some(Entry::Occupied(node)) => Some(node),
            _ => None
        };
        self.index += 1;
        ret
    }
}

// -----------------------------------------------------------------------------
//     - Index -
// -----------------------------------------------------------------------------
impl<T> Index<NodeId> for Tree<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeId) -> &Self::Output {
        match &self.inner[index.0] {
            Entry::Occupied(val) => val,
            Entry::Vacant(_) => panic!("No such entry"),
        }
    }
}

impl<T> IndexMut<NodeId> for Tree<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        match &mut self.inner[index.0] {
            Entry::Occupied(val) => val,
            Entry::Vacant(_) => panic!("No such entry"),
        }
    }
}

// -----------------------------------------------------------------------------
//     - Node -
// -----------------------------------------------------------------------------
pub struct Node<T> {
    inner: T,
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub left: Option<NodeId>,
    pub right: Option<NodeId>,
}

impl<T> Node<T> {
    fn new(val: T, id: NodeId, parent: Option<NodeId>) -> Self {
        Self {
            inner: val,
            id,
            parent,
            left: None,
            right: None,
        }
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }

}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Node id: {} val: {:?}", self.id.0, self.inner)?;
        if let Some(left) = self.left {
            write!(f, " left: {:?}", left.0)?;
        }

        if let Some(right) = self.right {
            write!(f, " right: {:?}", right.0)?;
        }

        write!(f, " />")
    }
}
