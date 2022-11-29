use std::{
    ops::{Add, AddAssign},
    str::FromStr,
};

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
            Direction::L => match self.left {
                Some(node) => node,
                None => {
                    self.left = Some(Box::new(Node::default()));
                    self.left.as_mut().unwrap().as_mut()
                }
            },
            Direction::R => match self.right.as_mut() {
                Some(node) => node,
                None => {
                    self.right = Some(Box::new(Node::default()));
                    &mut self.right.unwrap()
                }
            },
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

fn main() {
    let res =
        lib::input::<House>("input/input.txt").fold(Node::default(), |tree, house| tree.add(house));

    println!("{:?}", res);
}
