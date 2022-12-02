use std::str::FromStr;

use anyhow::anyhow;

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(anyhow!("{} is not a valid Shape", s)),
        }
    }
}

struct Round {
    opponent_move: Shape,
    elf_move: String,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((opponent, should_play)) => Ok(Self {
                opponent_move: opponent.parse()?,
                elf_move: should_play.into(),
            }),
            None => todo!(),
        }
    }
}

#[derive(Copy, Clone)]
enum End {
    Win,
    Draw,
    Loss,
}

impl FromStr for End {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(anyhow!("{} is not a valid Shape", s)),
        }
    }
}

fn part1<'a>(rounds: impl Iterator<Item = &'a Round>) {
    let sum = rounds
        .map(|round| {
            (
                round.opponent_move,
                round
                    .elf_move
                    .parse::<Shape>()
                    .expect("Error parsing elf_move"),
            )
        })
        .map(|(opponent_move, elf_move)| match elf_move {
            Shape::Rock => {
                1 + match opponent_move {
                    Shape::Rock => 3,
                    Shape::Paper => 0,
                    Shape::Scissors => 6,
                }
            }
            Shape::Paper => {
                2 + match opponent_move {
                    Shape::Rock => 6,
                    Shape::Paper => 3,
                    Shape::Scissors => 0,
                }
            }
            Shape::Scissors => {
                3 + match opponent_move {
                    Shape::Rock => 0,
                    Shape::Paper => 6,
                    Shape::Scissors => 3,
                }
            }
        })
        .sum::<u32>();
    println!(
        "If the elf follows the strategy guide, his final score will be {}",
        sum
    );
}

fn part2<'a>(rounds: impl Iterator<Item = &'a Round>) {
    let sum = rounds
        .map(|round| {
            (
                round.opponent_move,
                round
                    .elf_move
                    .parse::<End>()
                    .expect("Error parsing elf_move"),
            )
        })
        .map(|(opponent_move, end)| match end {
            End::Win => {
                6 + match opponent_move {
                    Shape::Rock => 2,
                    Shape::Paper => 3,
                    Shape::Scissors => 1,
                }
            }
            End::Draw => {
                3 + match opponent_move {
                    Shape::Rock => 1,
                    Shape::Paper => 2,
                    Shape::Scissors => 3,
                }
            }
            End::Loss => {
                0 + match opponent_move {
                    Shape::Rock => 3,
                    Shape::Paper => 1,
                    Shape::Scissors => 2,
                }
            }
        })
        .sum::<u32>();
    println!(
        "If the elf follows the `real` strategy guide, his final score will be {}",
        sum
    );
}

fn main() {
    let rounds = lib::input::<Round>("input/day2.txt").collect::<Vec<_>>();
    part1(rounds.iter());
    part2(rounds.iter());
}
