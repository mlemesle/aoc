use std::str::FromStr;

use lib::{grid::Grid, position::Position};

#[derive(Debug)]
enum Instruction {
    Toggle,
    TurnOn,
    TurnOff,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "toggle" => Ok(Self::Toggle),
            "turn on" => Ok(Self::TurnOn),
            "turn off" => Ok(Self::TurnOff),
            _ => Err(anyhow::anyhow!("{s} isn't a valid instruction")),
        }
    }
}

#[derive(Debug)]
struct InstrAndPos {
    instruction: Instruction,
    top_left: Position,
    bottom_right: Position,
}

impl FromStr for InstrAndPos {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, bottom_right) = s
            .split_once(" through ")
            .ok_or_else(|| anyhow::anyhow!("{s}"))?;

        let (start, top_left) = start
            .rsplit_once(' ')
            .ok_or_else(|| anyhow::anyhow!("{start}"))?;

        let instruction = start.parse()?;
        let (top_left1, top_left2) = top_left.split_once(',').unwrap();
        let (bottom_right1, bottom_right2) = bottom_right.split_once(',').unwrap();

        Ok(Self {
            instruction,
            top_left: Position::new(top_left1.parse()?, top_left2.parse()?),
            bottom_right: Position::new(bottom_right1.parse()?, bottom_right2.parse()?),
        })
    }
}

fn part1(instr_and_pos: &[InstrAndPos]) -> anyhow::Result<()> {
    let mut grid = Grid::try_from((vec![false; 1000 * 1000], 1000))?;

    for instr in instr_and_pos {
        let func = match instr.instruction {
            Instruction::Toggle => |b: &mut bool| *b = !*b,
            Instruction::TurnOn => |b: &mut bool| *b = true,
            Instruction::TurnOff => |b: &mut bool| *b = false,
        };
        grid.iter_rect_mut(instr.top_left, instr.bottom_right)?
            .for_each(|b| func(b));
    }

    let lit_lights = grid.iter().filter(|&b| *b).count();

    println!("{lit_lights} are lit.");

    Ok(())
}

fn part2(instr_and_pos: &[InstrAndPos]) -> anyhow::Result<()> {
    let mut grid = Grid::try_from((vec![0usize; 1000 * 1000], 1000))?;

    for instr in instr_and_pos {
        let func = match instr.instruction {
            Instruction::Toggle => |u: &mut usize| *u += 2,
            Instruction::TurnOn => |u: &mut usize| *u += 1,
            Instruction::TurnOff => |u: &mut usize| *u = u.saturating_sub(1),
        };
        grid.iter_rect_mut(instr.top_left, instr.bottom_right)?
            .for_each(|b| func(b));
    }

    let total_brightness: usize = grid.iter().sum();

    println!("Total brightness is {total_brightness}.");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let instr_and_pos = lib::input::<InstrAndPos>("input/day6.txt")?;

    part1(&instr_and_pos)?;
    part2(&instr_and_pos)?;

    Ok(())
}
