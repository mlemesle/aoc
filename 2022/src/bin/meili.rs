use std::{ops::Add, str::FromStr};

use lib::btree::{BTree, Key};

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

impl Add<House> for BTree<Option<String>> {
    type Output = Self;

    fn add(mut self, house: House) -> Self::Output {
        let mut current = self.get_root().unwrap().get_key();

        for direction in house.path {
            current = match direction {
                Direction::L => self.get_or_add_left(None, &current),
                Direction::R => self.get_or_add_right(None, &current),
            }
        }

        *self.get_by_key_mut(&current).unwrap().get_data_mut() = Some(house.kid);

        self
    }
}

fn get_kid_with_depth(
    tree: &BTree<Option<String>>,
    key: &Key,
    current_depth: usize,
) -> (String, usize) {
    let node = tree.get_by_key(key).unwrap();

    if let Some(kid) = node.get_data() {
        return (kid.into(), current_depth);
    }

    match (tree.get_left(key), tree.get_right(key)) {
        (None, None) => unreachable!("Leaf without kid?"),
        (None, Some(node)) | (Some(node), None) => {
            get_kid_with_depth(tree, &node.get_key(), current_depth)
        }
        (Some(left), Some(right)) => {
            let (left_kid, left_depth) =
                get_kid_with_depth(tree, &left.get_key(), current_depth + 1);
            let (right_kid, right_depth) =
                get_kid_with_depth(tree, &right.get_key(), current_depth + 1);

            match left_depth.cmp(&right_depth) {
                std::cmp::Ordering::Greater => (right_kid, right_depth),
                _ => (left_kid, left_depth),
            }
        }
    }
}

fn part1(tree: &BTree<Option<String>>, root: Key) {
    println!("{:#?}", get_kid_with_depth(tree, &root, 0));
}

fn main() -> Result<(), anyhow::Error> {
    let houses = lib::input::<House>("input/input.txt")?;
    let (tree, root) = BTree::new(None);
    let tree = houses.into_iter().fold(tree, |tree, house| tree.add(house));

    part1(&tree, root);

    Ok(())
}
