use std::collections::HashMap;

use crate::{
    parse_input,
    structures::{coordinate::Coordinate, grid::Grid},
    utils::numbers::{CheckedAdd, CheckedSub},
    Error,
};

fn problem08_part1(input: &Input) -> Result<u64, Error> {
    let mut grid = input.grid.clone_to_empty();
    for sat_type_coords in input.char_coords.values() {
        // Two finger alg for all pairs
        for i in 0..sat_type_coords.len() {
            for j in i + 1..sat_type_coords.len() {
                let anti_nodes = get_anti_nodes_for_two_coordinates(
                    sat_type_coords[i],
                    sat_type_coords[j],
                    &input.grid,
                );
                for anti_node in anti_nodes {
                    grid.set(anti_node.row, anti_node.column, '#')?;
                }
            }
        }
    }

    Ok(grid.len().try_into()?)
}

fn problem08_part2(input: &Input) -> Result<u64, Error> {
    let mut grid = input.grid.clone_to_empty();
    for sat_type_coords in input.char_coords.values() {
        // Two finger alg for all pairs
        for i in 0..sat_type_coords.len() {
            for j in i + 1..sat_type_coords.len() {
                let anti_nodes =
                    get_inline_anti_nodes(sat_type_coords[i], sat_type_coords[j], &input.grid);
                for anti_node in anti_nodes {
                    grid.set(anti_node.row, anti_node.column, '#')?;
                }
            }
        }
    }

    Ok(grid.len().try_into()?)
}

fn get_anti_nodes_for_two_coordinates(
    a: Coordinate<i64>,
    b: Coordinate<i64>,
    grid: &Grid<i64, char>,
) -> Vec<Coordinate<i64>> {
    let mut anti_nodes = Vec::with_capacity(2);

    // This is the slope moving from a to b
    let delta = a.get_slope_to(&b).expect("Valid Slope");

    // The anti-node near b would be adding the slope to b
    match b.checked_add(delta) {
        Some(anti_b) => {
            if grid.valid_index(anti_b.row, anti_b.column) {
                anti_nodes.push(anti_b);
            }
        }
        None => (),
    }

    // The anti-node near a would be subtracting the slope from a
    match a.checked_sub(delta) {
        Some(anti_a) => {
            if grid.valid_index(anti_a.row, anti_a.column) {
                anti_nodes.push(anti_a);
            }
        }
        None => (),
    }

    anti_nodes
}

fn get_inline_anti_nodes(
    a: Coordinate<i64>,
    b: Coordinate<i64>,
    grid: &Grid<i64, char>,
) -> Vec<Coordinate<i64>> {
    let mut anti_nodes = Vec::new();

    let delta = a.get_slope_to(&b).expect("Valid Slope");

    // Add to b until we're outside the grid
    let mut current_pos = b;
    while grid.valid_index(current_pos.row, current_pos.column) {
        anti_nodes.push(current_pos);
        current_pos = current_pos.checked_add(delta).expect("No overflow");
    }

    // Sub from a until we're outside the grid
    current_pos = a;
    while grid.valid_index(current_pos.row, current_pos.column) {
        anti_nodes.push(current_pos);
        current_pos = current_pos.checked_sub(delta).expect("No underflow");
    }

    anti_nodes
}

struct Input {
    /// A map of unique non '.' characters in the grid and where they are
    char_coords: HashMap<char, Vec<Coordinate<i64>>>,

    /// An empty grid used to register anti-nodes
    grid: Grid<i64, char>,
}

fn get_unique_antenna_coordinates(lines: Vec<String>) -> Result<Input, Error> {
    let mut char_coords: HashMap<char, Vec<Coordinate<i64>>> = HashMap::new();

    let mut row: i64 = 0;
    let mut column: i64 = 0;
    for line in lines {
        column = 0;
        for c in line.chars() {
            // We care about this point
            if c != '.' {
                if !char_coords.contains_key(&c) {
                    char_coords.insert(c, Vec::new());
                }
                char_coords
                    .get_mut(&c)
                    .expect("Initialized")
                    .push(Coordinate::new(row, column));
            }

            column += 1;
        }
        row += 1;
    }

    // The last iteration will equal the value of len
    Ok(Input {
        char_coords,
        grid: Grid::new(row, column),
    })
}

pub fn problem08() -> Result<(), Error> {
    let input = parse_input("input/problem_08.txt")?;

    let mut parsed_input = get_unique_antenna_coordinates(input)?;

    let solution_one = problem08_part1(&parsed_input)?;
    println!("Problem 08 Part 1: {solution_one}");
    let solution_two = problem08_part2(&mut parsed_input)?;
    println!("Problem 08 Part 2: {solution_two}");

    Ok(())
}
