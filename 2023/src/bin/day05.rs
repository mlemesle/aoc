use std::ops::Range;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<MapRange>>,
}

#[derive(Debug)]
struct MapRange {
    source_range: Range<usize>,
    destination_range_start: usize,
}

fn parse_almanac(input: String) -> anyhow::Result<Almanac> {
    let mut parts = input.split("\n\n");

    let seeds = parts
        .next()
        .and_then(|seed_line| seed_line.strip_prefix("seeds: "))
        .map(|seed_numbers| {
            seed_numbers
                .split_whitespace()
                .map(|n| n.parse().map_err(anyhow::Error::from))
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .ok_or_else(|| anyhow::anyhow!("Can't parse seeds numbers"))??;

    let maps = parts
        .map(|map_str| {
            map_str
                .lines()
                .skip(1)
                .map(|line| {
                    let mut range_parts = line.split_whitespace();
                    let destination_range_start = range_parts.next().unwrap().parse().unwrap();
                    let source_range_start: usize = range_parts.next().unwrap().parse().unwrap();
                    let range_size: usize = range_parts.next().unwrap().parse().unwrap();

                    MapRange {
                        source_range: source_range_start..source_range_start + range_size,
                        destination_range_start,
                    }
                })
                .collect()
        })
        .collect();

    Ok(Almanac { seeds, maps })
}

fn part1(almanac: &Almanac) {
    let min_distance: usize = almanac
        .seeds
        .iter()
        .copied()
        .map(|seed| {
            almanac.maps.iter().fold(seed, |acc, map| {
                match map
                    .iter()
                    .find(|map_range| map_range.source_range.contains(&acc))
                {
                    Some(map_range) => {
                        map_range.destination_range_start + (acc - map_range.source_range.start)
                    }
                    None => acc,
                }
            })
        })
        .min()
        .unwrap();

    println!("The lowest location number that corresponds to any of the initial seed numbers is: {min_distance}.");
}

fn part2(almanac: Almanac) {
    let min_distance: usize = almanac
        .seeds
        .chunks_exact(2)
        .flat_map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .map(|seed| {
            almanac.maps.iter().fold(seed, |acc, map| {
                match map
                    .iter()
                    .find(|map_range| map_range.source_range.contains(&acc))
                {
                    Some(map_range) => {
                        map_range.destination_range_start + (acc - map_range.source_range.start)
                    }
                    None => acc,
                }
            })
        })
        .min()
        .unwrap();

    println!("The lowest location number that corresponds to any of the initial seed numbers is: {min_distance}.");
}

fn main() -> anyhow::Result<()> {
    let input = lib::input_to_string("./input/day05.txt")?;

    let almanac = parse_almanac(input)?;

    part1(&almanac);
    part2(almanac);

    Ok(())
}
