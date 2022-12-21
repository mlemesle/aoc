use lib::array::Array;

fn do_split(s: &str) -> Vec<&str> {
    s.split('x').collect()
}

fn part1(presents: &[Array<3, usize>]) {
    let paper_needed: usize = presents
        .iter()
        .map(|present| {
            let side1 = present.get(0) * present.get(1);
            let side2 = present.get(1) * present.get(2);
            let side3 = present.get(0) * present.get(2);
            2 * side1 + 2 * side2 + 2 * side3 + [side1, side2, side3].iter().min().unwrap()
        })
        .sum();

    println!("They should order {paper_needed} square feet of paper.");
}

fn part2(presents: &[Array<3, usize>]) {
    let ribbon_length: usize = presents
        .iter()
        .map(|present| -> usize {
            let mut sides = [present.get(0), present.get(1), present.get(2)];
            sides.sort();
            let sides: usize = sides.iter().take(2).map(|n| *n * 2).sum();

            sides
                + [*present.get(0), *present.get(1), *present.get(2)]
                    .iter()
                    .product::<usize>()
        })
        .sum();

    println!("They should order {ribbon_length} feet of ribbon.");
}

fn main() -> anyhow::Result<()> {
    let presents: Vec<_> = lib::input_lines("input/day2.txt")
        .map(|line| Array::<3, usize>::new(&line, do_split))
        .collect::<Result<Vec<_>, _>>()?;

    part1(&presents);
    part2(&presents);

    Ok(())
}
