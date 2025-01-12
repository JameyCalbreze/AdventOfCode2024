use strum_macros::{Display, EnumString, VariantArray};

use crate::utils::numbers::{CheckedAdd, CheckedDecrement, CheckedIncrement, CheckedSub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, VariantArray)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn rotate_90(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }
}

#[derive(Debug, Hash, Copy, Clone)]
pub struct Coordinate<T> {
    pub row: T,
    pub column: T,
}

impl<T> PartialEq for Coordinate<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl<T> Eq for Coordinate<T> where T: Eq {}

impl<T> CheckedSub for Coordinate<T>
where
    T: CheckedSub,
{
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        let row = match self.row.checked_sub(rhs.row) {
            Some(r) => r,
            None => return None,
        };

        let column = match self.column.checked_sub(rhs.column) {
            Some(c) => c,
            None => return None,
        };

        Some(Coordinate::new(row, column))
    }
}

impl<T> CheckedAdd for Coordinate<T>
where
    T: CheckedAdd,
{
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        let row = match self.row.checked_add(rhs.row) {
            Some(r) => r,
            None => return None,
        };

        let column = match self.column.checked_add(rhs.column) {
            Some(c) => c,
            None => return None,
        };

        Some(Coordinate::new(row, column))
    }
}

impl<T> Coordinate<T> {
    pub fn new(row: T, column: T) -> Self {
        Self { row, column }
    }
}

impl<T> Coordinate<T>
where
    T: CheckedIncrement + CheckedDecrement + Copy,
{
    pub fn traverse(&self, direction: Direction) -> Option<Coordinate<T>> {
        match direction {
            Direction::North => {
                let row = match self.row.checked_decrement() {
                    Some(row) => row,
                    None => return None,
                };
                let column = self.column;
                Some(Coordinate { row, column })
            }
            Direction::NorthEast => {
                let row = match self.row.checked_decrement() {
                    Some(row) => row,
                    None => return None,
                };
                let column = match self.column.checked_increment() {
                    Some(column) => column,
                    None => return None,
                };
                Some(Coordinate { row, column })
            }
            Direction::East => {
                let row = self.row;
                let column = match self.column.checked_increment() {
                    Some(column) => column,
                    None => return None,
                };
                Some(Coordinate { row, column })
            }
            Direction::SouthEast => {
                let row = match self.row.checked_increment() {
                    Some(row) => row,
                    None => return None,
                };
                let column = match self.column.checked_increment() {
                    Some(column) => column,
                    None => return None,
                };
                Some(Coordinate { row, column })
            }
            Direction::South => {
                let row = match self.row.checked_increment() {
                    Some(row) => row,
                    None => return None,
                };
                let column = self.column;
                Some(Coordinate { row, column })
            }
            Direction::SouthWest => {
                let row = match self.row.checked_increment() {
                    Some(row) => row,
                    None => return None,
                };
                let column = match self.column.checked_decrement() {
                    Some(column) => column,
                    None => return None,
                };
                Some(Coordinate { row, column })
            }
            Direction::West => {
                let row = self.row;
                let column = match self.column.checked_decrement() {
                    Some(column) => column,
                    None => return None,
                };
                Some(Coordinate { row, column })
            }
            Direction::NorthWest => {
                let row = match self.row.checked_decrement() {
                    Some(row) => row,
                    None => return None,
                };
                let column = match self.column.checked_decrement() {
                    Some(column) => column,
                    None => return None,
                };
                Some(Coordinate { row, column })
            }
        }
    }
}

mod test {
    use strum::VariantArray;

    use super::*;

    #[test]
    fn traverse_north() {
        assert_eq!(
            None,
            Coordinate {
                row: 0 as usize,
                column: 0
            }
            .traverse(Direction::North)
        );
        assert_eq!(
            None,
            Coordinate {
                row: 0 as usize,
                column: 1
            }
            .traverse(Direction::North)
        );
        assert_eq!(
            Some(Coordinate { row: 0, column: 0 }),
            Coordinate {
                row: 1 as usize,
                column: 0
            }
            .traverse(Direction::North)
        );
        assert_eq!(
            Some(Coordinate { row: 0, column: 1 }),
            Coordinate {
                row: 1 as usize,
                column: 1
            }
            .traverse(Direction::North)
        );
    }

    #[test]
    fn traverse_north_east() {
        assert_eq!(
            None,
            Coordinate {
                row: 0 as usize,
                column: 0
            }
            .traverse(Direction::NorthEast)
        );
        assert_eq!(
            None,
            Coordinate {
                row: 0 as usize,
                column: 1
            }
            .traverse(Direction::NorthEast)
        );
        assert_eq!(
            Some(Coordinate { row: 0, column: 1 }),
            Coordinate {
                row: 1 as usize,
                column: 0
            }
            .traverse(Direction::NorthEast)
        );
        assert_eq!(
            Some(Coordinate { row: 0, column: 2 }),
            Coordinate {
                row: 1 as usize,
                column: 1
            }
            .traverse(Direction::NorthEast)
        );
    }

