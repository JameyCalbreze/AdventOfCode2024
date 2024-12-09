use crate::Error;
use crate::parse_input;

fn problem04_part1() -> Result<i32, Error> {
    Ok(0)
}

fn problem04_part2() -> Result<i32, Error> {
    Ok(0)
}

pub fn problem04() -> Result<(), Error> {
    let input = parse_input("input/problem_04.txt")?;

    let solution_one = problem04_part1()?;
    println!("Problem 04 Part 1: {solution_one}");
    let solution_two = problem04_part2()?;
    println!("Problem 04 Part 1: {solution_two}");

    Ok(())
}