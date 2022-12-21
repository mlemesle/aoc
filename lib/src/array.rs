//! Array module. The goal is to ease the parsing of lines
//! that can be parsed and collected into an array.

use std::str::FromStr;

use thiserror::Error;

use crate::error::{LibError, LibResult};

#[derive(Error, Debug)]
pub enum Error {
    #[error("can't create Array, wanted size is `{0}`, got `{1}`")]
    BadLength(usize, usize),
    #[error("can't create Array")]
    New,
}

impl From<Error> for LibError {
    fn from(value: Error) -> Self {
        LibError::Array(value)
    }
}

#[derive(Debug)]
pub struct Array<const SIZE: usize, T: FromStr + Default + Clone + Copy>([T; SIZE]);

impl<const SIZE: usize, T> Array<SIZE, T>
where
    T: FromStr + Default + Clone + Copy,
{
    pub fn new(s: &str, f: fn(&str) -> Vec<&str>) -> LibResult<Self> {
        let res = f(s);

        if res.len() != SIZE {
            return Err(LibError::from(Error::BadLength(SIZE, res.len())));
        }

        let mut iter = res
            .iter()
            .map(|elem| elem.parse().map_err(|_| LibError::from(Error::New)));

        let mut arr = [T::default(); SIZE];
        for item in &mut arr {
            *item = match iter.next() {
                Some(elem) => elem?,
                None => return Err(LibError::from(Error::New)),
            }
        }

        Ok(Self(arr))
    }

    pub fn get(&self, i: usize) -> &T {
        &self.0[i]
    }
}
