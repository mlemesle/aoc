//! Position module. Can easily creates and interact with others types from the library.

use thiserror::Error;

use crate::{
    direction::Direction,
    error::{LibError, LibResult},
};

/// `position` module inner error. Must be wrapped in LibError before being raised.
#[derive(Error, Debug)]
pub enum Error {
    /// Raised when Position can't be created from the input.
    #[error("invalid tuple for creation")]
    Create,
    /// Raised when `Direction`can't be applied to `Position`.
    #[error("can't apply {0:?} to {1:?}")]
    Apply(Direction, Position),
}

/// Easily creates LibError with the desired variant.
impl From<Error> for LibError {
    fn from(value: Error) -> Self {
        LibError::Position(value)
    }
}

/// Position represents a position in a 2D environment.
/// * x is the horizontal coordinate,
/// * y is the vertical coordinate.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

/// Try to create a Position from a given tuple of two heterogenous elements.
/// Both elements must be `TryInto<usize>`.
impl<T, U> TryFrom<(T, U)> for Position
where
    T: TryInto<usize>,
    U: TryInto<usize>,
{
    type Error = LibError;

    fn try_from(tuple: (T, U)) -> LibResult<Self> {
        match (tuple.0.try_into(), tuple.1.try_into()) {
            (Ok(x), Ok(y)) => Ok(Self { x, y }),
            _ => Err(LibError::from(Error::Create)),
        }
    }
}

impl Position {
    /// Creates a new Position from x - y coordinates.
    /// ```rust
    ///    use lib::position::Position;
    ///
    ///    let pos = Position::new(1, 2);
    /// ```
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Retrieves the x coordinate of the position.
    /// ```rust
    ///    use lib::position::Position;
    ///
    ///    let pos = Position::new(1, 2);
    ///    assert_eq!(1, pos.x());
    /// ```
    pub fn x(&self) -> usize {
        self.x
    }

    /// Retrieves the y coordinate of the position.
    /// ```rust
    ///    use lib::position::Position;
    ///
    ///    let pos = Position::new(1, 2);
    ///    assert_eq!(2, pos.y());
    /// ```
    pub fn y(&self) -> usize {
        self.y
    }

    /// Applies a `Direction` to the `Position`, ie. moves to the said direction, if possible.
    /// ```rust
    ///    use lib::{position::Position, direction::Direction};
    ///
    ///    let mut pos = Position::default();
    ///    let res = pos.apply_direction(&Direction::Up);
    ///    assert!(res.is_ok());
    ///    assert_eq!(pos, Position::new(0, 1));
    /// ```
    pub fn apply_direction(&mut self, direction: &Direction) -> LibResult<()> {
        match direction {
            Direction::Up => {
                let res = self.y.checked_add(1);
                if let Some(new_y) = res {
                    self.y = new_y;
                    Ok(())
                } else {
                    Err(Error::Apply(Direction::Up, *self).into())
                }
            }
            Direction::Down => {
                let res = self.y.checked_sub(1);
                if let Some(new_y) = res {
                    self.y = new_y;
                    Ok(())
                } else {
                    Err(Error::Apply(Direction::Down, *self).into())
                }
            }
            Direction::Left => {
                let res = self.x.checked_sub(1);
                if let Some(new_x) = res {
                    self.x = new_x;
                    Ok(())
                } else {
                    Err(Error::Apply(Direction::Left, *self).into())
                }
            }
            Direction::Right => {
                let res = self.x.checked_add(1);
                if let Some(new_x) = res {
                    self.x = new_x;
                    Ok(())
                } else {
                    Err(Error::Apply(Direction::Right, *self).into())
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{direction::Direction, error::LibError};

    use super::{Error, Position};

    #[test]
    fn try_from() {
        let pos1 = Position::try_from((-1i8, -2i32));
        assert!(matches!(pos1, Err(LibError::Position(Error::Create))));
        let pos2 = Position::try_from((-1i8, 2u32));
        assert!(matches!(pos2, Err(LibError::Position(Error::Create))));
        let pos3 = Position::try_from((1i8, -3i8));
        assert!(matches!(pos3, Err(LibError::Position(Error::Create))));
        let pos4 = Position::try_from((1i8, 2u32));
        assert!(matches!(pos4, Ok(Position { x: 1, y: 2 })));
    }

    #[test]
    fn apply_direction() {
        // Trivial tests
        let mut pos = Position::default();
        assert!(pos.apply_direction(&Direction::Up).is_ok());
        assert_eq!((pos.x(), pos.y()), (0, 1));
        assert!(pos.apply_direction(&Direction::Right).is_ok());
        assert_eq!((pos.x(), pos.y()), (1, 1));
        assert!(pos.apply_direction(&Direction::Down).is_ok());
        assert_eq!((pos.x(), pos.y()), (1, 0));
        assert!(pos.apply_direction(&Direction::Left).is_ok());
        assert_eq!((pos.x(), pos.y()), (0, 0));

        // Limit testing
        let mut pos = Position::new(usize::MIN, 0);
        assert!(matches!(
            pos.apply_direction(&Direction::Left),
            Err(LibError::Position(Error::Apply(
                Direction::Left,
                Position {
                    x: usize::MIN,
                    y: 0,
                },
            )))
        ));

        let mut pos = Position::new(usize::MAX, 0);
        assert!(matches!(
            pos.apply_direction(&Direction::Right),
            Err(LibError::Position(Error::Apply(
                Direction::Right,
                Position {
                    x: usize::MAX,
                    y: 0,
                },
            )))
        ));

        let mut pos = Position::new(0, usize::MIN);
        assert!(matches!(
            pos.apply_direction(&Direction::Down),
            Err(LibError::Position(Error::Apply(
                Direction::Down,
                Position {
                    x: 0,
                    y: usize::MIN,
                },
            )))
        ));

        let mut pos = Position::new(0, usize::MAX);
        assert!(matches!(
            pos.apply_direction(&Direction::Up),
            Err(LibError::Position(Error::Apply(
                Direction::Up,
                Position {
                    x: 0,
                    y: usize::MAX,
                },
            )))
        ));
    }
}
