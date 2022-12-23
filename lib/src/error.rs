//! Error module. Every errors coming out of the library must be `LibError`.

use thiserror::Error;

use crate::{array, direction, grid, nposition, position};

/// Global Error type for the library.
#[derive(Error, Debug)]
pub enum LibError {
    #[error(transparent)]
    Array(array::Error),
    #[error(transparent)]
    Direction(direction::Error),
    #[error(transparent)]
    Position(position::Error),
    #[error(transparent)]
    Grid(grid::Error),
    /// Error coming from the `position` module.
    #[error(transparent)]
    NPosition(nposition::Error),
}

/// Reexport of the `Result` type, locking the error type.
pub type LibResult<T> = Result<T, LibError>;
