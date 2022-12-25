//! Grid module. This module aims to provide useful and easy to manipulate grids.

use std::fmt::Display;

use colored::Colorize;
use thiserror::Error;

use crate::{
    error::{LibError, LibResult},
    position::Position,
};

/// Error regarding Grid manipulations.
#[derive(Error, Debug)]
pub enum Error {
    /// Interacting with an invalid row.
    #[error("{0} is an invalid row")]
    InvalidRow(usize),
    /// Interacting with an invalid column.
    #[error("{0} is an invalid col")]
    InvalidCol(usize),
    /// Interacting with an invalid position.
    #[error("{0:?} can't be reached")]
    InvalidPosition(Position),
    /// Interacting with an invalid rectangle.
    #[error("{0:?} and {1:?} can't be used to draw a rectangle")]
    InvalidRectangle(Position, Position),
    /// Raised when an error occured while using the TryFrom impl.
    #[error("can't create Grid, Vec size doesn't correspond")]
    TryFrom,
}

impl From<Error> for LibError {
    fn from(value: Error) -> Self {
        LibError::Grid(value)
    }
}

/// A two dimension Grid, with fancy and easy to use methods.
#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<T>,
    nb_row: usize,
    nb_col: usize,
}

/// TryFrom implementation to create a Grid. The second parameter is the width, ie. the number of columns.
/// The Vec must hold the exact number of elements to fill all rows. This means that Vec<T>.len() = cols * rows.
impl<T> TryFrom<(Vec<T>, usize)> for Grid<T> {
    type Error = LibError;

    fn try_from(value: (Vec<T>, usize)) -> Result<Self, Self::Error> {
        let vec_len = value.0.len();
        if vec_len % value.1 == 0 {
            Ok(Self {
                grid: value.0,
                nb_row: vec_len / value.1,
                nb_col: value.1,
            })
        } else {
            Err(LibError::from(Error::TryFrom))
        }
    }
}

impl<T> Grid<T> {
    /// Retrieves a reference to data at a given position. Error is raised if the position is out of the [Grid].
    /// Example
    /// ```rust
    ///    use lib::{grid::Grid, position::Position};
    ///
    ///    let grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.get(&Position::new(1, 1)).unwrap(), &5);
    /// ```
    pub fn get(&self, pos: &Position) -> LibResult<&T> {
        (pos.x() < self.nb_col && pos.y() < self.nb_row)
            .then(|| self.grid.get(pos.y() * self.nb_col + pos.x()))
            .flatten()
            .ok_or_else(|| LibError::from(Error::InvalidPosition(*pos)))
    }

    /// Retrieves a mutable reference to data at a given position. Error is raised if the position is out of the [Grid].
    /// Example
    /// ```rust
    ///    use lib::{grid::Grid, position::Position};
    ///
    ///    let mut grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.get_mut(&Position::new(1, 1)).unwrap(), &mut 5);
    /// ```
    pub fn get_mut(&mut self, pos: &Position) -> LibResult<&mut T> {
        (pos.x() < self.nb_col && pos.y() < self.nb_row)
            .then(|| self.grid.get_mut(pos.y() * self.nb_col + pos.x()))
            .flatten()
            .ok_or_else(|| LibError::from(Error::InvalidPosition(*pos)))
    }

    /// Yields an [Iterator] of references over the row `row`. Error is raised if the row doesn't exist.
    /// Example
    /// ```rust
    ///    use lib::grid::Grid;
    ///
    ///    let grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.iter_row(1).unwrap().collect::<Vec<_>>(), vec![&4, &5, &6]);
    /// ```
    pub fn iter_row(&self, row: usize) -> LibResult<impl Iterator<Item = &T>> {
        (row < self.nb_row)
            .then(|| self.grid[row * self.nb_col..(row * self.nb_col + self.nb_col)].iter())
            .ok_or_else(|| LibError::from(Error::InvalidRow(row)))
    }

    /// Yields an [Iterator] of mutable references over the row `row`. Error is raised if the row doesn't exist.
    /// Example
    /// ```rust
    ///    use lib::grid::Grid;
    ///
    ///    let mut grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.iter_row_mut(1).unwrap().collect::<Vec<_>>(), vec![&mut 4, &mut 5, &mut 6]);
    /// ```
    pub fn iter_row_mut(&mut self, row: usize) -> LibResult<impl Iterator<Item = &mut T>> {
        (row < self.nb_row)
            .then(|| self.grid[row * self.nb_col..(row * self.nb_col + self.nb_col)].iter_mut())
            .ok_or_else(|| LibError::from(Error::InvalidRow(row)))
    }

