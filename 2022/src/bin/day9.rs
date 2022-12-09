use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    len: isize,
}

impl FromStr for Motion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let motion = match s.split_once(" ") {
            Some(("U", len)) => Self {
                direction: Direction::U,
                len: len.parse()?,
            },
            Some(("D", len)) => Self {
                direction: Direction::D,
                len: len.parse()?,
            },
            Some(("L", len)) => Self {
                direction: Direction::L,
                len: len.parse()?,
            },
            Some(("R", len)) => Self {
                direction: Direction::R,
                len: len.parse()?,
            },
            _ => Err(anyhow::anyhow!("{s} is invalid"))?,
        };

        Ok(motion)
    }
}

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position(isize, isize);

const RANGE: RangeInclusive<isize> = -1..=1;

impl Position {
    fn is_adjacent_to(&self, other: Self) -> bool {
        RANGE.contains(&(self.0 - other.0)) && RANGE.contains(&(self.1 - other.1))
    }

    fn go_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::U => self.1 += 1,
            Direction::D => self.1 -= 1,
            Direction::L => self.0 -= 1,
            Direction::R => self.0 += 1,
        }
    }

    fn join(&mut self, to_join: Self) {
        if !self.is_adjacent_to(to_join) {
            if self.0 > to_join.0 {
                self.0 -= 1;
            } else if self.0 < to_join.0 {
                self.0 += 1;
            }
            if self.1 > to_join.1 {
                self.1 -= 1;
            } else if self.1 < to_join.1 {
                self.1 += 1;
            }
        }
    }
}

struct Rope<const SIZE: usize>([Position; SIZE]);

impl<const SIZE: usize> Rope<SIZE> {
    fn new() -> Self {
        Self([Position::default(); SIZE])
    }

    fn get_head(&mut self) -> &mut Position {
        &mut self.0[0]
    }

    fn get_tail(&self) -> Position {
        self.0[SIZE - 1]
    }

    fn do_motion(&mut self, motion: &Motion, tail_positions: &mut HashSet<Position>) {
        for _ in 0..motion.len {
            self.get_head().go_direction(&motion.direction);

            for i in 0..SIZE - 1 {
                self.0[i + 1].join(self.0[i]);
            }
            tail_positions.insert(self.get_tail());
        }
    }
}

fn run<const ROPE_LEN: usize>(motions: &Vec<Motion>) -> usize {
    let mut rope = Rope::<ROPE_LEN>::new();
    let mut tail_positions = HashSet::new();
    for motion in motions {
        rope.do_motion(motion, &mut tail_positions);
    }

    tail_positions.len()
}

fn part1(motions: &Vec<Motion>) {
    println!(
        "The tail of the rope of length 2 visited {} positions at least once",
        run::<2>(motions)
    );
}

fn part2(motions: &Vec<Motion>) {
    println!(
        "The tail of the rope of length 10 visited {} positions at least once",
        run::<10>(motions)
    );
}

fn main() -> Result<(), anyhow::Error> {
    let motions = lib::input::<Motion>("input/day9.txt")?;

    part1(&motions);
    part2(&motions);

    Ok(())
}
