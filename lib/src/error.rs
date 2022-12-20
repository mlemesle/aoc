//! Error module. Every errors coming out of the library must be `LibError`.

use thiserror::Error;

use crate::{array, direction, position};

/// Global Error type for the library.
#[derive(Error, Debug)]
pub enum LibError {
    #[error(transparent)]
    Array(array::Error),
    #[error(transparent)]
    Direction(direction::Error),
    /// Error coming from the `position` module.
    #[error(transparent)]
    Position(position::Error),
}

/// Reexport of the `Result` type, locking the error type.
pub type LibResult<T> = Result<T, LibError>;
