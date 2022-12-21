fn part1(line: &str) -> bool {
    line.chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
        .count()
        >= 3
        && line
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|window| window[0] == window[1])
        && ["ab", "cd", "pq", "xy"]
            .iter()
            .all(|&duo| !line.contains(duo))
}

fn part2(line: &str) -> bool {
    let line = line.chars().collect::<Vec<_>>();

    line.windows(2)
        .enumerate()
        .any(|(i, window)| line[(i + 2)..].windows(2).any(|win| window == win))
        && line.windows(3).any(|window| window[0] == window[2])
}

fn solve(lines: &[String], filter: fn(&str) -> bool) {
    let good_lines = lines.iter().filter(|line| filter(line)).count();

    println!("{good_lines} strings are nice.");
}

fn main() -> anyhow::Result<()> {
    let lines = lib::input_lines("input/day5.txt").collect::<Vec<_>>();

    solve(&lines, part1);
    solve(&lines, part2);

    Ok(())
}