    #[test]
    fn traverse_east() {
        assert_eq!(
            Some(Coordinate { row: 0, column: 1 }),
            Coordinate {
                row: 0 as usize,
                column: 0
            }
            .traverse(Direction::East)
        );
        assert_eq!(
            Some(Coordinate { row: 0, column: 2 }),
            Coordinate {
                row: 0 as usize,
                column: 1
            }
            .traverse(Direction::East)
        );
        assert_eq!(
            Some(Coordinate { row: 1, column: 1 }),
            Coordinate {
                row: 1 as usize,
                column: 0
            }
            .traverse(Direction::East)
        );
        assert_eq!(
            Some(Coordinate { row: 1, column: 2 }),
            Coordinate {
                row: 1 as usize,
                column: 1
            }
            .traverse(Direction::East)
        );
    }

    #[test]
    fn traverse_south_east() {
        assert_eq!(
            Some(Coordinate { row: 1, column: 1 }),
            Coordinate {
                row: 0 as usize,
                column: 0
            }
            .traverse(Direction::SouthEast)
        );
        assert_eq!(
            Some(Coordinate { row: 1, column: 2 }),
            Coordinate {
                row: 0 as usize,
                column: 1
            }
            .traverse(Direction::SouthEast)
        );
        assert_eq!(
            Some(Coordinate { row: 2, column: 1 }),
            Coordinate {
                row: 1 as usize,
                column: 0
            }
            .traverse(Direction::SouthEast)
        );
        assert_eq!(
            Some(Coordinate { row: 2, column: 2 }),
            Coordinate {
                row: 1 as usize,
                column: 1
            }
            .traverse(Direction::SouthEast)
        );
    }

    #[test]
    fn traverse_south() {
        assert_eq!(
            Some(Coordinate { row: 1, column: 0 }),
            Coordinate {
                row: 0 as usize,
                column: 0
            }
            .traverse(Direction::South)
        );
        assert_eq!(
            Some(Coordinate { row: 1, column: 1 }),
            Coordinate {
                row: 0 as usize,
                column: 1
            }
            .traverse(Direction::South)
        );
        assert_eq!(
            Some(Coordinate { row: 2, column: 0 }),
            Coordinate {
                row: 1 as usize,
                column: 0
            }
            .traverse(Direction::South)
        );
        assert_eq!(
            Some(Coordinate { row: 2, column: 1 }),
            Coordinate {
                row: 1 as usize,
                column: 1
            }
            .traverse(Direction::South)
        );
    }

    #[test]
    fn traverse_south_west() {
        assert_eq!(
            None,
            Coordinate {
                row: 0 as usize,
                column: 0
            }
            .traverse(Direction::SouthWest)
        );
        assert_eq!(
            Some(Coordinate { row: 1, column: 0 }),
            Coordinate {
                row: 0 as usize,
                column: 1
            }
            .traverse(Direction::SouthWest)
        );
        assert_eq!(
            None,
            Coordinate {
                row: 1 as usize,
                column: 0
            }
            .traverse(Direction::SouthWest)
        );
        assert_eq!(
            Some(Coordinate { row: 2, column: 0 }),
            Coordinate {
                row: 1 as usize,
                column: 1
            }
            .traverse(Direction::SouthWest)
        );
    }

    #[test]
    fn traverse_west() {
        assert_eq!(
            None,
            Coordinate {
                row: 0 as usize,
                column: 0
            }
            .traverse(Direction::West)
        );
        assert_eq!(
            Some(Coordinate { row: 0, column: 0 }),
            Coordinate {
                row: 0 as usize,
                column: 1
            }
            .traverse(Direction::West)
        );
        assert_eq!(
            None,
            Coordinate {
                row: 1 as usize,
                column: 0
            }
            .traverse(Direction::West)
        );
        assert_eq!(
            Some(Coordinate { row: 1, column: 0 }),
            Coordinate {
                row: 1 as usize,
                column: 1
            }
            .traverse(Direction::West)
        );
    }

    #[test]
    fn traverse_north_west() {
        assert_eq!(
            None,
            Coordinate {
                row: 0 as usize,
                column: 0
            }
            .traverse(Direction::NorthWest)
        );
        assert_eq!(
            None,
            Coordinate {
                row: 0 as usize,
                column: 1
            }
            .traverse(Direction::NorthWest)
        );
        assert_eq!(
            None,
            Coordinate {
                row: 1 as usize,
                column: 0
            }
            .traverse(Direction::NorthWest)
        );
        assert_eq!(
            Some(Coordinate { row: 0, column: 0 }),
            Coordinate {
                row: 1 as usize,
                column: 1
            }
            .traverse(Direction::NorthWest)
        );
    }

    #[test]
    fn direction_cycles_with_rotate_90() {
        for direction in Direction::VARIANTS {
            assert_eq!(
                direction,
                &direction.rotate_90().rotate_90().rotate_90().rotate_90()
            );
        }
    }
}
