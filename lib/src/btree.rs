pub type Key = usize;

#[derive(Debug)]
pub struct BTree<T> {
    nodes: Vec<Option<Node<T>>>,
    empty_cells: Vec<Key>,
}

impl<T> BTree<T> {
    pub fn new(data: T) -> (Self, Key) {
        (
            Self {
                nodes: vec![Some(Node::new(data, 0, None))],
                empty_cells: Vec::new(),
            },
            0,
        )
    }

    pub fn get_root(&self) -> Option<&Node<T>> {
        match self.nodes.get(0) {
            Some(node_opt) => node_opt.as_ref(),
            None => None,
        }
    }

    fn inner_add(&mut self, data: T, parent: Key) -> Key {
        match self.empty_cells.pop() {
            Some(index) => {
                let new_node = Some(Node::new(data, index, Some(parent)));
                unsafe { *self.nodes.get_unchecked_mut(index) = new_node };
                index
            }
            None => {
                let new_node_key = self.nodes.len();
                let new_node = Some(Node::new(data, new_node_key, Some(parent)));
                self.nodes.push(new_node);
                new_node_key
            }
        }
    }

    pub fn add_left(&mut self, data: T, parent: Key) -> Key {
        let new_node_key = self.inner_add(data, parent);
        if let Some(node) = self.get_node_unchecked_mut(parent) {
            node.left = Some(new_node_key);
        }
        new_node_key
    }

    pub fn get_or_add_left(&mut self, data: T, parent: Key) -> Key {
        match self.get_left(&parent) {
            Some(left) => left.get_key(),
            None => self.add_left(data, parent),
        }
    }

    pub fn add_right(&mut self, data: T, parent: Key) -> Key {
        let new_node_key = self.inner_add(data, parent);
        if let Some(node) = self.get_node_unchecked_mut(parent) {
            node.right = Some(new_node_key);
        }
        new_node_key
    }

    pub fn get_or_add_right(&mut self, data: T, parent: Key) -> Key {
        match self.get_right(&parent) {
            Some(right) => right.get_key(),
            None => self.add_right(data, parent),
        }
    }

    pub fn remove(&mut self, index: Key) {
        let (parent, node_left, node_right) = {
            let node = unsafe { self.nodes.get_unchecked(index).as_ref().unwrap() };
            (node.parent, node.left, node.right)
        };
        if let Some(left_index) = node_left {
            let left = unsafe { self.nodes.get_unchecked(left_index) };
            self.remove(left.as_ref().unwrap().key);
        }
        if let Some(right_index) = node_right {
            let right = unsafe { self.nodes.get_unchecked(right_index) };
            self.remove(right.as_ref().unwrap().key);
        }
        unsafe { *self.nodes.get_unchecked_mut(index) = None };

        if let Some(parent_node) = parent
            .and_then(|parent_key| self.get_node_unchecked_mut(parent_key))
            .as_mut()
        {
            if parent_node.right.is_some() && parent_node.right.unwrap() == index {
                parent_node.right = None;
            }
            if parent_node.left.is_some() && parent_node.left.unwrap() == index {
                parent_node.left = None;
            }
        };

        self.empty_cells.push(index);
    }

    pub fn get_by_key(&self, key: &Key) -> Option<&Node<T>> {
        self.nodes.get(*key).and_then(|node_opt| node_opt.as_ref())
    }

    pub fn get_by_key_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        self.nodes
            .get_mut(*key)
            .and_then(|node_opt| node_opt.as_mut())
    }

    pub fn get_left(&self, key: &Key) -> Option<&Node<T>> {
        self.nodes
            .get(*key)
            .and_then(|node_opt| node_opt.as_ref().and_then(|node| node.left))
            .and_then(|key| self.get_node_unchecked(key))
    }

    pub fn get_left_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        self.nodes
            .get(*key)
            .and_then(|node_opt| node_opt.as_ref().and_then(|node| node.left))
            .and_then(|key| self.get_node_unchecked_mut(key))
    }

    pub fn get_right(&self, key: &Key) -> Option<&Node<T>> {
        self.nodes
            .get(*key)
            .and_then(|node_opt| node_opt.as_ref().and_then(|node| node.right))
            .and_then(|key| self.get_node_unchecked(key))
    }

    pub fn get_right_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        self.nodes
            .get(*key)
            .and_then(|node_opt| node_opt.as_ref().and_then(|node| node.right))
            .and_then(|key| self.get_node_unchecked_mut(key))
    }

    pub fn get_parent(&self, key: &Key) -> Option<&Node<T>> {
        self.nodes
            .get(*key)
            .and_then(|node_opt| node_opt.as_ref().and_then(|node| node.parent))
            .and_then(|key| self.get_node_unchecked(key))
    }

    pub fn get_parent_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        self.nodes
            .get(*key)
            .and_then(|node_opt| node_opt.as_ref().and_then(|node| node.parent))
            .and_then(|key| self.get_node_unchecked_mut(key))
    }

    fn get_node_unchecked(&self, key: Key) -> Option<&Node<T>> {
        unsafe { self.nodes.get_unchecked(key).as_ref() }
    }

    fn get_node_unchecked_mut(&mut self, key: Key) -> Option<&mut Node<T>> {
        unsafe { self.nodes.get_unchecked_mut(key).as_mut() }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    key: Key,
    parent: Option<Key>,
    left: Option<Key>,
    right: Option<Key>,
}

impl<T> Node<T> {
    fn new(data: T, key: Key, parent: Option<Key>) -> Self {
        Self {
            data,
            key,
            parent,
            left: None,
            right: None,
        }
    }

    pub fn get_key(&self) -> Key {
        self.key
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[cfg(test)]
mod test {
    use crate::btree::Node;

    #[test]
    fn is_leaf() {
        let node1 = Node {
            data: (),
            key: 0,
            parent: None,
            left: None,
            right: None,
        };
        let node2 = Node {
            data: (),
            key: 0,
            parent: None,
            left: Some(0),
            right: None,
        };
        let node3 = Node {
            data: (),
            key: 0,
            parent: None,
            left: None,
            right: Some(1),
        };
        let node4 = Node {
            data: (),
            key: 0,
            parent: None,
            left: Some(0),
            right: Some(1),
        };

        assert_eq!(node1.is_leaf(), true);
        assert_eq!(node2.is_leaf(), false);
        assert_eq!(node3.is_leaf(), false);
        assert_eq!(node4.is_leaf(), false);
    }
}
