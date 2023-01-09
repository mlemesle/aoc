use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use lib::permutation::Permutations;

#[derive(Debug)]
struct CityToCityDistance(String, String, usize);

impl FromStr for CityToCityDistance {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cities, distance) = s.split_once(" = ").unwrap();
        let (from, to) = cities.split_once(" to ").unwrap();

        Ok(CityToCityDistance(
            from.into(),
            to.into(),
            distance.parse()?,
        ))
    }
}

fn all_travel_distances(c_to_c_dist: Vec<CityToCityDistance>) -> Vec<usize> {
    let (distance_map, cities) = c_to_c_dist.iter().fold(
        (HashMap::new(), HashSet::new()),
        |(mut map, mut set), elem| {
            map.insert(format!("{}-{}", elem.0, elem.1), elem.2);
            map.insert(format!("{}-{}", elem.1, elem.0), elem.2);
            set.insert(elem.0.clone());
            set.insert(elem.1.clone());

            (map, set)
        },
    );

    let cities = cities.into_iter().collect::<Vec<_>>();

    let cities_perm = Permutations::<String>::from(cities);

    cities_perm
        .iter()
        .map(|perm| {
            perm.windows(2)
                .map(|w| distance_map.get(&format!("{}-{}", w[0], w[1])).unwrap())
                .sum::<usize>()
        })
        .collect()
}

fn part1(travel_distances: &[usize]) {
    let min_distance = travel_distances.iter().min();

    println!("The distance of the shortest route is {min_distance:?}");
}

fn part2(travel_distances: &[usize]) {
    let max_distance = travel_distances.iter().max();

    println!("The distance of the longest route is {max_distance:?}");
}

fn main() -> anyhow::Result<()> {
    let c_to_c_dist = lib::input::<CityToCityDistance>("input/day9.txt")?;
    let travel_distances = all_travel_distances(c_to_c_dist);

    part1(&travel_distances);
    part2(&travel_distances);

    Ok(())
}
