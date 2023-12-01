#![deny(missing_docs)]
//! Library created to abtract types and their principal uses for [https://adventofcode.com](AoC).

use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub mod btree;
pub mod direction;
pub mod error;
pub mod grid;
pub mod nposition;
pub mod permutation;
pub mod position;

use anyhow::Context;

/// Retrives a BufReader from a Path.
/// ie. Opens the file and create a new Reader from it.
fn get_buf(path: impl AsRef<Path>) -> BufReader<File> {
    let file = File::open(path).expect("File not found");
    BufReader::new(file)
}

/// Open the file located at `path` and parse every line to `T`.
/// `T` must be FromStr and raise `anyhow::Error` error type.
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

/// Open the file located at `path` and yield an iterator on every line, as String.
pub fn input_lines(path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    get_buf(path).lines().map_while(Result::ok)
}

/// Open the file located at `path` and read everything at once, storing the whole file in a String.
pub fn input_to_string(path: impl AsRef<Path>) -> Result<String, anyhow::Error> {
    Ok(fs::read_to_string(path)?)
}
