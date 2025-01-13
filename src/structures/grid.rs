use std::{collections::HashMap, fmt::Display, hash::Hash};

use thiserror::Error;

use crate::utils::numbers::{CheckedDecrement, CheckedIncrement, LessThanZero};

use super::coordinate::{Coordinate, Direction};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// Either a row or a height index is out of bounds
    #[error("{0}")]
    IndexOutOfBounds(String),

    /// When attempting to traverse to an index which does not fit in usize
    #[error("Attempted to traverse to an impossible index")]
    TraversalError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<I, T>
where
    I: Sized + Hash + Eq,
    T: Sized + Copy,
{
    rows: I,
    columns: I,
    data: HashMap<Coordinate<I>, T>,
}

impl<I, T> Grid<I, T>
where
    I: Sized
        + Copy
        + Hash
        + Eq
        + CheckedIncrement
        + CheckedDecrement
        + LessThanZero
        + Ord
        + Display,
    T: Sized + Copy,
{
    /// Don't initialize this with zero values. That's gonna break stuff!
    pub fn new(rows: I, columns: I) -> Self {
        Grid {
            rows,
            columns,
            data: HashMap::new(),
        }
    }

    /// Create a new grid of the same size as this grid
    pub fn clone_to_empty(&self) -> Self {
        Grid {
            rows: self.rows,
            columns: self.columns,
            data: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn rows(&self) -> I {
        self.rows
    }

    pub fn columns(&self) -> I {
        self.columns
    }

    /// Read out the value at the specified index. Returned value is copied if present
    /// and will not return a reference
    pub fn get(&self, row: I, column: I) -> Result<Option<T>, Error> {
        self.assert_valid_index(row, column)?;
        Ok(match self.data.get(&Coordinate::new(row, column)) {
            Some(t) => Some(*t),
            None => None,
        })
    }

    pub fn set(&mut self, row: I, column: I, val: T) -> Result<(), Error> {
        self.assert_valid_index(row, column)?;
        self.data.insert(Coordinate::new(row, column), val);
        Ok(())
    }

    /// Assuming all indecies are valid we're going read values traversing the grid
    /// in a provided direction and return those values in the order they were read
    /// returns an optional vector of type T. Some(_) returned if all traversed indecies
    /// have values present. Otherwise we return None
    pub fn get_strip(
        &self,
        row: I,
        column: I,
        len: usize,
        direction: Direction,
    ) -> Result<Option<Vec<T>>, Error> {
        let mut strip = Vec::with_capacity(len);

        match self.get(row, column)? {
            Some(t) => strip.push(t),
            None => return Ok(None),
        }

        let mut coordinate = Coordinate::new(row, column);
        for _ in 1..len {
            coordinate = match coordinate.traverse(direction) {
                Some(c) => c,
                None => return Err(Error::TraversalError),
            };
            match self.get(coordinate.row, coordinate.column)? {
                Some(t) => strip.push(t),
                None => return Ok(None),
            }
        }

        Ok(Some(strip))
    }

    pub fn valid_index(&self, row: I, column: I) -> bool {
        self.valid_row(row) && self.valid_column(column)
    }

    // Private methods

    fn assert_valid_index(&self, row: I, column: I) -> Result<(), Error> {
        if self.valid_index(row, column) {
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds(format!(
                "Row: {}, Column: {} is invalid",
                row, column
            )))
        }
    }

    fn valid_row(&self, row: I) -> bool {
        if row.less_than_zero() {
            false
        } else if row >= self.rows {
            false
        } else {
            true
        }
    }

    fn valid_column(&self, column: I) -> bool {
        if column.less_than_zero() {
            false
        } else if column >= self.columns {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_grid() -> Result<(), Error> {
        let grid: Grid<usize, i32> = Grid::new(0, 0);
        Ok(())
    }

    #[test]
    fn test_get_from_grid() -> Result<(), Error> {
        let grid: Grid<usize, i32> = Grid::new(1, 1);
        assert_eq!(None, grid.get(0, 0)?);
        Ok(())
    }

    #[test]
    fn test_set_and_get_from_grid() -> Result<(), Error> {
        let mut grid: Grid<usize, i32> = Grid::new(10, 10);
        for row in 0..10 {
            for column in 0..10 {
                assert_eq!(None, grid.get(row, column)?);
            }
        }

        let mut index: i32 = 0;
        for row in 0..10 {
            for column in 0..10 {
                grid.set(row, column, index)?;
                index += 1;
            }
        }
        assert_eq!(100, index);

        index = 0;
        for row in 0..10 {
            for column in 0..10 {
                let value = grid.get(row, column)?.unwrap();
                assert_eq!(index, value);
                index += 1;
            }
        }
        Ok(())
    }

    #[test]
    fn test_invalid_index() -> Result<(), Error> {
        let grid: Grid<usize, u8> = Grid::new(10, 10);
        assert_eq!(
            Err(Error::IndexOutOfBounds(
                "Row: 11, Column: 0 is invalid".to_string()
            )),
            grid.get(11, 0)
        );
        assert_eq!(
            Err(Error::IndexOutOfBounds(
                "Row: 0, Column: 11 is invalid".to_string()
            )),
            grid.get(0, 11)
        );
        Ok(())
    }
}
