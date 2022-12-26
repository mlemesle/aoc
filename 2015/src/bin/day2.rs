fn do_split(s: &str) -> [usize; 3] {
    let t = s.split('x').map(|n| n.parse().unwrap()).collect::<Vec<_>>();

    [t[0], t[1], t[2]]
}

fn part1(presents: &[[usize; 3]]) {
    let paper_needed: usize = presents
        .iter()
        .map(|present| {
            let side1 = present[0] * present[1];
            let side2 = present[1] * present[2];
            let side3 = present[0] * present[2];
            2 * side1 + 2 * side2 + 2 * side3 + [side1, side2, side3].iter().min().unwrap()
        })
        .sum();

    println!("They should order {paper_needed} square feet of paper.");
}

fn part2(presents: &mut [[usize; 3]]) {
    let ribbon_length: usize = presents
        .iter_mut()
        .map(|present| -> usize {
            present.sort();
            let sides: usize = present.iter().take(2).map(|n| *n * 2).sum();

            sides + present.iter().product::<usize>()
        })
        .sum();

    println!("They should order {ribbon_length} feet of ribbon.");
}

fn main() -> anyhow::Result<()> {
    let mut presents: Vec<_> = lib::input_lines("input/day2.txt")
        .map(|line| do_split(&line))
        .collect::<Vec<_>>();

    part1(&presents);
    part2(&mut presents);

    Ok(())
}
