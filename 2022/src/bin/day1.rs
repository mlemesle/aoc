fn part1(inventory_lines: &[u32]) {
    let max = inventory_lines.iter().max();
    println!("Total calories of all elves is {:?}", max);
}

fn part2(mut inventory_lines: Vec<u32>) {
    inventory_lines.sort();
    let result: u32 = inventory_lines.iter().rev().take(3).sum();
    println!("Sum calories for the top 3 elves is {:?}", result);
}

fn main() -> anyhow::Result<()> {
    let inventory_lines: Vec<_> = lib::input_lines("input/day1.txt")
        .collect::<Vec<_>>()
        .split(|inventory_line| inventory_line.is_empty())
        .map(|elf_inventory_lines| {
            elf_inventory_lines
                .iter()
                .map(|elf_inventory_line| elf_inventory_line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    part1(&inventory_lines[..]);
    part2(inventory_lines);
    Ok(())
}
