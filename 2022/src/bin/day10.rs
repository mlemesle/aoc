use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        match (split.next(), split.next()) {
            (Some("addx"), Some(n)) => Ok(Self::Addx(n.parse()?)),
            (Some("noop"), None) => Ok(Self::Noop),
            _ => Err(anyhow::anyhow!("{s} isn't a valid instruction")),
        }
    }
}

#[derive(Debug)]
struct Cpu {
    x: isize,
    cycle: usize,
    signal_cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 1,
            signal_cycle: 20,
        }
    }

    fn signal(&mut self) -> isize {
        if self.cycle == self.signal_cycle {
            self.signal_cycle += 40;
            self.x * self.cycle as isize
        } else {
            0
        }
    }

    fn compute_part1(&mut self, instruction: &Instruction) -> isize {
        match instruction {
            Instruction::Noop => {
                self.cycle += 1;
                self.signal()
            }
            Instruction::Addx(x) => {
                self.cycle += 1;
                let prev = self.signal();
                self.x += x;
                self.cycle += 1;
                prev + self.signal()
            }
        }
    }

    fn draw_pixel(&self) {
        let current_pixel = ((self.cycle - 1) % 40) as isize;

        if current_pixel == self.x - 1 || current_pixel == self.x || current_pixel == self.x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        if (self.cycle) % 40 == 0 {
            println!();
        }
    }

    fn compute_part2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.draw_pixel();
                self.cycle += 1;
            }
            Instruction::Addx(x) => {
                self.draw_pixel();
                self.cycle += 1;
                self.draw_pixel();
                self.cycle += 1;
                self.x += x;
            }
        }
    }
}

fn part1(instructions: &[Instruction]) {
    let mut cpu = Cpu::new();

    let sum: isize = instructions
        .iter()
        .map(|instruction| cpu.compute_part1(instruction))
        .sum();

    println!("The sum of these six signal strengths is {sum}");
}

fn part2(instructions: &[Instruction]) {
    let mut cpu = Cpu::new();

    instructions
        .iter()
        .for_each(|instruction| cpu.compute_part2(instruction));
}

fn main() -> Result<(), anyhow::Error> {
    let instructions = lib::input::<Instruction>("input/day10.txt")?;

    part1(&instructions);
    part2(&instructions);

    Ok(())
}
