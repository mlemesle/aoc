//! Direction module. Used to interact with various types.

use thiserror::Error;

use crate::error::LibError;

/// Error regarding Direction manipulations.
#[derive(Error, Debug)]
pub enum Error {
    /// Raised when an error occured while parsing from char.
    #[error("invalid char `{0}`")]
    TryFromChar(char),
}

impl From<Error> for LibError {
    fn from(value: Error) -> Self {
        LibError::Direction(value)
    }
}

/// Represent the four directions in a 2D environment.
#[derive(Debug)]
pub enum Direction {
    /// Up.
    Up,
    /// Down.
    Down,
    /// Left.
    Left,
    /// Right.
    Right,
}

impl TryFrom<char> for Direction {
    type Error = LibError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(Error::TryFromChar(value).into()),
        }
    }
}
