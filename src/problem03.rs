use crate::Error;
use crate::parse_input;
use regex::Regex;

fn problem03_part1(input: &Vec<String>) -> Result<i32, Error> {
    let selector = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .expect("hard coded selector for this problem");

    let mut sum = 0;

    for line in input {
        for (_, [left_num, right_num]) in selector.captures_iter(line).map(|c| c.extract()) {
            let left_i32: i32 = left_num.parse()?;
            let right_i32: i32 = right_num.parse()?;
            sum += left_i32 * right_i32
        }
    }

    Ok(sum)
}

fn problem03_part2(input: &Vec<String>) -> Result<i32, Error> {
    let mul_selector = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .expect("hard coded selector for this problem");

    let do_selector = Regex::new(r"do\(\)").expect("Nothing special with this regex");
    let dont_selector = Regex::new(r"don't\(\)").expect("nothing special here");

    let mut mul_enabled = true;
    let mut sum = 0;

    for line in input {
        // Keep track of where we are in this line
        let mut index: usize = 0;

        while index < line.len() {
            let slice = &line[index..];

            if mul_enabled {
                let mul_index = match mul_selector.find(slice) {
                    Some(capture) => capture.start(),
                    // Don't want to miss a "don't()" here. Push out to the end of the slice
                    None => slice.len(),
                };

                let dont_index = match dont_selector.find(slice) {
                    Some(capture) => capture.start(),
                    None => slice.len(),
                };

                // We should process the multiplication
                if mul_index < dont_index {
                    // Pull out values
                    let (_, [left_str, right_str]) = mul_selector
                    .captures_at(slice, mul_index)
                    .expect("Already matched")
                    .extract();

                    // Parse the values
                    let left_i32: i32 = left_str.parse()?;
                    let right_i32: i32 = right_str.parse()?;

                    // Add up
                    sum += left_i32 * right_i32;

                    // Shift to mul and offset to progress
                    index += mul_index + 1;
                } else if dont_index < mul_index {
                    mul_enabled = false;
                    
                    // Shift to don't and offset to progress
                    index += dont_index + 1;
                } else {
                    // Progress to end of slice as there's nothing left to do
                    index += slice.len();
                }               
            } else {
                // End of a do in this line
                let do_index = match do_selector.find(slice) {
                    Some(capture) => {
                        mul_enabled = true;
                        capture.end()
                    },

                    // Nothing to do for the rest of the slice
                    None => slice.len(),
                };
                
                // Progress
                index += do_index;
            }
        }
    }

    Ok(sum)
}

pub fn problem03() -> Result<(), Error> {
    let input = parse_input("input/problem_03.txt")?;
    let solution_one = problem03_part1(&input)?;
    println!("Problem 03 Part 1: {solution_one}");
    let solution_two = problem03_part2(&input)?;
    println!("Problem 03 Part 2: {solution_two}");
    Ok(())
}