    /// Yields an [Iterator| of references over the column `col`. Error is raised if the column doesn't exist.
    /// Example
    /// ```rust
    ///    use lib::grid::Grid;
    ///
    ///    let grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.iter_col(1).unwrap().collect::<Vec<_>>(), vec![&2, &5, &8]);
    /// ```
    pub fn iter_col(&self, col: usize) -> LibResult<impl Iterator<Item = &T>> {
        (col < self.nb_col)
            .then(|| self.grid.iter().skip(col).step_by(self.nb_col))
            .ok_or_else(|| LibError::from(Error::InvalidCol(col)))
    }

    /// Yields an [Iterator| of mutable references over the column `col`. Error is raised if the column doesn't exist.
    /// Example
    /// ```rust
    ///    use lib::grid::Grid;
    ///
    ///    let mut grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.iter_col_mut(1).unwrap().collect::<Vec<_>>(), vec![&mut 2, &mut 5, &mut 8]);
    /// ```
    pub fn iter_col_mut(&mut self, col: usize) -> LibResult<impl Iterator<Item = &mut T>> {
        (col < self.nb_col)
            .then(|| self.grid.iter_mut().skip(col).step_by(self.nb_col))
            .ok_or_else(|| LibError::from(Error::InvalidCol(col)))
    }

    /// Yields an [Iterator] of references over the rectangle defined by `top_left` and `bottom_right` positions.
    /// Error is raised if either one of the position is out of the [Grid] or they are miss placed relatively to each other.
    /// Example
    /// ```rust
    ///    use lib::{grid::Grid, position::Position};
    ///
    ///    let grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.iter_rect(Position::new(1, 1), Position::new(2, 2)).unwrap().collect::<Vec<_>>(), vec![&5, &6, &8, &9]);
    /// ```
    pub fn iter_rect(
        &self,
        top_left: Position,
        bottom_right: Position,
    ) -> LibResult<impl Iterator<Item = &T>> {
        if top_left.x() >= self.nb_col || top_left.y() >= self.nb_row {
            Err(LibError::from(Error::InvalidPosition(top_left)))
        } else if bottom_right.x() >= self.nb_col || bottom_right.y() >= self.nb_row {
            Err(LibError::from(Error::InvalidPosition(bottom_right)))
        } else if top_left.x() > bottom_right.x() || top_left.y() > bottom_right.y() {
            Err(LibError::from(Error::InvalidRectangle(
                top_left,
                bottom_right,
            )))
        } else {
            Ok(self.grid
                [top_left.y() * self.nb_col..(bottom_right.y() * self.nb_col + self.nb_col)]
                .chunks_exact(self.nb_col)
                .flat_map(move |row| {
                    row.iter()
                        .skip(top_left.x())
                        .take(bottom_right.x() - top_left.x() + 1)
                }))
        }
    }

    /// Yields an [Iterator] of mutable references over the rectangle defined by `top_left` and `bottom_right` positions.
    /// Error is raised if either one of the position is out of the [Grid] or they are miss placed relatively to each other.
    /// Example
    /// ```rust
    ///    use lib::{grid::Grid, position::Position};
    ///
    ///    let mut grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.iter_rect_mut(Position::new(1, 1), Position::new(2, 2)).unwrap().collect::<Vec<_>>(), vec![&mut 5, &mut 6, &mut 8, &mut 9]);
    /// ```
    pub fn iter_rect_mut(
        &mut self,
        top_left: Position,
        bottom_right: Position,
    ) -> LibResult<impl Iterator<Item = &mut T>> {
        if top_left.x() >= self.nb_col || top_left.y() >= self.nb_row {
            Err(LibError::from(Error::InvalidPosition(top_left)))
        } else if bottom_right.x() >= self.nb_col || bottom_right.y() >= self.nb_row {
            Err(LibError::from(Error::InvalidPosition(bottom_right)))
        } else if top_left.x() > bottom_right.x() || top_left.y() > bottom_right.y() {
            Err(LibError::from(Error::InvalidRectangle(
                top_left,
                bottom_right,
            )))
        } else {
            Ok(self.grid
                [top_left.y() * self.nb_col..(bottom_right.y() * self.nb_col + self.nb_col)]
                .chunks_exact_mut(self.nb_col)
                .flat_map(move |row| {
                    row.iter_mut()
                        .skip(top_left.x())
                        .take(bottom_right.x() - top_left.x() + 1)
                }))
        }
    }

