use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

#[derive(Debug, Clone)]
enum Value {
    Number(u16),
    Wire(String),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u16>()
            .map(Self::Number)
            .unwrap_or_else(|_| Self::Wire(s.into())))
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Assign(Value),
    And(String, String),
    Or(String, String),
    RShift(String, u16),
    LShift(String, u16),
    Not(String),
}

#[derive(Debug, Clone)]
struct Instruction(String, Operation);

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, target) = s
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("no -> in {s}"))?;

        let op_parts = op.split_whitespace().collect::<Vec<_>>();

        let instruction = match op_parts.len() {
            1 => Self(target.into(), Operation::Assign(op_parts[0].parse()?)),
            2 => Self(target.into(), Operation::Not(op_parts[1].into())),
            3 => match op_parts[1] {
                "AND" => Self(
                    target.into(),
                    Operation::And(op_parts[0].into(), op_parts[2].into()),
                ),
                "OR" => Self(
                    target.into(),
                    Operation::Or(op_parts[0].into(), op_parts[2].into()),
                ),
                "RSHIFT" => Self(
                    target.into(),
                    Operation::RShift(op_parts[0].into(), op_parts[2].parse()?),
                ),
                "LSHIFT" => Self(
                    target.into(),
                    Operation::LShift(op_parts[0].into(), op_parts[2].parse()?),
                ),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        Ok(instruction)
    }
}

fn part1(instructions: &mut HashMap<String, Operation>) {
    let a = wires.get("a").unwrap_or(&0);

    println!("Final signal on wire a is {a}");
}

fn main() -> anyhow::Result<()> {
    let mut wires = lib::input::<Instruction>("input/day7.txt")?
        .iter()
        .map(|instruction| (instruction.0, instruction.1))
        .collect::<HashMap<String, Operation>>();

    part1(&instructions);

    Ok(())
}
