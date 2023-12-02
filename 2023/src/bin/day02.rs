use std::str::FromStr;

#[derive(Debug, Default)]
struct Draw {
    green: usize,
    red: usize,
    blue: usize,
}

impl FromStr for Draw {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(", ")
            .try_fold(Draw::default(), |mut draw_acc, draw_part| {
                match draw_part.split_once(' ') {
                    Some((number, color)) => match color {
                        "red" => draw_acc.red = number.parse()?,
                        "blue" => draw_acc.blue = number.parse()?,
                        "green" => draw_acc.green = number.parse()?,
                        unknown => anyhow::bail!("Unknown color - {unknown}"),
                    },
                    None => anyhow::bail!("No space in {s:?}"),
                }
                Ok(draw_acc)
            })
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("Game ")
            .ok_or_else(|| anyhow::anyhow!("Not starting with 'Game ' - {s}"))?;
        let (id_str, draws_str) = s
            .split_once(": ")
            .ok_or_else(|| anyhow::anyhow!("No ': ' - {s}"))?;

        let id = id_str.parse()?;

        let draws = draws_str
            .split("; ")
            .map(str::parse)
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Game { id, draws })
    }
}

fn part1(games: &[Game]) {
    let game_sum: usize = games
        .iter()
        .filter_map(|game| {
            game.draws
                .iter()
                .all(|Draw { green, red, blue }| *green <= 13 && *red <= 12 && *blue <= 14)
                .then_some(game.id)
        })
        .sum();

    println!("The sum of the IDs of those games is {game_sum}.");
}

fn part2(games: Vec<Game>) {
    let power_sum: usize = games
        .into_iter()
        .map(|game| {
            let set = game
                .draws
                .into_iter()
                .reduce(|mut acc, Draw { green, red, blue }| {
                    if green > acc.green {
                        acc.green = green;
                    }
                    if red > acc.red {
                        acc.red = red;
                    }
                    if blue > acc.blue {
                        acc.blue = blue;
                    }
                    acc
                })
                .unwrap();

            set.green * set.red * set.blue
        })
        .sum();

    println!("The sum of the power of these sets is {power_sum}.");
}

fn main() -> anyhow::Result<()> {
    let games: Vec<Game> = lib::input_lines("./input/day02.txt")
        .map(|s| s.parse())
        .collect::<anyhow::Result<Vec<_>>>()?;

    part1(&games);
    part2(games);

    Ok(())
}
