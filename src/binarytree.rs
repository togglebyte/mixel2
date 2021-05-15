use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

use nightmaregl::pixels::{Pixel, Pixels};
use nightmaregl::texture::{Format, Texture};
use nightmaregl::{Position, Size, Sprite};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct NodeId(usize);

// -----------------------------------------------------------------------------
//     - Tree entry -
// -----------------------------------------------------------------------------
#[derive(Debug)]
enum Entry<T> {
    Occupied(Node<T>),
    Vacant(Option<NodeId>),
}

// -----------------------------------------------------------------------------
//     - Tree -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Tree<T> {
    inner: Vec<Entry<T>>,
    next: Option<NodeId>,
}

impl<T> Tree<T> {
    // TODO: remove this
    pub fn len(&self) -> usize {
        self.inner.iter().filter_map(|entry| match entry {
            Entry::Occupied(_) => Some(()),
            _ => None
        }).count()
    }

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

    pub fn sibling(&self, node: NodeId) -> Option<NodeId> {
        let parent = self[node].parent?;
        match self[parent].left {
            Some(left) if left == node => self[parent].right,
            _ => self[parent].left
        }
    }

    pub fn get(&self, node_id: NodeId) -> Option<&Node<T>> {
        match self.inner.get(node_id.0) {
            Some(Entry::Occupied(node)) => Some(node),
            Some(Entry::Vacant(_)) | None => None,
        }
    }

    /// Remove the specific node.
    /// This will panic if either the root node is removed
    /// or a vacant entry is removed.
    ///
    /// This will return the removed node,
    /// however all the children will be removed and
    /// won't be returned.
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

    pub fn collapse_into_parent(&mut self, node_id: NodeId) {
        // Get the parent and the grand parent id
        let (parent_id, grand_parent_id) = {
            match *&self[node_id].parent {
                Some(p) => {
                    {
                        // Remove children from parent
                        let p = &mut self[p];
                        p.left = None;
                        p.right = None;
                    }
                    match *&self[p].parent {
                        Some(grandpa) => (p, grandpa),
                        None => return,
                    }
                }
                None => return,
            }
        };

        // Remove the old parent...
        self.remove(parent_id);
        // ... and set the grand parent as the parent
        self[node_id].parent = Some(grand_parent_id);

        // Replace the old parent as a child of the grand parent
        // with the node id
        let mut grandpa = &mut self[grand_parent_id];

        match (grandpa.left, grandpa.right) {
            (Some(ref mut left), _) if *left == parent_id => {
                *left = node_id;
            }
            (_, Some(ref mut right)) if *right == parent_id => {
                *right = node_id;
            }
            (None, _) => {
                grandpa.left = Some(node_id);
            }
            (_, None) => {
                grandpa.right = Some(node_id);
            }
            _ => { 
                eprintln!("gramps be all like: {:?}", grandpa.id);
            }
        }
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
            tree: self,
        }
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
            _ => None,
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
        if let Some(parent) = self.parent {
            write!(f, " parent: {:?}", parent.0)?;
        }

        if let Some(left) = self.left {
            write!(f, " left: {:?}", left.0)?;
        }

        if let Some(right) = self.right {
            write!(f, " right: {:?}", right.0)?;
        }

        write!(f, " />")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_sure_its_removed() {
        let mut tree = Tree::new(1u32);
        let root = tree.root_id();
        tree.insert_left(root, 2);
        let right = tree.insert_right(root, 3);

        let actual = tree.len();
        let expected = 3;
        assert_eq!(expected, actual);

        tree.remove(right);

        let actual = tree.len();
        let expected = 2;
        assert_eq!(expected, actual);
    }

    #[test]
    fn collapse() {
        let mut tree = Tree::new(1u32);
        let root = tree.root_id();
            let left = tree.insert_left(root, 2);
            let right = tree.insert_right(root, 3);
                let right_left = tree.insert_left(right, 4); // remove this guy
                let right_right = tree.insert_right(right, 5); // this becomes `right`

        tree.remove(right_left);
        tree.collapse_into_parent(right_right);
        eprintln!("{:#?}", tree);

        tree.remove(left);
        tree.collapse_into_parent(right);
    }
}
