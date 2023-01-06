fn memory_size_1(string: &str) -> (usize, usize) {
    let mut chars = string[1..string.len() - 1].chars();
    let (mut nb_chars, mut mem_size) = (2, 0);

    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next().unwrap() {
                '"' | '\\' => {
                    nb_chars += 2;
                    mem_size += 1;
                }
                'x' => {
                    // No advance_by yet
                    chars.next();
                    chars.next();
                    nb_chars += 4;
                    mem_size += 1;
                }
                _ => unreachable!(),
            },
            _ => {
                nb_chars += 1;
                mem_size += 1;
            }
        }
    }

    (nb_chars, mem_size)
}

fn memory_size_2(string: &str) -> (usize, usize) {
    let mut chars = string[1..string.len() - 1].chars();
    let (mut nb_chars, mut mem_size) = (6, 2);

    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next().unwrap() {
                '"' | '\\' => {
                    nb_chars += 4;
                    mem_size += 2;
                }
                'x' => {
                    // No advance_by yet
                    chars.next();
                    chars.next();
                    nb_chars += 5;
                    mem_size += 4;
                }
                _ => unreachable!(),
            },
            _ => {
                nb_chars += 1;
                mem_size += 1;
            }
        }
    }

    (nb_chars, mem_size)
}

fn solve(lines: &[String], f: fn(&str) -> (usize, usize)) {
    let total: usize = lines.iter().map(|line| f(line)).map(|t| t.0 - t.1).sum();

    println!("The number of characters of code for string literals minus the number of characters in memory for the values of the strings in total for the entire file is {total}");
}

fn main() -> anyhow::Result<()> {
    let lines = lib::input_lines("input/day8.txt").collect::<Vec<_>>();

    solve(&lines, memory_size_1);
    solve(&lines, memory_size_2);

    Ok(())
}
