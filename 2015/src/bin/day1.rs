fn part1(instructions: &str) {
    let floor = instructions
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum::<isize>();

    println!("The instructions will take Santa to {floor} floor.");
}

fn part2(instructions: &str) {
    let position = instructions
        .chars()
        .scan(0isize, |state, elem| {
            *state += match elem {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            };

            Some(*state)
        })
        .position(|elem| elem == -1)
        .unwrap_or_default()
        + 1;

    println!("The position of the character that causes Santa to first enter the basement is {position}.");
}

fn main() -> anyhow::Result<()> {
    let instructions = lib::input_to_string("input/day1.txt")?;

    part1(&instructions);
    part2(&instructions);

    Ok(())
}
