//! NPosition module. Can easily creates and interact with others types from the library.

use thiserror::Error;

use crate::{
    direction::Direction,
    error::{LibError, LibResult},
};

/// `nposition` module inner error. Must be wrapped in LibError before being raised.
#[derive(Error, Debug)]
pub enum Error {
    /// Raised when NPosition can't be created from the input.
    #[error("invalid tuple for creation")]
    Create,
    /// Raised when `Direction` can't be applied to `NPosition`.
    #[error("can't apply {0:?} to {1:?}")]
    Apply(Direction, NPosition),
}

/// Easily creates LibError with the desired variant.
impl From<Error> for LibError {
    fn from(value: Error) -> Self {
        LibError::NPosition(value)
    }
}

/// NPosition represents a position in a 2D environment.
/// The coordinates are relative and can be negative.
/// * x is the horizontal coordinate,
/// * y is the vertical coordinate.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct NPosition {
    x: isize,
    y: isize,
}

/// Try to create a NPosition from a given tuple of two heterogenous elements.
/// Both elements must be `TryInto<isize>`.
impl<T, U> TryFrom<(T, U)> for NPosition
where
    T: TryInto<isize>,
    U: TryInto<isize>,
{
    type Error = LibError;

    fn try_from(tuple: (T, U)) -> LibResult<Self> {
        match (tuple.0.try_into(), tuple.1.try_into()) {
            (Ok(x), Ok(y)) => Ok(Self { x, y }),
            _ => Err(LibError::from(Error::Create)),
        }
    }
}

impl NPosition {
    /// Creates a new NPosition from x - y coordinates.
    /// ```rust
    ///    use lib::nposition::NPosition;
    ///
    ///    let pos = NPosition::new(-1, 2);
    /// ```
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Retrieves the x coordinate of the NPosition.
    /// ```rust
    ///    use lib::nposition::NPosition;
    ///
    ///    let pos = NPosition::new(-1, 2);
    ///    assert_eq!(-1, pos.x());
    /// ```
    pub fn x(&self) -> isize {
        self.x
    }

    /// Retrieves the y coordinate of the NPosition.
    /// ```rust
    ///    use lib::nposition::NPosition;
    ///
    ///    let pos = NPosition::new(-1, 2);
    ///    assert_eq!(2, pos.y());
    /// ```
    pub fn y(&self) -> isize {
        self.y
    }

    /// Applies a `Direction` to the `NPosition`, ie. moves to the said direction, if possible.
    /// ```rust
    ///    use lib::{nposition::NPosition, direction::Direction};
    ///
    ///    let mut pos = NPosition::default();
    ///    let res = pos.apply_direction(&Direction::Down);
    ///    assert!(res.is_ok());
    ///    assert_eq!(pos, NPosition::new(0, -1));
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

    use super::{Error, NPosition};

    #[test]
    fn try_from() {
        let pos = NPosition::try_from((-1i8, 2u32));

        assert!(matches!(pos, Ok(NPosition { x: -1, y: 2 })));
    }

    #[test]
    fn apply_direction() {
        // Trivial tests
        let mut pos = NPosition::default();
        assert!(pos.apply_direction(&Direction::Up).is_ok());
        assert_eq!((pos.x(), pos.y()), (0, 1));
        assert!(pos.apply_direction(&Direction::Right).is_ok());
        assert_eq!((pos.x(), pos.y()), (1, 1));
        assert!(pos.apply_direction(&Direction::Down).is_ok());
        assert_eq!((pos.x(), pos.y()), (1, 0));
        assert!(pos.apply_direction(&Direction::Left).is_ok());
        assert_eq!((pos.x(), pos.y()), (0, 0));

        // Limit testing
        let mut pos = NPosition::new(isize::MIN, 0);
        assert!(matches!(
            pos.apply_direction(&Direction::Left),
            Err(LibError::NPosition(Error::Apply(
                Direction::Left,
                NPosition {
                    x: isize::MIN,
                    y: 0,
                },
            )))
        ));

        let mut pos = NPosition::new(isize::MAX, 0);
        assert!(matches!(
            pos.apply_direction(&Direction::Right),
            Err(LibError::NPosition(Error::Apply(
                Direction::Right,
                NPosition {
                    x: isize::MAX,
                    y: 0,
                },
            )))
        ));

        let mut pos = NPosition::new(0, isize::MIN);
        assert!(matches!(
            pos.apply_direction(&Direction::Down),
            Err(LibError::NPosition(Error::Apply(
                Direction::Down,
                NPosition {
                    x: 0,
                    y: isize::MIN,
                },
            )))
        ));

        let mut pos = NPosition::new(0, isize::MAX);
        assert!(matches!(
            pos.apply_direction(&Direction::Up),
            Err(LibError::NPosition(Error::Apply(
                Direction::Up,
                NPosition {
                    x: 0,
                    y: isize::MAX,
                },
            )))
        ));
    }
}
