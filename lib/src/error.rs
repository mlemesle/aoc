//! Error module. Every errors coming out of the library must be `LibError`.

use thiserror::Error;

use crate::{direction, grid, nposition, position};

/// Global Error type for the library.
#[derive(Error, Debug)]
pub enum LibError {
    /// Wrapping Direction's error.
    #[error(transparent)]
    Direction(direction::Error),
    /// Wrapping Grid's error.
    #[error(transparent)]
    Grid(grid::Error),
    /// Wrapping NPosition's error.
    #[error(transparent)]
    NPosition(nposition::Error),
    /// Wrapping Position's error.
    #[error(transparent)]
    Position(position::Error),
}

/// Reexport of the `Result` type, locking the error type.
pub type LibResult<T> = Result<T, LibError>;
