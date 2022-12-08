use std::{ops::Range, str::FromStr};

#[derive(Debug)]
struct Forest {
    tree_lines: Vec<TreeLine>,
    max_row_index: usize,
    max_col_index: usize,
}

impl Forest {
    fn get_max_in_row_range(&self, row: usize, range: Range<usize>) -> Option<u8> {
        self.tree_lines[row].0[range].iter().max().copied()
    }

    fn get_max_in_col_range(&self, col: usize, range: Range<usize>) -> Option<u8> {
        self.tree_lines
            .iter()
            .skip(range.start)
            .take(range.end - range.start)
            .map(|row| row.0[col])
            .max()
    }

    fn is_tree_visible(&self, i: usize, j: usize, tree: u8) -> bool {
        let tree = Some(tree);

        i == 0
            || i == self.max_row_index
            || j == 0
            || j == self.max_col_index
            || self.get_max_in_row_range(i, 0..j) < tree
            || self.get_max_in_row_range(i, j + 1..self.max_row_index + 1) < tree
            || self.get_max_in_col_range(j, 0..i) < tree
            || self.get_max_in_col_range(j, i + 1..self.max_col_index + 1) < tree
    }

    fn nb_trees_visible_left(&self, tree_size: u8, row: usize, range: Range<usize>) -> usize {
        let range_len = range.len();
        self.tree_lines[row].0[range]
            .iter()
            .rev()
            .enumerate()
            .find(|(_, &tree)| tree_size <= tree)
            .map(|(nb, _)| nb + 1)
            .unwrap_or(range_len)
    }

    fn nb_trees_visible_right(&self, tree_size: u8, row: usize, range: Range<usize>) -> usize {
        let range_len = range.len();
        self.tree_lines[row].0[range]
            .iter()
            .enumerate()
            .find(|(_, &tree)| tree_size <= tree)
            .map(|(nb, _)| nb + 1)
            .unwrap_or(range_len)
    }

    fn nb_trees_visible_up(&self, tree_size: u8, col: usize, range: Range<usize>) -> usize {
        let range_len = range.len();
        self.tree_lines
            .iter()
            .skip(range.start)
            .take(range.end - range.start)
            .map(|row| row.0[col])
            .rev()
            .enumerate()
            .find(|(_, tree)| tree_size <= *tree)
            .map(|(nb, _)| nb + 1)
            .unwrap_or(range_len)
    }

    fn nb_trees_visible_down(&self, tree_size: u8, col: usize, range: Range<usize>) -> usize {
        let range_len = range.len();
        self.tree_lines
            .iter()
            .skip(range.start)
            .take(range.end - range.start)
            .map(|row| row.0[col])
            .enumerate()
            .find(|(_, tree)| tree_size <= *tree)
            .map(|(nb, _)| nb + 1)
            .unwrap_or(range_len)
    }

    fn get_scenic_score(&self, i: usize, j: usize, tree: u8) -> usize {
        let left = self.nb_trees_visible_left(tree, i, 0..j);
        let right = self.nb_trees_visible_right(tree, i, j + 1..self.max_row_index + 1);
        let up = self.nb_trees_visible_up(tree, j, 0..i);
        let down = self.nb_trees_visible_down(tree, j, i + 1..self.max_col_index + 1);

        up * down * left * right
    }
}

#[derive(Debug)]
struct TreeLine(Vec<u8>);

impl FromStr for TreeLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s.chars().map(|c| c as u8 - 48).collect();

        Ok(Self(row))
    }
}

fn part1(forest: &Forest) {
    let visible_trees: usize = forest
        .tree_lines
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.0
                .iter()
                .enumerate()
                .map(|(j, &tree)| usize::from(forest.is_tree_visible(i, j, tree)))
                .sum::<usize>()
        })
        .sum();

    println!("{visible_trees} trees are visible from outside the grid");
}

fn part2(forest: &Forest) {
    let highest_scenic_score: usize = forest
        .tree_lines
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.0
                .iter()
                .enumerate()
                .map(|(j, &tree)| forest.get_scenic_score(i, j, tree))
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap_or_default();

    println!("The highest scenic score possible for any tree is {highest_scenic_score}");
}

fn main() -> Result<(), anyhow::Error> {
    let rows = lib::input::<TreeLine>("input/day8.txt")?;
    let max_row_index = rows[0].0.len() - 1;
    let max_col_index = rows.len() - 1;
    let forest = Forest {
        tree_lines: rows,
        max_row_index,
        max_col_index,
    };
    part1(&forest);
    part2(&forest);

    Ok(())
}
