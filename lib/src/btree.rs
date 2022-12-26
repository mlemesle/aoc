//! BTree module. This module aims to create a safe, fast and intelligent binary tree implementation.

/// Represents a key in BTree. Used in parameters and function output.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Key(usize);

impl Key {
    /// Creates a new key from an usize.
    fn new(k: usize) -> Self {
        Self(k)
    }
}

/// A smart growing and fast BTree.
#[derive(Debug)]
pub struct BTree<T> {
    nodes: Vec<Option<Node<T>>>,
    empty_cells: Vec<Key>,
}

impl<T> BTree<T> {
    /// Creates a new Btree. `data` is used as root.
    pub fn new(data: T) -> (Self, Key) {
        let key = Key::new(0);
        (
            Self {
                nodes: vec![Some(Node::new(data, key.clone(), None))],
                empty_cells: Vec::new(),
            },
            key,
        )
    }

    /// Retrieves the root of the BTree, if any.
    pub fn get_root(&self) -> Option<&Node<T>> {
        match self.nodes.get(0) {
            Some(node_opt) => node_opt.as_ref(),
            None => None,
        }
    }

    /// Private function that adds a node to the BTree, using a previously emptied space if any,
    /// or creates a new one if needs be.
    fn inner_add(&mut self, data: T, parent: &Key) -> Key {
        match self.empty_cells.pop() {
            Some(index) => {
                let new_node = Some(Node::new(data, index.clone(), Some(parent.clone())));
                unsafe { *self.nodes.get_unchecked_mut(index.0) = new_node };
                index
            }
            None => {
                let new_node_key = Key::new(self.nodes.len());
                let new_node = Some(Node::new(data, new_node_key.clone(), Some(parent.clone())));
                self.nodes.push(new_node);
                new_node_key
            }
        }
    }

    /// Adds a left child containing `data` to the node referenced by `parent`.
    /// The Key of the new node is returned.
    pub fn add_left(&mut self, data: T, parent: &Key) -> Key {
        let new_node_key = self.inner_add(data, parent);
        if let Some(node) = self.get_node_unchecked_mut(parent) {
            node.left = Some(new_node_key.clone());
        }
        new_node_key
    }

    /// Retrieves the key of the left child referenced by `parent`.
    /// If `parent` doesn't have a left child, it is inserted, containing `data` and its key
    /// is returned.
    pub fn get_or_add_left(&mut self, data: T, parent: &Key) -> Key {
        match self.get_left(parent) {
            Some(left) => left.get_key(),
            None => self.add_left(data, parent),
        }
    }

    /// Adds a right child containing `data` to the node referenced by `parent`.
    /// The Key of the new node is returned.
    pub fn add_right(&mut self, data: T, parent: &Key) -> Key {
        let new_node_key = self.inner_add(data, parent);
        if let Some(node) = self.get_node_unchecked_mut(parent) {
            node.right = Some(new_node_key.clone());
        }
        new_node_key
    }

    /// Retrieves the key of the right child referenced by `parent`.
    /// If `parent` doesn't have a right child, it is inserted, containing `data` and its key
    /// is returned.
    pub fn get_or_add_right(&mut self, data: T, parent: &Key) -> Key {
        match self.get_right(parent) {
            Some(right) => right.get_key(),
            None => self.add_right(data, parent),
        }
    }

    /// Removes a node from the BTree. If the node has childs, the remove process is called recursively.
    pub fn remove(&mut self, key: Key) {
        let (parent, node_left, node_right) = {
            let node = unsafe { self.nodes.get_unchecked(key.0).as_ref().unwrap() };
            (node.parent.clone(), node.left.clone(), node.right.clone())
        };
        if let Some(left_key) = node_left {
            let left = unsafe { self.nodes.get_unchecked(left_key.0) };
            self.remove(left.as_ref().unwrap().key.clone());
        }
        if let Some(right_key) = node_right {
            let right = unsafe { self.nodes.get_unchecked(right_key.0) };
            self.remove(right.as_ref().unwrap().key.clone());
        }
        unsafe { *self.nodes.get_unchecked_mut(key.0) = None };

        if let Some(parent_node) = parent
            .and_then(|parent_key| self.get_node_unchecked_mut(&parent_key))
            .as_mut()
        {
            if parent_node.right.is_some() && parent_node.right.as_ref().unwrap() == &key {
                parent_node.right = None;
            }
            if parent_node.left.is_some() && parent_node.left.as_ref().unwrap() == &key {
                parent_node.left = None;
            }
        };

        self.empty_cells.push(key);
    }

