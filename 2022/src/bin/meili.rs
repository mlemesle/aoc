use std::{ops::Add, str::FromStr};

type Key = usize;

#[derive(Default, Debug)]
struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    kid: Option<String>,
}

impl Node {
    fn is_empty(&self) -> bool {
        self.is_leaf() && self.kid.is_none()
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[derive(Default, Debug)]
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn push(&mut self, node: Node) -> Key {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    fn get(&self, key: Key) -> &Node {
        self.nodes.get(key).unwrap()
    }

    fn get_mut(&mut self, key: Key) -> &mut Node {
        self.nodes.get_mut(key).unwrap()
    }

    fn get_or_insert_default(&mut self, index: Key, direction: Direction) -> Key {
        let Node { left, right, .. } = self.get(index);

        match direction {
            Direction::L => match *left {
                Some(left) => {
                    self.get_mut(left).parent = Some(index);
                    left
                }
                None => {
                    let child = self.push(Node {
                        parent: Some(index),
                        left: None,
                        right: None,
                        kid: None,
                    });
                    self.get_mut(index).left = Some(child);
                    child
                }
            },
            Direction::R => match *right {
                Some(right) => {
                    self.get_mut(right).parent = Some(index);
                    right
                }
                None => {
                    let child = self.push(Node {
                        parent: Some(index),
                        left: None,
                        right: None,
                        kid: None,
                    });
                    self.get_mut(index).right = Some(child);
                    child
                }
            },
        }
    }

    /// Removes parent if empty, but does not remove children :)
    fn remove(&mut self, key: Key) -> Option<Key> {
        let node = self.nodes.remove(key);

        assert!(node.left.is_none());
        assert!(node.right.is_none());

        if let Some(parent_key) = node.parent {
            let parent = self.get_mut(parent_key);

            if parent.left == Some(key) {
                parent.left = None;
            }

            if parent.right == Some(key) {
                parent.right = None;
            }

            if parent.is_empty() {
                self.remove(parent_key)
            } else {
                Some(parent_key)
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Direction {
    R,
    L,
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

impl Add<House> for Tree {
    type Output = Self;

    fn add(mut self, house: House) -> Self::Output {
        let mut current = 0;

        for direction in house.path {
            current = self.get_or_insert_default(current, direction);
        }

        self.get_mut(current).kid = Some(house.kid);

        self
    }
}

fn get_kid_with_depth(tree: &Tree, key: Key, current_depth: usize) -> (&String, usize) {
    let node = tree.get(key);

    if let Some(kid) = &node.kid {
        return (kid, current_depth);
    }

    match (node.left.as_ref(), node.right.as_ref()) {
        (None, None) => unreachable!("Leaf without kid ?"),
        (None, Some(&node)) | (Some(&node), None) => get_kid_with_depth(tree, node, current_depth),
        (Some(&left), Some(&right)) => {
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
    let node = tree.get(key);

    if node.kid.is_some() {
        // kid = None
        // Si feuille remove
        tree.get_mut(key).kid = None;

        if tree.get(key).is_leaf() {
            tree.remove(key);
        }

        return current_depth;
    }

    match (node.left, node.right, node.parent) {
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

fn part1(tree: &Tree) {
    println!("{:#?}", get_kid_with_depth(tree, 0, 0));
}

fn part2(tree: &mut Tree) {
    println!("{:#?}", get_kid_with_depth_2(tree, 0, 0, 0));
}

fn main() {
    let houses = lib::input::<House>("input/input.txt");
    let mut tree = {
        let mut tree = Tree::default();
        tree.nodes.push(Node::default());

        houses.fold(tree, |tree, house| tree.add(house))
    };

    part1(&tree);
    part2(&mut tree);
}
