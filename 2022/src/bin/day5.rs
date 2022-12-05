use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    nb: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        // discard 'move'
        words.next();
        let nb = words.next().unwrap().parse()?;
        // discard 'from'
        words.next();
        let from = words.next().unwrap().parse()?;
        // discord 'to'
        words.next();
        let to = words.next().unwrap().parse()?;

        Ok(Self { nb, from, to })
    }
}

#[derive(Debug)]
struct Crates<'a> {
    stacks: Vec<Vec<&'a char>>,
    instructions: Vec<Instruction>,
}

fn part1(crates: &Crates) {
    let mut stacks = crates.stacks.clone();

    crates.instructions.iter().for_each(|instruction| {
        let to_drain = stacks[instruction.from - 1].len() - instruction.nb;
        let crates_to_move: Vec<&char> = stacks[instruction.from - 1]
            .drain(to_drain..)
            .rev()
            .collect();
        stacks[instruction.to - 1].extend(crates_to_move);
    });

    let res: String = stacks
        .iter()
        .map(|chars| chars.last().unwrap_or(&&'\0'))
        .copied()
        .collect();

    println!(
        "The crates that end up on top of each stack with CrateMover 9000 are {}",
        res
    );
}

fn part2(crates: &Crates) {
    let mut stacks = crates.stacks.clone();

    crates.instructions.iter().for_each(|instruction| {
        let to_drain = stacks[instruction.from - 1].len() - instruction.nb;
        let crates_to_move: Vec<&char> = stacks[instruction.from - 1].drain(to_drain..).collect();
        stacks[instruction.to - 1].extend(crates_to_move);
    });

    let res: String = stacks
        .iter()
        .map(|chars| chars.last().unwrap_or(&&'\0'))
        .copied()
        .collect();

    println!(
        "The crates that end up on top of each stack are CrateMover 9001 are {}",
        res
    );
}

fn main() {
    let lines = lib::input_lines("input/day5.txt").collect::<Vec<_>>();
    let stacks_def: Vec<Vec<char>> = lines
        .iter()
        .take_while(|s| !s.starts_with(" 1"))
        .map(|s| {
            s.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chars| if chars[0] == '[' { chars[1] } else { '0' })
                .collect::<Vec<char>>()
        })
        .collect();
    let mut stacks: Vec<Vec<&char>> = Vec::with_capacity(stacks_def[0].len());
    for _ in 0..stacks_def[0].len() {
        stacks.push(Vec::new());
    }

    for stack_line in stacks_def.iter().rev() {
        for (i, cratee) in stack_line.iter().enumerate() {
            if cratee != &'0' {
                stacks[i].push(cratee);
            }
        }
    }

    let instructions: Vec<Instruction> = lines
        .iter()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect();

    let crates = Crates {
        stacks,
        instructions,
    };

    part1(&crates);
    part2(&crates);
}
