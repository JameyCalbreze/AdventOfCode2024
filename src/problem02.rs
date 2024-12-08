use crate::parse_input;
use crate::Error;

fn report_is_safe(report: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut valid_distance = true;

    // Assuming we always have 2 or more numbers in a report
    for i in 1..report.len() {
        let prev_num = report[i-1];
        let cur_num = report[i];

        // How did we change
        let delta = cur_num - prev_num;

        increasing = increasing && delta > 0;
        decreasing = decreasing && delta < 0;
        valid_distance = valid_distance && delta.abs() >= 1 && delta.abs() <= 3
    }

    return ((increasing && !decreasing) || (!increasing && decreasing)) && valid_distance
}

fn report_valid_with_errors(report: &Vec<i32>) -> bool {
    if report_is_safe(report) {
        return true;
    }

    // Brute force cause... I can 
    for i in 0..report.len() {
        let mut sub_report = Vec::new();
        for j in 0..report.len() {
            if i != j {
                sub_report.push(report[j]);
            }
        }
        if report_is_safe(&sub_report) {
            return true;
        }
    }

    false
}

fn problem02_part1(input: &Input) -> Result<i32, Error> {
    let mut num_safe_reports = 0;
    for report in &input.reports {
        if report_is_safe(report) {
            num_safe_reports += 1;
        }
    }
    Ok(num_safe_reports)
}

fn problem02_part2(input: &Input) -> Result<i32, Error> {
    let mut num_safe_reports = 0;
    for report in &input.reports {
        if report_valid_with_errors(report) {
            num_safe_reports += 1;
        }
    }
    Ok(num_safe_reports)
}

struct Input {
    reports: Vec<Vec<i32>>
}

fn process_input(input: Vec<String>) -> Result<Input, Error> {
    let mut reports = Vec::new();

    for line in input {
        let mut numbers: Vec<i32> = Vec::new();

        let line_bytes = line.as_bytes();
        let mut cur_num_start: usize = 0;
        for i in 0..line_bytes.len() {
            if line_bytes[i] == b' ' {
                numbers.push(line[cur_num_start..i].parse()?);
                cur_num_start = i+1;
            }
        }
        numbers.push(line[cur_num_start..].parse()?);

        reports.push(numbers);
    }

    Ok(Input { reports })
}

pub fn problem02() -> Result<(), Error> {
    let input = parse_input("input/problem_02.txt")?;
    let processed_input = process_input(input)?;

    let solution_one = problem02_part1(&processed_input)?;
    println!("Solution Problem02 Part 1: {solution_one}");
    let solution_two = problem02_part2(&processed_input)?;
    println!("Solution Problem02 Part 2: {solution_two}");

    Ok(())
}
