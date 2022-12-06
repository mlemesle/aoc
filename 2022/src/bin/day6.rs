use std::{collections::HashSet, str::FromStr};

struct Stream(Vec<char>);

impl FromStr for Stream {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

fn run(stream: &Stream, marker_len: usize) {
    let index = stream
        .0
        .windows(marker_len)
        .enumerate()
        .filter(|(_, chars)| chars.iter().collect::<HashSet<_>>().len() == marker_len)
        .nth(0)
        .map(|(i, _)| i + marker_len)
        .unwrap();

    println!(
        "{index} characters need to be processed before the first start-of-packet marker of len {marker_len} is detected"
    );
}

fn part1(stream: &Stream) {
    run(stream, 4);
}

fn part2(stream: &Stream) {
    run(stream, 14);
}

fn main() {
    let stream = lib::input::<Stream>("input/day6.txt").nth(0).unwrap();

    part1(&stream);
    part2(&stream);
}
