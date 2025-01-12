use std::fmt::Display;

use crate::{parse_input, Error};

/// Is the list of pages correct? If so what is the middle page?
fn problem07_part1(input: &Input) -> Result<u64, Error> {
    let mut count = 0;

    for equation in &input.equations {
        if equation.solvable() {
            count += equation.result;
        }
    }

    Ok(count)
}

fn problem07_part2(input: &Input) -> Result<u64, Error> {
    let mut count = 0;

    for equation in &input.equations {
        if equation.solvable_three_operands() {
            count += equation.result;
        }
    }

    Ok(count)
}

struct Equation {
    result: u64,
    inputs: Vec<u64>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut inputs_str = String::new();
        for val in &self.inputs {
            inputs_str = format!("{} {}", inputs_str, *val);
        }
        f.write_str(format!("Eq: result: {}, inputs: {}", self.result, inputs_str).as_str())
    }
}

impl Equation {
    fn solvable(&self) -> bool {
        let mut previous_values: Vec<u64> = vec![self.inputs[0]];

        // Making an assumption that all input vectors have at least 2 values
        for input_val in self.inputs[1..].iter() {
            let mut next_values: Vec<u64> = Vec::with_capacity(previous_values.len() * 2);

            for val in previous_values {
                match val.checked_add(*input_val) {
                    Some(v) => next_values.push(v),
                    None => (),
                }
                match val.checked_mul(*input_val) {
                    Some(v) => next_values.push(v),
                    None => (),
                }
            }

            previous_values = next_values;
        }

        previous_values.contains(&self.result)
    }

    fn solvable_three_operands(&self) -> bool {
        let mut previous_values: Vec<u64> = vec![self.inputs[0]];

        // Making an assumption that all input vectors have at least 2 values
        for input_val in self.inputs[1..].iter() {
            let mut next_values: Vec<u64> = Vec::with_capacity(previous_values.len() * 2);

            for val in previous_values {
                match val.checked_add(*input_val) {
                    Some(v) => next_values.push(v),
                    None => (),
                }
                match val.checked_mul(*input_val) {
                    Some(v) => next_values.push(v),
                    None => (),
                }
                match format!("{}{}", val, input_val).parse::<u64>() {
                    Ok(v) => next_values.push(v),
                    Err(_) => (),
                }
            }

            previous_values = next_values;
        }

        previous_values.contains(&self.result)
    }
}

struct Input {
    equations: Vec<Equation>,
}

fn get_equations_from_input(lines: Vec<String>) -> Result<Input, Error> {
    let mut equations = Vec::with_capacity(lines.len());

    for line in lines {
        let colon_pos = line.find(':').expect(": is present in all lines");
        let result: u64 = line[0..colon_pos].parse()?;

        let mut inputs: Vec<u64> = Vec::new();
        // +1 for the : and +1 for the first space.
        for input_val in line[colon_pos + 2..].split(' ') {
            inputs.push(input_val.parse()?);
        }
        equations.push(Equation { result, inputs });
    }

    Ok(Input { equations })
}

pub fn problem07() -> Result<(), Error> {
    let input = parse_input("input/problem_07.txt")?;

    let mut parsed_input = get_equations_from_input(input)?;

    let solution_one = problem07_part1(&parsed_input)?;
    println!("Problem 07 Part 1: {solution_one}");
    let solution_two = problem07_part2(&mut parsed_input)?;
    println!("Problem 07 Part 2: {solution_two}");

    Ok(())
}
