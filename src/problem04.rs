use strum::VariantArray;

use crate::structures::coordinate::{Coordinate, Direction};
use crate::structures::grid::Grid;
use crate::{parse_input, Error};

fn problem04_part1(grid: &Grid<usize, char>) -> Result<i32, Error> {
    let mut count = 0;

    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            // println!("Row: {row}, Column: {column}");
            count += count_xmas_at_row_column(row, column, grid)
        }
    }

    Ok(count)
}

fn problem04_part2(grid: &Grid<usize, char>) -> Result<i32, Error> {
    let mut count = 0;

    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            // println!("Row: {row}, Column: {column}");
            if let Some(c) = grid.get(row, column)? {
                if c == 'A' && index_has_x_mas(row, column, grid)? {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

const X_MAS_STRS: &[&str] = &["MMSS", "MSSM", "SSMM", "SMMS"];

fn index_has_x_mas(row: usize, column: usize, grid: &Grid<usize, char>) -> Result<bool, Error> {
    let mut strip: Vec<char> = Vec::new();

    let mut coordinates = Vec::new();
    coordinates.push(Coordinate::new(row, column).traverse(Direction::NorthEast));
    coordinates.push(Coordinate::new(row, column).traverse(Direction::SouthEast));
    coordinates.push(Coordinate::new(row, column).traverse(Direction::SouthWest));
    coordinates.push(Coordinate::new(row, column).traverse(Direction::NorthWest));

    for coordinate in coordinates {
        match coordinate {
            None => return Ok(false),
            Some(c) => {
                if let Ok(Some(c)) = grid.get(c.row, c.column) {
                    strip.push(c)
                }
            }
        }
    }

    let strip_str: String = strip.into_iter().collect();
    Ok(X_MAS_STRS.contains(&strip_str.as_str()))
}

fn count_xmas_at_row_column(row: usize, column: usize, grid: &Grid<usize, char>) -> i32 {
    let mut count = 0;

    // There are 8 possible directions to go.
    for direction in Direction::VARIANTS {
        if let Ok(Some(strip)) = grid.get_strip(row, column, 4, *direction) {
            let strip_str: String = strip.into_iter().collect();
            if "XMAS" == strip_str {
                count += 1;
            }
        }
    }

    count
}

fn init_grid_from_input(input: Vec<String>) -> Result<Grid<usize, char>, Error> {
    let rows: usize = input.len();
    let columns: usize = input[0].len();
    let mut grid = Grid::new(rows, columns);

    let mut row = 0;
    for line in input {
        let mut column = 0;
        for c in line.chars() {
            grid.set(row, column, c)?;
            column += 1;
        }
        row += 1;
    }

    Ok(grid)
}

pub fn problem04() -> Result<(), Error> {
    let input = parse_input("input/problem_04.txt")?;

    let grid = init_grid_from_input(input)?;

    let solution_one = problem04_part1(&grid)?;
    println!("Problem 04 Part 1: {solution_one}");
    let solution_two = problem04_part2(&grid)?;
    println!("Problem 04 Part 2: {solution_two}");

    Ok(())
}
