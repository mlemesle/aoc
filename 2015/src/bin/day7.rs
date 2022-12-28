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

impl Value {
    fn resolve(&self, wires: &mut HashMap<String, Data>) -> u16 {
        match self {
            Value::Number(n) => *n,
            Value::Wire(w) => resolve(w, wires),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Assign(Value),
    And(Value, Value),
    Or(Value, Value),
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
                    Operation::And(op_parts[0].parse()?, op_parts[2].parse()?),
                ),
                "OR" => Self(
                    target.into(),
                    Operation::Or(op_parts[0].parse()?, op_parts[2].parse()?),
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

#[derive(Debug, Clone)]
enum Data {
    Resolved(u16),
    ToResolve(Operation),
}

fn resolve(wire: &str, wires: &mut HashMap<String, Data>) -> u16 {
    let entry = wires.get(wire).cloned();
    match entry {
        Some(data) => match data {
            Data::Resolved(signal) => signal,
            Data::ToResolve(operation) => {
                let signal = match operation {
                    Operation::Assign(value) => value.resolve(wires),
                    Operation::And(value1, value2) => value1.resolve(wires) & value2.resolve(wires),
                    Operation::Or(value1, value2) => value1.resolve(wires) | value2.resolve(wires),
                    Operation::RShift(w, shift) => resolve(&w, wires) >> shift,
                    Operation::LShift(w, shift) => resolve(&w, wires) << shift,
                    Operation::Not(w) => !resolve(&w, wires),
                };
                wires.insert(wire.to_string(), Data::Resolved(signal));
                signal
            }
        },
        None => {
            unreachable!("there is a hole in the `wires` HashMap, wire {wire} does not exist ?")
        }
    }
}

fn part1(instructions: &mut HashMap<String, Data>) {
    let a = resolve("a", instructions);

    println!("Final signal on wire a is {a:?}");
}

fn part2(instructions: &mut HashMap<String, Data>) {
    let a = resolve("a", &mut instructions.clone());
    instructions.insert("b".to_string(), Data::Resolved(a));

    let new_a = resolve("a", instructions);

    println!("Super ultimate final signal on wire a is {new_a:?}");
}

fn main() -> anyhow::Result<()> {
    let mut instructions = lib::input::<Instruction>("input/day7.txt")?
        .into_iter()
        .map(|instruction| (instruction.0, Data::ToResolve(instruction.1)))
        .collect::<HashMap<String, Data>>();

    part1(&mut instructions.clone());
    part2(&mut instructions);

    Ok(())
}