    /// Get a reference to a Node from BTree.
    pub fn get_by_key(&self, key: &Key) -> Option<&Node<T>> {
        self.nodes.get(key.0).and_then(|node_opt| node_opt.as_ref())
    }

    /// Get a mutable reference to a Node from BTree.
    pub fn get_by_key_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        self.nodes
            .get_mut(key.0)
            .and_then(|node_opt| node_opt.as_mut())
    }

    /// Get a reference to the left child of the Node referred by `key`, if any.
    pub fn get_left(&self, key: &Key) -> Option<&Node<T>> {
        unsafe {
            self.nodes
                .get_unchecked(key.0)
                .as_ref()
                .and_then(|node| node.left.clone())
                .and_then(|left_key| self.get_node_unchecked(&left_key))
        }
    }

    /// Get a mutable reference to the left child of the Node referred by `key`, if any.
    pub fn get_left_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        unsafe {
            self.nodes
                .get_unchecked(key.0)
                .as_ref()
                .and_then(|node| node.left.clone())
                .and_then(|left_key| self.get_node_unchecked_mut(&left_key))
        }
    }

    /// Get a reference to the right child of the Node referred by `key`, if any.
    pub fn get_right(&self, key: &Key) -> Option<&Node<T>> {
        unsafe {
            self.nodes
                .get_unchecked(key.0)
                .as_ref()
                .and_then(|node| node.right.clone())
                .and_then(|right_key| self.get_node_unchecked(&right_key))
        }
    }

    /// Get a mutable reference to the right child of the Node referred by `key`, if any.
    pub fn get_right_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        unsafe {
            self.nodes
                .get_unchecked(key.0)
                .as_ref()
                .and_then(|node| node.right.clone())
                .and_then(|right_key| self.get_node_unchecked_mut(&right_key))
        }
    }

    /// Get a reference to the parent of the Node referred by `key`, if any.
    pub fn get_parent(&self, key: &Key) -> Option<&Node<T>> {
        unsafe {
            self.nodes
                .get_unchecked(key.0)
                .as_ref()
                .and_then(|node| node.parent.clone())
                .and_then(|parent_key| self.get_node_unchecked(&parent_key))
        }
    }

    /// Get a mutable reference to the parent of the Node referred by `key`, if any.
    pub fn get_parent_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        unsafe {
            self.nodes
                .get_unchecked(key.0)
                .as_ref()
                .and_then(|node| node.parent.clone())
                .and_then(|parent_key| self.get_node_unchecked_mut(&parent_key))
        }
    }

    /// Get a reference to a Node from BTree.
    fn get_node_unchecked(&self, key: &Key) -> Option<&Node<T>> {
        unsafe { self.nodes.get_unchecked(key.0).as_ref() }
    }

    /// Get a mutable reference to a Node from BTree.
    fn get_node_unchecked_mut(&mut self, key: &Key) -> Option<&mut Node<T>> {
        unsafe { self.nodes.get_unchecked_mut(key.0).as_mut() }
    }
}

/// A Node, for BTree, easy to use.
#[derive(Debug)]
pub struct Node<T> {
    data: T,
    key: Key,
    parent: Option<Key>,
    left: Option<Key>,
    right: Option<Key>,
}

impl<T> Node<T> {
    /// Creates a new Node.
    fn new(data: T, key: Key, parent: Option<Key>) -> Self {
        Self {
            data,
            key,
            parent,
            left: None,
            right: None,
        }
    }

    /// Get the Key associated to the Node.
    pub fn get_key(&self) -> Key {
        self.key.clone()
    }

    /// Retrieves a reference to the data owned by the Node.
    pub fn get_data(&self) -> &T {
        &self.data
    }

    /// Retrieves a mutable reference to the data owned by the Node.
    pub fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Tells if the Node is a leaf. A Node is a leaf if it has not childs.
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[cfg(test)]
mod test {
    use crate::btree::{Key, Node};

    #[test]
    fn is_leaf() {
        let node1 = Node {
            data: (),
            key: Key::new(0),
            parent: None,
            left: None,
            right: None,
        };
        let node2 = Node {
            data: (),
            key: Key::new(0),
            parent: None,
            left: Some(Key::new(0)),
            right: None,
        };
        let node3 = Node {
            data: (),
            key: Key::new(0),
            parent: None,
            left: None,
            right: Some(Key::new(1)),
        };
        let node4 = Node {
            data: (),
            key: Key::new(0),
            parent: None,
            left: Some(Key::new(0)),
            right: Some(Key::new(1)),
        };

        assert!(node1.is_leaf());
        assert!(!node2.is_leaf());
        assert!(!node3.is_leaf());
        assert!(!node4.is_leaf());
    }
}
