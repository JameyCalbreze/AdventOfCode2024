use std::collections::HashMap;

use thiserror::Error;

use super::coordinate::{Coordinate, Direction};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// Either a row or a height index is out of bounds
    #[error("{0}, {1} is out of bounds")]
    IndexOutOfBounds(String, usize),

    /// When attempting to traverse to an index which does not fit in usize
    #[error("Attempted to traverse to an impossible index")]
    TraversalError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T>
where
    T: Sized + Copy,
{
    rows: usize,
    columns: usize,
    data: HashMap<(usize, usize), T>,
}

impl<T> Grid<T>
where
    T: Sized + Copy,
{
    /// Don't initialize this with zero values. That's gonna break stuff!
    pub fn new(rows: usize, columns: usize) -> Self {
        Grid {
            rows,
            columns,
            data: HashMap::new(),
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Read out the value at the specified index. Returned value is copied if present
    /// and will not return a reference
    pub fn get(&self, row: usize, column: usize) -> Result<Option<T>, Error> {
        self.valid_row(row)?;
        self.valid_column(column)?;
        Ok(match self.data.get(&(row, column)) {
            Some(t) => Some(*t),
            None => None,
        })
    }

    pub fn set(&mut self, row: usize, column: usize, val: T) -> Result<(), Error> {
        self.valid_row(row)?;
        self.valid_column(column)?;
        self.data.insert((row, column), val);
        Ok(())
    }

    /// Assuming all indecies are valid we're going read values traversing the grid
    /// in a provided direction and return those values in the order they were read
    /// returns an optional vector of type T. Some(_) returned if all traversed indecies
    /// have values present. Otherwise we return None
    pub fn get_strip(
        &self,
        row: usize,
        column: usize,
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

    pub fn valid_index(&self, row: usize, column: usize) -> bool {
        self.valid_row(row).is_ok() && self.valid_column(column).is_ok()
    }

    // Private methods

    fn valid_row(&self, row: usize) -> Result<(), Error> {
        if row >= self.rows {
            Err(Error::IndexOutOfBounds("Row".to_string(), row))
        } else {
            Ok(())
        }
    }

    fn valid_column(&self, column: usize) -> Result<(), Error> {
        if column >= self.columns {
            Err(Error::IndexOutOfBounds("Column".to_string(), column))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_grid() -> Result<(), Error> {
        let grid: Grid<i32> = Grid::new(0, 0);
        Ok(())
    }

    #[test]
    fn test_get_from_grid() -> Result<(), Error> {
        let grid: Grid<i32> = Grid::new(1, 1);
        assert_eq!(None, grid.get(0, 0)?);
        Ok(())
    }

    #[test]
    fn test_set_and_get_from_grid() -> Result<(), Error> {
        let mut grid: Grid<i32> = Grid::new(10, 10);
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
        let grid: Grid<u8> = Grid::new(10, 10);
        assert_eq!(
            Err(Error::IndexOutOfBounds("Row".to_string(), 11)),
            grid.get(11, 0)
        );
        assert_eq!(
            Err(Error::IndexOutOfBounds("Column".to_string(), 11)),
            grid.get(0, 11)
        );
        Ok(())
    }
}
