use std::{ops::Add, str::FromStr};

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

#[derive(Default, Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    kid: Option<String>,
}

impl Node {
    fn get_or_insert_default(&mut self, direction: Direction) -> &mut Self {
        match direction {
            Direction::L => {
                if self.left.is_none() {
                    self.left = Some(Box::new(Node::default()));
                }

                self.left.as_mut().unwrap().as_mut()
            }
            Direction::R => {
                if self.right.is_none() {
                    self.right = Some(Box::new(Node::default()));
                }

                self.right.as_mut().unwrap().as_mut()
            }
        }
    }
}

impl Add<House> for Node {
    type Output = Node;

    fn add(mut self, rhs: House) -> Self::Output {
        let mut current = &mut self;

        for direction in rhs.path {
            current = current.get_or_insert_default(direction);
        }

        current.kid = Some(rhs.kid);

        self
    }
}

fn get_kid_with_depth(node: &Node, current_depth: usize) -> (&String, usize) {
    if let Some(kid) = &node.kid {
        return (kid, current_depth);
    }

    match (node.left.as_ref(), node.right.as_ref()) {
        (None, None) => unreachable!("Leaf without kid ?"),
        (None, Some(node)) | (Some(node), None) => get_kid_with_depth(node, current_depth),
        (Some(left), Some(right)) => {
            let (left_kid, left_depth) = get_kid_with_depth(left, current_depth + 1);
            let (right_kid, right_depth) = get_kid_with_depth(right, current_depth + 1);

            match left_depth.cmp(&right_depth) {
                std::cmp::Ordering::Greater => (right_kid, right_depth),
                _ => (left_kid, left_depth),
            }
        }
    }
}

fn get_kid_with_depth2(node: &mut Node, current_depth: usize) -> (&String, usize) {
    if let Some(kid) = &node.kid {
        return (kid, current_depth);
    }

    match (node.left.as_ref(), node.right.as_ref()) {
        (None, None) => unreachable!("Leaf without kid ?"),
        (None, Some(node)) | (Some(node), None) => get_kid_with_depth(node, current_depth),
        (Some(left), Some(right)) => {
            let (left_kid, left_depth) = get_kid_with_depth(left, current_depth + 1);
            let (right_kid, right_depth) = get_kid_with_depth(right, current_depth + 1);

            match left_depth.cmp(&right_depth) {
                std::cmp::Ordering::Greater => (right_kid, right_depth),
                _ => (left_kid, left_depth),
            }
        }
    }
}

fn part1(tree: &Node) {
    println!("{:#?}", get_kid_with_depth(tree, 0));
}

fn part2(tree: &mut Node) {}

fn main() {
    let tree =
        lib::input::<House>("input/input.txt").fold(Node::default(), |tree, house| tree.add(house));

    part1(&tree);
}
