use std::str::FromStr;

#[derive(Debug)]
struct Rucksack(String, String);

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_compartment, second_compartment) = s.split_at(s.len() / 2);

        Ok(Rucksack(
            first_compartment.into(),
            second_compartment.into(),
        ))
    }
}

impl Rucksack {
    fn get_common_letter(&self) -> char {
        self.0
            .chars()
            .filter(|&c| self.1.contains(c))
            .next()
            .unwrap()
    }

    fn get_common_letter_multi(&self, second: &Rucksack, third: &Rucksack) -> char {
        self.0
            .chars()
            .filter(|&c| {
                (second.0.contains(c) || second.1.contains(c))
                    && (third.0.contains(c) || third.1.contains(c))
            })
            .next()
            .or_else(|| {
                self.1
                    .chars()
                    .filter(|&c| {
                        (second.0.contains(c) || second.1.contains(c))
                            && (third.0.contains(c) || third.1.contains(c))
                    })
                    .next()
            })
            .unwrap()
    }
}

fn get_value(c: char) -> usize {
    match c {
        ('a'..='z') => c as usize - 96,
        ('A'..='Z') => c as usize - 64 + 26,
        _ => unreachable!("Got {} and it's an error", c),
    }
}

fn part1<'a>(rucksacks: impl Iterator<Item = &'a Rucksack>) {
    let sum = rucksacks
        .map(|rucksack| rucksack.get_common_letter())
        .map(|letter| get_value(letter))
        .sum::<usize>();

    println!("The sum of the priorities of those item types is {}", sum);
}

fn part2(rucksacks: Vec<Rucksack>) {
    let sum = rucksacks
        .chunks_exact(3)
        .map(|chunk| chunk[0].get_common_letter_multi(&chunk[1], &chunk[2]))
        .map(|c| get_value(c))
        .sum::<usize>();

    println!("The sum of the priorities of those item types is {}", sum);
}

fn main() {
    let rucksacks = lib::input::<Rucksack>("input/day3.txt").collect::<Vec<_>>();

    part1(rucksacks.iter());
    part2(rucksacks);
}
