use std::{ops::Add, str::FromStr};

use tree::*;
mod tree {
    pub use lib::slutmap::Key;

    use super::Direction;
    use lib::slutmap::Slutmap;

    #[derive(Clone, Default, Debug)]
    pub struct Node {
        parent: Option<Key>,
        left: Option<Key>,
        right: Option<Key>,
        kid: Option<String>,
    }

    impl Node {
        pub fn parent(&self) -> Option<Key> {
            self.parent
        }

        pub fn left(&self) -> Option<Key> {
            self.left
        }

        pub fn right(&self) -> Option<Key> {
            self.right
        }

        pub fn kid(&self) -> Option<&str> {
            self.kid.as_deref()
        }

        pub fn is_orphan(&self) -> bool {
            self.parent.is_none()
        }

        pub fn is_leaf(&self) -> bool {
            self.left.is_none() && self.right.is_none()
        }
    }

    #[derive(Clone, Default, Debug)]
    pub struct Tree(Slutmap<Node>);

    impl Tree {
        pub fn new() -> Self {
            Self(Slutmap::new())
        }

        pub fn with_root() -> (Self, Key) {
            let mut tree = Self::default();
            let key = tree.0.insert(Node::default());
            (tree, key)
        }

        pub fn get(&self, key: Key) -> Option<&Node> {
            self.0.get(key)
        }

        pub fn kid_mut(&mut self, key: Key) -> Option<&mut Option<String>> {
            self.0.get_mut(key).map(|node| &mut node.kid)
        }

        pub fn ensure(&mut self, key: Key, direction: Direction) -> Option<Key> {
            match direction {
                Direction::L => self.ensure_left(key),
                Direction::R => self.ensure_right(key),
            }
        }

        pub fn ensure_left(&mut self, key: Key) -> Option<Key> {
            match self.0.get(key).map(|node| node.left) {
                Some(Some(left)) => Some(left),
                Some(None) => {
                    let left = self.0.insert(Node::default());
                    self.0.get_mut(key).expect("Unreachable: invalid key").left = Some(left);
                    Some(left)
                }
                None => None,
            }
        }

        pub fn ensure_right(&mut self, key: Key) -> Option<Key> {
            match self.0.get(key).map(|node| node.right) {
                Some(Some(right)) => Some(right),
                Some(None) => {
                    let right = self.0.insert(Node::default());
                    self.0.get_mut(key).expect("Unreachable: invalid key").right = Some(right);
                    Some(right)
                }
                None => None,
            }
        }

        pub fn remove(&mut self, key: Key) -> Option<Key> {
            fn remove(tree: &mut Tree, key: Key) {
                tree.0.remove(key).map(|node| {
                    node.left.map(|left| remove(tree, left));
                    node.right.map(|right| remove(tree, right));
                });
            }

            if let Some(parent_key) = self.0.get(key).and_then(Node::parent) {
                let parent_node = self.0.get_mut(parent_key).expect("Parent exists");

                if parent_node.left == Some(key) {
                    parent_node.left = None;
                }

                if parent_node.right == Some(key) {
                    parent_node.right = None;
                }

                remove(self, key);

                Some(parent_key)
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    L,
    R,
}

#[derive(Debug)]
struct House {
    kid: String,
    path: Vec<Direction>,
}

impl FromStr for House {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" - ")
            .map(|(kid, house)| {
                let path = house
                    .chars()
                    .map(|char| match char {
                        'R' => Ok(Direction::R),
                        'L' => Ok(Direction::L),
                        _ => Err(anyhow::anyhow!("Got {char} in house path")),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(House {
                    kid: String::from(kid),
                    path,
                })
            })
            .ok_or_else(|| anyhow::anyhow!("Error deserializing"))?
    }
}

impl Add<House> for (Tree, Key) {
    type Output = Self;

    fn add(self, house: House) -> Self::Output {
        let (mut tree, key) = self;
        let mut current = key;

        for direction in house.path {
            current = tree.ensure(current, direction).unwrap();
        }

        *tree.kid_mut(current).unwrap() = Some(house.kid);

        (tree, key)
    }
}

fn get_kid_with_depth(tree: &Tree, key: Key, current_depth: usize) -> (&str, usize) {
    let node = tree.get(key).unwrap();

    if let Some(kid) = node.kid() {
        return (kid, current_depth);
    }

    match (node.left(), node.right()) {
        (None, None) => unreachable!("Leaf without kid?"),
        (None, Some(node)) | (Some(node), None) => get_kid_with_depth(tree, node, current_depth),
        (Some(left), Some(right)) => {
            let (left_kid, left_depth) = get_kid_with_depth(tree, left, current_depth + 1);
            let (right_kid, right_depth) = get_kid_with_depth(tree, right, current_depth + 1);

            match left_depth.cmp(&right_depth) {
                std::cmp::Ordering::Greater => (right_kid, right_depth),
                _ => (left_kid, left_depth),
            }
        }
    }
}

fn get_kid_with_depth_2(tree: &mut Tree, key: Key, prev: Key, current_depth: usize) -> usize {
    let node = tree.get(key).unwrap();

    if node.kid().is_some() {
        // kid = None
        // Si feuille remove
        *tree.kid_mut(key).unwrap() = None;

        if tree.get(key).unwrap().is_leaf() {
            tree.remove(key);
        }

        return current_depth;
    }

    match (node.left(), node.right(), node.parent()) {
        (None, None, _) => unreachable!(),
        (None, Some(node), None) | (Some(node), None, None) => {
            get_kid_with_depth_2(tree, node, key, current_depth)
        }
        (Some(node1), None, Some(node2))
        | (None, Some(node1), Some(node2))
            // BUG +1 only when left AND right
        | (Some(node1), Some(node2), None) => match (node1 == prev, node2 == prev) {
            (true, true) => unreachable!("??????"),
            (false, true) => get_kid_with_depth_2(tree, node1, key, current_depth + 1),
            (true, false) => get_kid_with_depth_2(tree, node2, key, current_depth + 1),
            (false, false) => {
                let left = get_kid_with_depth_2(tree, node1, key, current_depth + 1);
                let right = get_kid_with_depth_2(tree, node2, key, current_depth + 1);

                match left.cmp(&right) {
                    std::cmp::Ordering::Greater => right,
                    _ => left,
                }
            }
        },
        (Some(left), Some(right), Some(parent)) => {
            (left != prev).then(|| get_kid_with_depth_2(tree, left, key, current_depth + 1));
            (right != prev).then(|| get_kid_with_depth_2(tree, right, key, current_depth + 1));
            (parent != prev).then(|| get_kid_with_depth_2(tree, parent, key, current_depth + 1));

            todo!()
        }
    }
}

fn part1(tree: &Tree, root: Key) {
    println!("{:#?}", get_kid_with_depth(tree, root, 0));
}

fn part2(tree: &mut Tree, root: Key) {
    println!("{:#?}", get_kid_with_depth_2(tree, root, root, 0));
}

fn main() {
    let houses = lib::input::<House>("input/input.txt");
    let (mut tree, root) = houses.fold(Tree::with_root(), |tree, house| tree.add(house));

    part1(&tree, root);
    part2(&mut tree, root);
}
