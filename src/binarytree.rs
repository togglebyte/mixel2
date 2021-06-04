use std::fmt;

#[derive(Debug)]
pub enum Node {
    Leaf(usize),
    Branch { left: Box<Node>, right: Box<Node> },
}

impl Node {
    fn find_node(&mut self, id: usize) -> Option<&mut Node> {
        match self {
            Node::Leaf(node_id) if *node_id == id => Some(self),
            Node::Leaf(node_id) => None,
            Node::Branch { left, right } => match left.find_node(id) {
                Some(node) => Some(node),
                None => match right.find_node(id) {
                    Some(node) => Some(node),
                    None => None,
                }
            }
        }
    }

    pub fn split(&mut self, id: usize, right: usize) {
        if let Some(node) = self.find_node(id) {
            match node {
                Node::Leaf(left) => {
                    *node = Node::Branch {
                        left: Box::new(Node::Leaf(*left)),
                        right: Box::new(Node::Leaf(right)),
                    };
                }
                Node::Branch { left, right } => panic!(),
            }
        }
    }

    fn collapse(&mut self, node_id: usize) -> bool {
        match self {
            Node::Leaf(id) =>  false,
            Node::Branch { left, right } => {
                match left.as_mut() {
                    Node::Leaf(id) if *id == node_id => {
                        // If the left node is removed then make the right node the parent
                        let new_parent = std::mem::replace(right.as_mut(), Node::Leaf(std::usize::MAX));
                        *self = new_parent;
                        true
                    }
                    _ => match right.as_ref() {
                        Node::Leaf(id) if *id == node_id => { 
                            // If the right node is removed then make the left node the parent
                            let new_parent = std::mem::replace(left.as_mut(), Node::Leaf(std::usize::MAX));
                            *self = new_parent;
                            true 
                        } 
                        _ => {
                            match left.collapse(node_id) {
                                true => false,
                                false => right.collapse(node_id)
                            }
                        }
                    }
                }
            }
        }
    }

}

impl Node {
    fn display(&self, level: usize, f: &mut fmt::Formatter<'_>) {
        match self {
            Node::Leaf(val) => { 
                write!(f, "{}{}\n", " ".repeat(level * 2), val); 
            }
            Node::Branch { left, right } => {
                write!(f, "{}\\\n", " ".repeat(level * 2)); 
                left.display(level + 1, f);
                right.display(level + 1, f);
            }
        }

    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(0, f);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collapse_nodes() {
        let mut tree = Node::Leaf(0);
        tree.split(0, 100);
        tree.split(100, 200);
        tree.split(200, 300);
        eprintln!("{}", tree);
        tree.collapse(0);
        eprintln!("{}", tree);
        // assert_eq!(expected, actual);
    }
}