    /// Yields an [Iterator| of references over the whole [Grid]. The data is traversed row by row.
    /// Example
    /// ```rust
    ///    use lib::grid::Grid;
    ///
    ///    let grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.iter().collect::<Vec<_>>(), vec![
    ///            &1, &2, &3, &4, &5, &6, &7, &8, &9
    ///        ]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    /// Yields an [Iterator| of mutable references over the whole [Grid]. The data is traversed row by row.
    /// Example
    /// ```rust
    ///    use lib::grid::Grid;
    ///
    ///    let mut grid = Grid::try_from((vec![
    ///            1, 2, 3, 4, 5, 6, 7, 8, 9
    ///        ], 3)).unwrap();
    ///    assert_eq!(grid.iter_mut().collect::<Vec<_>>(), vec![
    ///            &mut 1, &mut 2, &mut 3, &mut 4, &mut 5, &mut 6, &mut 7, &mut 8, &mut 9
    ///        ]);
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.grid.iter_mut()
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut top_bot = "-".repeat(self.nb_col + 2);
        top_bot.push('\n');
        f.write_str(&top_bot)?;

        for (i, b) in self.grid.iter().enumerate() {
            if i % self.nb_col == 0 {
                f.write_str("|")?;
            }
            match b {
                true => write!(f, "{}", "t".green())?,
                false => write!(f, "{}", "f".red())?,
            }
            if i % self.nb_col == self.nb_col - 1 {
                f.write_str("|\n")?;
            }
        }

