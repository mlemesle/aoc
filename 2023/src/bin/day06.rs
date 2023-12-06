use core::time;

fn part1() -> anyhow::Result<()> {
    let mut lines = lib::input_lines("./input/day06.txt");
    let time_and_distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().map_err(anyhow::Error::from))
        .zip(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|n| n.parse().map_err(anyhow::Error::from)),
        )
        .map(|(time, dist)| match (time, dist) {
            (Ok(t), Ok(d)) => Ok((t, d)),
            (Ok(t), Err(e)) => Err(anyhow::anyhow!("Invalid distance for time {t}: {e}")),
            (Err(e), Ok(d)) => Err(anyhow::anyhow!("Invalid time for distance {d}: {e}")),
            (Err(et), Err(ed)) => Err(anyhow::anyhow!("Invalid time and distance {et}\n{ed}")),
        })
        .collect::<anyhow::Result<Vec<(usize, usize)>>>()?;

    let computed_margin: usize = time_and_distances
        .iter()
        .map(|(time, distance)| {
            (0..*time)
                .filter(|time_holding| time_holding * (time - time_holding) > *distance)
                .count()
        })
        .product();

    println!("The computed margin is : {computed_margin}.");

    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let mut lines = lib::input_lines("./input/day06.txt");
    let time: usize = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse()?;
    let distance: usize = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse()?;

    let mut nb_ways_to_win = 0;
    for time_holding in 0..time {
        if time_holding * (time - time_holding) > distance {
            nb_ways_to_win += 1;
        }
    }

    println!("There are {nb_ways_to_win} ways to win.");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    part1()?;
    part2()?;

    Ok(())
}
