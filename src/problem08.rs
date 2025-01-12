use std::fmt::Display;

use crate::{parse_input, Error};

fn problem08_part1(input: &Input) -> Result<u64, Error> {
    let mut count = 0;

    Ok(count)
}

fn problem08_part2(input: &Input) -> Result<u64, Error> {
    let mut count = 0;

    Ok(count)
}

struct Input {}

fn get_unique_antenna_coordinates(lines: Vec<String>) -> Result<Input, Error> {
    Ok(Input {})
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
