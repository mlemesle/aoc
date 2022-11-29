pub mod slutmap;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Context;

pub fn input<T>(path: &str) -> impl Iterator<Item = T>
where
    T: FromStr<Err = anyhow::Error>,
{
    let file = File::open(path).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().enumerate().map(|(index, l)| {
        l.with_context(|| format!("Error while reading line {}", index + 1))
            .expect("Error reading line")
            .parse::<T>()
            .map_err(|_| anyhow::anyhow!("Error while parsing line {}", index + 1))
            .unwrap()
    })
}
