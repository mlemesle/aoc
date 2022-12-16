use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub mod btree;

use anyhow::Context;

fn get_buf(path: impl AsRef<Path>) -> BufReader<File> {
    let file = File::open(path).expect("File not found");
    BufReader::new(file)
}

pub fn input<T>(path: impl AsRef<Path>) -> Result<Vec<T>, anyhow::Error>
where
    T: FromStr<Err = anyhow::Error>,
{
    get_buf(path)
        .lines()
        .enumerate()
        .map(|(index, l)| {
            l.with_context(|| format!("Error while reading line {}", index + 1))
                .expect("Error reading line")
                .parse::<T>()
        })
        .collect()
}

pub fn input_lines(path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    get_buf(path).lines().filter_map(|line| line.ok())
}

pub fn input_to_string(path: impl AsRef<Path>) -> Result<String, anyhow::Error> {
    Ok(fs::read_to_string(path)?)
}
