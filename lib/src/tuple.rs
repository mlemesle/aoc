//! Tuple module. The goal is to ease the parsing of lines
//! that can be parsed to a tuple.

pub struct Tuple<const SIZE: usize, T>([T; SIZE]);

impl Tuple {
    pub fn new() {}
}
