use std::collections::HashSet;

fn part1(grid: &[Vec<char>]) {
    let mut set = HashSet::new();
    // Iterate over each char.
    for row in 0..grid.len() {
        for (col, c) in grid[row].iter().enumerate() {
            // Dodge every char that isn't a symbol.
            if c.is_ascii_digit() || *c == '.' {
                continue;
            }

            // Cursed code to iterate over cells around the symbol.
            let (coli, rowi) = (col as isize, row as isize);
            for cand_row in [rowi - 1, rowi, rowi + 1] {
                for cand_col in [coli - 1, coli, coli + 1] {
                    if cand_row < 0
                        || cand_row >= (grid.len() as isize)
                        || cand_col < 0
                        || cand_col >= (grid[row].len() as isize)
                        || !grid[cand_row as usize][cand_col as usize].is_ascii_digit()
                    {
                        continue;
                    }

                    // Get the position of the first digit of the number.
                    let mut colii = cand_col;
                    while colii > 0
                        && grid[cand_row as usize][(colii - 1) as usize].is_ascii_digit()
                    {
                        colii -= 1;
                    }
                    set.insert((cand_row as usize, colii as usize));
                }
            }
        }
    }

    // Sum all the numbers.
    let sum = set
        .iter()
        .map(|(row, mut col)| {
            let mut acc = 0;
            while col < grid[*row].len() && grid[*row][col].is_ascii_digit() {
                acc = acc * 10 + (grid[*row][col].to_digit(10).unwrap() as usize);
                col += 1;
            }
            acc
        })
        .sum::<usize>();

    println!("The sum of all of the part numbers in the engine schematic is {sum}");
}

fn part2(grid: &[Vec<char>]) {
    let mut sum = 0;
    for row in 0..grid.len() {
        for (col, c) in grid[row].iter().enumerate() {
            // Only interested in '*'
            if *c != '*' {
                continue;
            }

            let mut set = HashSet::new();

            // Iterate over all possible cells around '*'.
            let (coli, rowi) = (col as isize, row as isize);
            for cand_row in [rowi - 1, rowi, rowi + 1] {
                for cand_col in [coli - 1, coli, coli + 1] {
                    if cand_row < 0
                        || cand_row >= (grid.len() as isize)
                        || cand_col < 0
                        || cand_col >= (grid[row].len() as isize)
                        || !grid[cand_row as usize][cand_col as usize].is_ascii_digit()
                    {
                        continue;
                    }

                    // Register the positions of the first digit of every numbers.
                    let mut colii = cand_col;
                    while colii > 0
                        && grid[cand_row as usize][(colii - 1) as usize].is_ascii_digit()
                    {
                        colii -= 1;
                    }
                    set.insert((cand_row as usize, colii as usize));
                }
            }

            // If there isn't exactly 2 numbers, we skip.
            if set.len() != 2 {
                continue;
            }

            // Add the product of those numbers to the total.
            sum += set
                .iter()
                .map(|(row, mut col)| {
                    let mut acc = 0;
                    while col < grid[*row].len() && grid[*row][col].is_ascii_digit() {
                        acc = acc * 10 + (grid[*row][col].to_digit(10).unwrap() as usize);
                        col += 1;
                    }
                    acc
                })
                .product::<usize>();
        }
    }

    println!("The sum of all of the gear ratios in your engine schematic is {sum}");
}

fn main() {
    let grid = lib::input_lines("./input/day03.txt")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    part1(&grid);
    part2(&grid);
}
