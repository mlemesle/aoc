use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

struct Card {
    id: usize,
    winnings: HashSet<usize>,
    havings: HashSet<usize>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("Card")
            .map(str::trim_start)
            .ok_or_else(|| anyhow::anyhow!("No prefix 'Card...'"))?;

        let (id_str, numbers_str) = s
            .split_once(": ")
            .ok_or_else(|| anyhow::anyhow!("No ': '"))?;

        let id = id_str.parse()?;

        let (winnings_str, havings_str) = numbers_str
            .split_once(" | ")
            .ok_or_else(|| anyhow::anyhow!("No ' | '"))?;

        let winnings = winnings_str
            .split_whitespace()
            .map(|w| w.parse().map_err(anyhow::Error::from))
            .collect::<anyhow::Result<HashSet<_>>>()?;
        let havings = havings_str
            .split_whitespace()
            .map(|w| w.parse().map_err(anyhow::Error::from))
            .collect::<anyhow::Result<HashSet<_>>>()?;

        Ok(Card {
            id,
            winnings,
            havings,
        })
    }
}

fn part1<'a>(cards: impl Iterator<Item = &'a Card>) {
    let sum = cards
        .map(|card| card.winnings.intersection(&card.havings).count())
        .map(|nb_common| {
            if nb_common != 0 {
                2usize.pow(nb_common as u32 - 1)
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("Total points worth : {sum}.");
}

fn part2(cards: Vec<Card>) {
    let sum = cards
        .into_iter()
        .map(|card| (card.id, card.winnings.intersection(&card.havings).count()))
        .fold(
            HashMap::<usize, usize>::new(),
            |mut map, (card_id, nb_matches)| {
                let n = *map.entry(card_id).or_insert(1);
                (card_id + 1..card_id + 1 + nb_matches)
                    .for_each(|cid| *map.entry(cid).or_insert(1) += n);
                map
            },
        )
        .into_values()
        .sum::<usize>();

    println!("Total scratchcards is : {sum:#?}.");
}

fn main() -> anyhow::Result<()> {
    let cards = lib::input("./input/day04.txt")?;

    part1(cards.iter());
    part2(cards);

    Ok(())
}
