use strum_macros::{Display, EnumString, VariantArray};

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

pub type Coordinate = (usize, usize);

pub fn traverse(coordinate: Coordinate, direction: Direction) -> Option<Coordinate> {
    match direction {
        Direction::North => {
            let row = match coordinate.0.checked_sub(1) {
                Some(row) => row,
                None => return None,
            };
            let column = coordinate.1;
            Some((row, column))
        }
        Direction::NorthEast => {
            let row = match coordinate.0.checked_sub(1) {
                Some(row) => row,
                None => return None,
            };
            let column = match coordinate.1.checked_add(1) {
                Some(column) => column,
                None => return None,
            };
            Some((row, column))
        }
        Direction::East => {
            let row = coordinate.0;
            let column = match coordinate.1.checked_add(1) {
                Some(column) => column,
                None => return None,
            };
            Some((row, column))
        }
        Direction::SouthEast => {
            let row = match coordinate.0.checked_add(1) {
                Some(row) => row,
                None => return None,
            };
            let column = match coordinate.1.checked_add(1) {
                Some(column) => column,
                None => return None,
            };
            Some((row, column))
        }
        Direction::South => {
            let row = match coordinate.0.checked_add(1) {
                Some(row) => row,
                None => return None,
            };
            let column = coordinate.1;
            Some((row, column))
        }
        Direction::SouthWest => {
            let row = match coordinate.0.checked_add(1) {
                Some(row) => row,
                None => return None,
            };
            let column = match coordinate.1.checked_sub(1) {
                Some(column) => column,
                None => return None,
            };
            Some((row, column))
        }
        Direction::West => {
            let row = coordinate.0;
            let column = match coordinate.1.checked_sub(1) {
                Some(column) => column,
                None => return None,
            };
            Some((row, column))
        }
        Direction::NorthWest => {
            let row = match coordinate.0.checked_sub(1) {
                Some(row) => row,
                None => return None,
            };
            let column = match coordinate.1.checked_sub(1) {
                Some(column) => column,
                None => return None,
            };
            Some((row, column))
        }
    }
}

mod test {
    use strum::VariantArray;

    use super::*;

    #[test]
    fn traverse_north() {
        assert_eq!(None, traverse((0, 0), Direction::North));
        assert_eq!(None, traverse((0, 1), Direction::North));
        assert_eq!(Some((0, 0)), traverse((1, 0), Direction::North));
        assert_eq!(Some((0, 1)), traverse((1, 1), Direction::North));
    }

    #[test]
    fn traverse_north_east() {
        assert_eq!(None, traverse((0, 0), Direction::NorthEast));
        assert_eq!(None, traverse((0, 1), Direction::NorthEast));
        assert_eq!(Some((0, 1)), traverse((1, 0), Direction::NorthEast));
        assert_eq!(Some((0, 2)), traverse((1, 1), Direction::NorthEast));
    }

    #[test]
    fn traverse_east() {
        assert_eq!(Some((0, 1)), traverse((0, 0), Direction::East));
        assert_eq!(Some((0, 2)), traverse((0, 1), Direction::East));
        assert_eq!(Some((1, 1)), traverse((1, 0), Direction::East));
        assert_eq!(Some((1, 2)), traverse((1, 1), Direction::East));
    }

    #[test]
    fn traverse_south_east() {
        assert_eq!(Some((1, 1)), traverse((0, 0), Direction::SouthEast));
        assert_eq!(Some((1, 2)), traverse((0, 1), Direction::SouthEast));
        assert_eq!(Some((2, 1)), traverse((1, 0), Direction::SouthEast));
        assert_eq!(Some((2, 2)), traverse((1, 1), Direction::SouthEast));
    }

    #[test]
    fn traverse_south() {
        assert_eq!(Some((1, 0)), traverse((0, 0), Direction::South));
        assert_eq!(Some((1, 1)), traverse((0, 1), Direction::South));
        assert_eq!(Some((2, 0)), traverse((1, 0), Direction::South));
        assert_eq!(Some((2, 1)), traverse((1, 1), Direction::South));
    }

    #[test]
    fn traverse_south_west() {
        assert_eq!(None, traverse((0, 0), Direction::SouthWest));
        assert_eq!(Some((1, 0)), traverse((0, 1), Direction::SouthWest));
        assert_eq!(None, traverse((1, 0), Direction::SouthWest));
        assert_eq!(Some((2, 0)), traverse((1, 1), Direction::SouthWest));
    }

    #[test]
    fn traverse_west() {
        assert_eq!(None, traverse((0, 0), Direction::West));
        assert_eq!(Some((0, 0)), traverse((0, 1), Direction::West));
        assert_eq!(None, traverse((1, 0), Direction::West));
        assert_eq!(Some((1, 0)), traverse((1, 1), Direction::West));
    }

    #[test]
    fn traverse_north_west() {
        assert_eq!(None, traverse((0, 0), Direction::NorthWest));
        assert_eq!(None, traverse((0, 1), Direction::NorthWest));
        assert_eq!(None, traverse((1, 0), Direction::NorthWest));
        assert_eq!(Some((0, 0)), traverse((1, 1), Direction::NorthWest));
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
