use std::{collections::HashMap, path::PathBuf, str::FromStr};

use anyhow::anyhow;

#[derive(Debug)]
enum Command {
    CdRoot,
    CdBack,
    CdTo(String),
    File(String, usize),
    Noop,
}

const DOLLAR: Option<&'static str> = Some("$");
const CD: Option<&'static str> = Some("cd");
const BACK: Option<&'static str> = Some("..");
const ROOT: Option<&'static str> = Some("/");
const DIR: Option<&'static str> = Some("dir");

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut command_parts = s.split_whitespace();
        let command_parts = (
            command_parts.next(),
            command_parts.next(),
            command_parts.next(),
        );
        match command_parts {
            (DOLLAR, CD, BACK) => Ok(Command::CdBack),
            (DOLLAR, CD, ROOT) => Ok(Command::CdRoot),
            (DOLLAR, CD, Some(path)) => Ok(Command::CdTo(path.to_string())),
            (DOLLAR, Some(_), _) => Ok(Command::Noop),
            (DIR, _, _) => Ok(Command::Noop),
            (Some(size_str), Some(filename), None) => {
                Ok(Command::File(filename.to_string(), size_str.parse()?))
            }
            (_, _, _) => Err(anyhow!("weird parts {:?}", command_parts)),
        }
    }
}

#[derive(Debug)]
struct File {
    path: PathBuf,
    name: String,
    size: usize,
}

impl File {
    fn new(path: PathBuf, name: String, size: usize) -> Self {
        Self { path, name, size }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let (_, files) = lib::input::<Command>("input/day7.txt")?.iter().fold(
        (PathBuf::from("/"), Vec::new()),
        |(mut path_buf, mut files), command| {
            match command {
                Command::CdRoot => path_buf = PathBuf::new(),
                Command::CdBack => {
                    path_buf.pop();
                }
                Command::CdTo(to) => path_buf.push(to),
                Command::File(name, size) => {
                    files.push(File::new(path_buf.clone(), name.clone(), *size))
                }
                Command::Noop => (),
            };
            (path_buf, files)
        },
    );

    println!("{:?}", files);

    Ok(())
}
