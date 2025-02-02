use std::collections::{HashMap, HashSet};

use crate::{
    parse_input,
    structures::{
        coordinate::{Coordinate, Direction},
        grid::Grid,
    },
    Error,
};

fn problem06_part1(input: &Input) -> Result<i32, Error> {
    match traverse_grid_from_point(&input.grid, input.starting_pos)? {
        // This should be an error but that'll be a TODO!
        Termination::Cycle(_) => todo!(),

        // What we expect!
        Termination::OutOfBounds(path) => Ok(path.len().try_into()?),
    }
}

fn problem06_part2(input: &mut Input) -> Result<i32, Error> {
    let path = match traverse_grid_from_point(&mut input.grid, input.starting_pos)? {
        // This should be an error but that'll be a TODO!
        Termination::Cycle(_) => todo!(),

        // What we expect!
        Termination::OutOfBounds(path) => path,
    };

    let mut count = 0;
    for position in path.keys() {
        // Ignore the starting spot
        if position == &input.starting_pos {
            continue;
        }

        // This will work
        let _ = input.grid.set(position.row, position.column, '#');

        // Have to traverse
        match traverse_grid_from_point(&input.grid, input.starting_pos)? {
            Termination::Cycle(_) => count += 1,
            Termination::OutOfBounds(_) => (),
        }

        // This will work
        let _ = input.grid.set(position.row, position.column, '.');
    }

    Ok(count)
}

struct Input {
    grid: Grid<usize, char>,
    starting_pos: Coordinate<usize>,
}

enum Termination {
    /// The grid is re-visits a point in the same direction.
    Cycle(HashMap<Coordinate<usize>, HashSet<Direction>>),

    /// The grid traverses off of the plane.
    OutOfBounds(HashMap<Coordinate<usize>, HashSet<Direction>>),
}

fn traverse_grid_from_point(
    grid: &Grid<usize, char>,
    starting_pos: Coordinate<usize>,
) -> Result<Termination, Error> {
    let mut direction = Direction::North;
    let mut position = starting_pos;

    let mut visited: HashMap<Coordinate<usize>, HashSet<Direction>> = HashMap::new();

    // This loop will terminate via a return
    loop {
        // Mark that we've been facing this direction at this position
        if visited.contains_key(&position)
            && visited
                .get(&position)
                .expect("Initialized")
                .contains(&direction)
        {
            return Ok(Termination::Cycle(visited));
        } else if visited.contains_key(&position) {
            visited
                .get_mut(&position)
                .expect("Initialized")
                .insert(direction);
        } else {
            let mut direction_set = HashSet::new();
            direction_set.insert(direction);
            visited.insert(position, direction_set);
        }

        // First attempt to move forward
        let next_position = match position.traverse(direction) {
            Some(pos) => match grid.valid_index(pos.row, pos.column) {
                true => pos,
                // Left the grid in a positive index (Overflow).
                false => return Ok(Termination::OutOfBounds(visited)),
            },

            // We've left the grid in an impossible index (Underflow).
            None => return Ok(Termination::OutOfBounds(visited)),
        };

        // The position is valid within the grid. We expect a value in all positions
        let c = grid
            .get(next_position.row, next_position.column)?
            .expect("All indexes initialized");
        if c == '#' {
            // Rotate
            direction = direction.rotate_90();
        } else {
            // Traverse forward
            position = next_position;
        }
    }
}

fn get_grid_from_input(lines: Vec<String>) -> Result<Input, Error> {
    let mut grid = Grid::new(lines.len(), lines[0].len());
    let mut starting_pos = Coordinate::new(0, 0);

    let mut row = 0;
    for line in lines {
        let mut column = 0;
        for character in line.chars() {
            if character == '^' {
                starting_pos = Coordinate::new(row, column);
                // The starting position will not be written into the grid.
                grid.set(row, column, '.')?;
            } else {
                grid.set(row, column, character)?;
            }
            column += 1;
        }
        row += 1;
    }

    Ok(Input { grid, starting_pos })
}

pub fn problem06() -> Result<(), Error> {
    let input = parse_input("input/problem_06.txt")?;

    let mut parsed_input = get_grid_from_input(input)?;

    let solution_one = problem06_part1(&parsed_input)?;
    println!("Problem 06 Part 1: {solution_one}");
    let solution_two = problem06_part2(&mut parsed_input)?;
    println!("Problem 06 Part 2: {solution_two}");

    Ok(())
}