        f.write_str(&top_bot)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        error::LibError,
        grid::{Error, Grid},
        position::Position,
    };

    #[test]
    fn try_from() -> anyhow::Result<()> {
        let grid_elems = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Can't create a Grid of 4 rows from 9 elements.
        let g1 = Grid::try_from((grid_elems.clone(), 4));
        assert!(matches!(g1, Err(LibError::Grid(Error::TryFrom))));

        // Can't create a Grid having more rows than elements.
        let g2 = Grid::try_from((grid_elems.clone(), 12));
        assert!(matches!(g2, Err(LibError::Grid(Error::TryFrom))));

        // Can create a Grid with 1 row.
        let g3 = Grid::try_from((grid_elems.clone(), 1));
        assert!(matches!(
            g3,
            Ok(Grid {
                grid: _,
                nb_row: 9,
                nb_col: 1
            })
        ));

        // Can create a Grid with 3 rows.
        let g4 = Grid::try_from((grid_elems.clone(), 3));
        assert!(matches!(
            g4,
            Ok(Grid {
                grid: _,
                nb_row: 3,
                nb_col: 3
            })
        ));

        // Can create a Grid with 3 rows.
        let g5 = Grid::try_from((grid_elems.clone(), 9));
        assert!(matches!(
            g5,
            Ok(Grid {
                grid: _,
                nb_row: 1,
                nb_col: 9
            })
        ));

        Ok(())
    }

    #[test]
    fn get() -> anyhow::Result<()> {
        let g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        // Valid positions.
        assert!(matches!(g.get(&Position::new(0, 0)), Ok(&1)));
        assert!(matches!(g.get(&Position::new(1, 1)), Ok(&5)));
        assert!(matches!(g.get(&Position::new(2, 2)), Ok(&9)));
        assert!(matches!(g.get(&Position::new(2, 5)), Ok(&18)));
        // 7th row doesn't exist.
        let pos1 = Position::new(1, 7);
        assert!(matches!(
            g.get(&pos1),
            Err(LibError::Grid(Error::InvalidPosition(_pos1)))
        ));
        // 4th column doesn't exist.
        let pos2 = Position::new(4, 1);
        assert!(matches!(
            g.get(&pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos2)))
        ));

        Ok(())
    }

    #[test]
    fn get_mut() -> anyhow::Result<()> {
        let mut g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        // Valid positions.
        assert!(matches!(g.get_mut(&Position::new(0, 0)), Ok(&mut 1)));
        assert!(matches!(g.get_mut(&Position::new(1, 1)), Ok(&mut 5)));
        assert!(matches!(g.get_mut(&Position::new(2, 2)), Ok(&mut 9)));
        assert!(matches!(g.get_mut(&Position::new(2, 5)), Ok(&mut 18)));
        // 7th row doesn't exist.
        let pos1 = Position::new(1, 7);
        assert!(matches!(
            g.get_mut(&pos1),
            Err(LibError::Grid(Error::InvalidPosition(_pos1)))
        ));
        // 4th column doesn't exist.
        let pos2 = Position::new(4, 1);
        assert!(matches!(
            g.get_mut(&pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos2)))
        ));

        Ok(())
    }

    #[test]
    fn iter_row() -> anyhow::Result<()> {
        let g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        assert_eq!(g.iter_row(0)?.collect::<Vec<_>>(), vec![&1, &2, &3]);
        assert_eq!(g.iter_row(2)?.collect::<Vec<_>>(), vec![&7, &8, &9]);
        assert_eq!(g.iter_row(5)?.collect::<Vec<_>>(), vec![&16, &17, &18]);

        assert!(matches!(
            g.iter_row(6),
            Err(LibError::Grid(Error::InvalidRow(6)))
        ));

        Ok(())
    }

    #[test]
    fn iter_row_mut() -> anyhow::Result<()> {
        let mut g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        assert_eq!(
            g.iter_row_mut(0)?.collect::<Vec<_>>(),
            vec![&mut 1, &mut 2, &mut 3]
        );
        assert_eq!(
            g.iter_row_mut(2)?.collect::<Vec<_>>(),
            vec![&mut 7, &mut 8, &mut 9]
        );
        assert_eq!(
            g.iter_row_mut(5)?.collect::<Vec<_>>(),
            vec![&mut 16, &mut 17, &mut 18]
        );

        assert!(matches!(
            g.iter_row_mut(6),
            Err(LibError::Grid(Error::InvalidRow(6)))
        ));

        Ok(())
    }

    #[test]
    fn iter_col() -> anyhow::Result<()> {
        let g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        assert_eq!(
            g.iter_col(0)?.collect::<Vec<_>>(),
            vec![&1, &4, &7, &10, &13, &16]
        );
        assert_eq!(
            g.iter_col(1)?.collect::<Vec<_>>(),
            vec![&2, &5, &8, &11, &14, &17]
        );
        assert_eq!(
            g.iter_col(2)?.collect::<Vec<_>>(),
            vec![&3, &6, &9, &12, &15, &18]
        );

        assert!(matches!(
            g.iter_col(3),
            Err(LibError::Grid(Error::InvalidCol(3)))
        ));

        Ok(())
    }

    #[test]
    fn iter_col_mut() -> anyhow::Result<()> {
        let mut g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        assert_eq!(
            g.iter_col_mut(0)?.collect::<Vec<_>>(),
            vec![&mut 1, &mut 4, &mut 7, &mut 10, &mut 13, &mut 16]
        );
        assert_eq!(
            g.iter_col_mut(1)?.collect::<Vec<_>>(),
            vec![&mut 2, &mut 5, &mut 8, &mut 11, &mut 14, &mut 17]
        );
        assert_eq!(
            g.iter_col_mut(2)?.collect::<Vec<_>>(),
            vec![&mut 3, &mut 6, &mut 9, &mut 12, &mut 15, &mut 18]
        );

        assert!(matches!(
            g.iter_col_mut(3),
            Err(LibError::Grid(Error::InvalidCol(3)))
        ));

        Ok(())
    }

    #[test]
    fn iter_rect() -> anyhow::Result<()> {
        let g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        assert_eq!(
            g.iter_rect(Position::new(1, 3), Position::new(2, 5))?
                .collect::<Vec<_>>(),
            vec![&11, &12, &14, &15, &17, &18]
        );
        assert_eq!(
            g.iter_rect(Position::new(0, 1), Position::new(2, 3))?
                .collect::<Vec<_>>(),
            vec![&4, &5, &6, &7, &8, &9, &10, &11, &12]
        );

        // When top_left is out of grid.
        let (pos1, pos2) = (Position::new(1, 7), Position::new(3, 5));
        assert!(matches!(
            g.iter_rect(pos1, pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos1)))
        ));
        let (pos1, pos2) = (Position::new(4, 1), Position::new(3, 5));
        assert!(matches!(
            g.iter_rect(pos1, pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos1)))
        ));

        // When bottom_right is out of grid.
        let (pos1, pos2) = (Position::new(1, 1), Position::new(3, 7));
        assert!(matches!(
            g.iter_rect(pos1, pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos2)))
        ));
        let (pos1, pos2) = (Position::new(1, 1), Position::new(3, 7));
        assert!(matches!(
            g.iter_rect(pos1, pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos2)))
        ));

        // When top_left and bottom_right are on the same column.
        let (pos1, pos2) = (Position::new(2, 1), Position::new(2, 5));
        assert_eq!(
            g.iter_rect(pos1, pos2)?.collect::<Vec<_>>(),
            vec![&6, &9, &12, &15, &18]
        );

        // When top_left and bottom_right are on the same row.
        let (pos1, pos2) = (Position::new(1, 2), Position::new(2, 2));
        assert_eq!(g.iter_rect(pos1, pos2)?.collect::<Vec<_>>(), vec![&8, &9]);

        // When "top_left" and "bottom_right" are inverted.
        let (pos1, pos2) = (Position::new(2, 5), Position::new(1, 3));
        assert!(matches!(
            g.iter_rect(pos1, pos2),
            Err(LibError::Grid(Error::InvalidRectangle(_pos1, _pos2)))
        ));

        // When "top_left" and "bottom_right" are the same.
        let (pos1, pos2) = (Position::new(2, 3), Position::new(2, 3));
        assert_eq!(g.iter_rect(pos1, pos2)?.collect::<Vec<_>>(), vec![&12]);

        Ok(())
    }

    #[test]
    fn iter_rect_mut() -> anyhow::Result<()> {
        let mut g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        assert_eq!(
            g.iter_rect_mut(Position::new(1, 3), Position::new(2, 5))?
                .collect::<Vec<_>>(),
            vec![&mut 11, &mut 12, &mut 14, &mut 15, &mut 17, &mut 18]
        );
        assert_eq!(
            g.iter_rect_mut(Position::new(0, 1), Position::new(2, 3))?
                .collect::<Vec<_>>(),
            vec![&mut 4, &mut 5, &mut 6, &mut 7, &mut 8, &mut 9, &mut 10, &mut 11, &mut 12]
        );

        // When top_left is out of grid.
        let (pos1, pos2) = (Position::new(1, 7), Position::new(3, 5));
        assert!(matches!(
            g.iter_rect_mut(pos1, pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos1)))
        ));
        let (pos1, pos2) = (Position::new(4, 1), Position::new(3, 5));
        assert!(matches!(
            g.iter_rect_mut(pos1, pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos1)))
        ));

        // When bottom_right is out of grid.
        let (pos1, pos2) = (Position::new(1, 1), Position::new(3, 7));
        assert!(matches!(
            g.iter_rect_mut(pos1, pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos2)))
        ));
        let (pos1, pos2) = (Position::new(1, 1), Position::new(3, 7));
        assert!(matches!(
            g.iter_rect_mut(pos1, pos2),
            Err(LibError::Grid(Error::InvalidPosition(_pos2)))
        ));

        // When top_left and bottom_right are on the same column.
        let (pos1, pos2) = (Position::new(2, 1), Position::new(2, 5));
        assert_eq!(
            g.iter_rect_mut(pos1, pos2)?.collect::<Vec<_>>(),
            vec![&mut 6, &mut 9, &mut 12, &mut 15, &mut 18]
        );

        // When top_left and bottom_right are on the same row.
        let (pos1, pos2) = (Position::new(1, 2), Position::new(2, 2));
        assert_eq!(
            g.iter_rect_mut(pos1, pos2)?.collect::<Vec<_>>(),
            vec![&mut 8, &mut 9]
        );

        // When "top_left" and "bottom_right" are inverted.
        let (pos1, pos2) = (Position::new(2, 5), Position::new(1, 3));
        assert!(matches!(
            g.iter_rect_mut(pos1, pos2),
            Err(LibError::Grid(Error::InvalidRectangle(_pos1, _pos2)))
        ));

        // When "top_left" and "bottom_right" are the same.
        let (pos1, pos2) = (Position::new(2, 3), Position::new(2, 3));
        assert_eq!(
            g.iter_rect_mut(pos1, pos2)?.collect::<Vec<_>>(),
            vec![&mut 12]
        );

        Ok(())
    }

    #[test]
    fn iter() -> anyhow::Result<()> {
        let g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        assert_eq!(
            g.iter().collect::<Vec<_>>(),
            vec![&1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13, &14, &15, &16, &17, &18,]
        );

        Ok(())
    }

    #[test]
    fn iter_mut() -> anyhow::Result<()> {
        let mut g = Grid::try_from((
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            ],
            3,
        ))?;

        assert_eq!(
            g.iter_mut().collect::<Vec<_>>(),
            vec![
                &mut 1, &mut 2, &mut 3, &mut 4, &mut 5, &mut 6, &mut 7, &mut 8, &mut 9, &mut 10,
                &mut 11, &mut 12, &mut 13, &mut 14, &mut 15, &mut 16, &mut 17, &mut 18,
            ]
        );

        Ok(())
    }
}
