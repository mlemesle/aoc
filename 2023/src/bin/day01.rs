fn solve<'a>(
    part: usize,
    instructions: impl Iterator<Item = &'a String>,
    str_fn: fn(&str) -> String,
) -> anyhow::Result<()> {
    let calibration_values = instructions
        .enumerate()
        .map(|(index, ins)| -> anyhow::Result<u32> {
            let ins = str_fn(ins);
            let mut digits = ins.chars().filter(char::is_ascii_digit);
            let first_digit = digits
                .next()
                .map(|c| c.to_digit(10).unwrap())
                .ok_or_else(|| anyhow::anyhow!("Line {index} - {ins} has no digit"))?;
            let second_digit = digits
                .last()
                .map(|c| c.to_digit(10).unwrap())
                .unwrap_or(first_digit);

            Ok(first_digit * 10 + second_digit)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let total: u32 = calibration_values.iter().sum();

    println!("Total for part {part} is {total}");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let lines = lib::input_lines("input/day01.txt").collect::<Vec<_>>();

    solve(1, lines.iter(), str::to_string)?;
    solve(2, lines.iter(), |s| {
        s.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
    })?;

    Ok(())
}
