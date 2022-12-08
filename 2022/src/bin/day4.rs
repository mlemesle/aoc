use std::{ops::RangeInclusive, str::FromStr};

struct SectionAssignments {
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

impl FromStr for SectionAssignments {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = {
            let (assign1, assign2) = s.split_once(',').expect("can't split line on ','");
            let (first1, first2) = assign1.split_once('-').unwrap();
            let first = first1.parse()?..=first2.parse()?;

            let (second1, second2) = assign2.split_once('-').unwrap();
            let second = second1.parse()?..=second2.parse()?;

            (first, second)
        };

        Ok(SectionAssignments { first, second })
    }
}

fn iter_filter_count<'a>(
    assignments: impl Iterator<Item = &'a SectionAssignments>,
    func: impl FnMut(&&'a SectionAssignments) -> bool,
) -> usize {
    assignments.filter(func).count()
}

fn part1<'a>(assignments: impl Iterator<Item = &'a SectionAssignments>) {
    let func = |sections_assignments: &&SectionAssignments| {
        let (f_start, f_end) = (
            sections_assignments.first.start(),
            sections_assignments.first.end(),
        );
        let (s_start, s_end) = (
            sections_assignments.second.start(),
            sections_assignments.second.end(),
        );

        f_start <= s_start && f_end >= s_end || s_start <= f_start && s_end >= f_end
    };

    let res = iter_filter_count(assignments, func);

    println!(
        "There is {} assignment pairs where a range fully contain the other",
        res
    );
}

fn part2<'a>(assignments: impl Iterator<Item = &'a SectionAssignments>) {
    let func = |sections_assignments: &&SectionAssignments| {
        let (f_start, f_end) = (
            sections_assignments.first.start(),
            sections_assignments.first.end(),
        );
        let (s_start, s_end) = (
            sections_assignments.second.start(),
            sections_assignments.second.end(),
        );

        f_start <= s_end && f_end >= s_start
    };

    let res = iter_filter_count(assignments, func);

    println!("There is {} assignment pairs that overlaps", res);
}

fn main() -> Result<(), anyhow::Error> {
    let assignments = lib::input::<SectionAssignments>("input/day4.txt")?;
    part1(assignments.iter());
    part2(assignments.iter());

    Ok(())
}
