use std::collections::HashMap;
use std::collections::HashSet;

use crate::parse_input;
use crate::Error;

fn problem01_part1(input: &Input) -> Result<i32, Error> {
    let mut c1 = input.c1.clone();
    let mut c2 = input.c2.clone();

    c1.sort();
    c2.sort();

    let mut diff_sum = 0;
    for i in 0..c1.len() {
        diff_sum += i32::abs(c2[i] - c1[i])
    }

    Ok(diff_sum)
}

fn problem01_part2(input: &Input) -> Result<i32, Error> {
    let c1 = input.c1.clone();
    let c2 = input.c2.clone();

    let mut right_col_counter: HashMap<i32, i32> = HashMap::new();

    for num in c2 {
        let count = match right_col_counter.get(&num) {
            Some(c) => c + 1,
            None => 1,
        };

        right_col_counter.insert(num, count);
    }

    let mut left_col_set: HashSet<i32> = HashSet::new();
    for num in c1 {
        left_col_set.insert(num);
    }

    let mut sum = 0;

    for num in left_col_set {
        match right_col_counter.get(&num) {
            Some(count) => sum += num * count,
            None => (),
        }
    }

    Ok(sum)
}

struct Input {
    /// First column
    c1: Vec<i32>,

    /// Second column
    c2: Vec<i32>,
}

fn process_input(input: Vec<String>) -> Result<Input, Error> {
    let mut c1 = Vec::new();
    let mut c2 = Vec::new();

    for line in input {
        let s = line.as_str();
        let left_num: i32 = match s[..5].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(Error::PreprocessError(
                    "Failed to parse i32 from input".to_string(),
                ))
            }
        };
        c1.push(left_num);

        let right_num: i32 = match s[8..].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(Error::PreprocessError(
                    "Failed to parse i32 from input".to_string(),
                ))
            }
        };
        c2.push(right_num);
    }

    Ok(Input { c1, c2 })
}

pub fn problem01() -> Result<(), Error> {
    let input = parse_input("input/problem_01.txt")?;
    let parsed_input = process_input(input)?;

    let solution1 = problem01_part1(&parsed_input)?;
    println!("Problem 01 Part 1: {solution1}");
    let solution2 = problem01_part2(&parsed_input)?;
    println!("Problem 01 Part 2: {solution2}");

    Ok(())
}
