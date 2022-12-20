use std::{collections::HashMap, iter::once};

use lib::{direction::Direction, error::LibResult, position::Position};

fn part1(directions: &Vec<Direction>) {
    let visited_houses: HashMap<Position, usize> = once(Position::new(0, 0))
        .chain(
            directions
                .iter()
                .scan(Position::default(), |position, direction| {
                    position
                        .apply_direction(&direction)
                        .and_then(|_| Ok(*position))
                        .ok()
                }),
        )
        .fold(HashMap::new(), |mut map, position| {
            *map.entry(position).or_default() += 1;
            map
        });

    let houses_visited = visited_houses.values().count();

    println!("Thanks to Santa, {houses_visited} houses received at least one present.");
}

fn part2(directions: &Vec<Direction>) {
    let visited_houses: HashMap<Position, usize> = once((Position::new(0, 0), Position::new(0, 0)))
        .chain(directions.chunks_exact(2).scan(
            (Position::new(0, 0), Position::new(0, 0)),
            |(santa, robot), directions| {
                santa
                    .apply_direction(&directions[0])
                    .and_then(|_| Ok(*santa))
                    .ok()
                    .zip(
                        robot
                            .apply_direction(&directions[1])
                            .and_then(|_| Ok(*robot))
                            .ok(),
                    )
            },
        ))
        .fold(HashMap::new(), |mut map, (santa, robot)| {
            *map.entry(santa).or_default() += 1;
            *map.entry(robot).or_default() += 1;

            map
        });

    let houses_visited = visited_houses.values().count();

    println!(
        "Thanks to Santa and Robo-Santa, {houses_visited} houses received at least one present."
    );
}

fn main() -> anyhow::Result<()> {
    let directions = lib::input_to_string("input/day3.txt")?
        .chars()
        .map(|c| Direction::try_from(c))
        .collect::<LibResult<Vec<Direction>>>()?;

    part1(&directions);
    part2(&directions);

    Ok(())
}
