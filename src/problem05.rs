use std::collections::{HashMap, HashSet};

use crate::{parse_input, Error};

/// Is the list of pages correct? If so what is the middle page?
fn problem05_part1(input: &Input) -> Result<i32, Error> {
    let mut count = 0;

    for pages in &input.page_lists {
        let mut valid_pages = true;

        let mut cur_page_index = 0;
        while valid_pages && cur_page_index < pages.len() {
            let current_page = &pages[cur_page_index];
            let current_page_rules = input.rules.get(current_page).unwrap();

            let pages_preceding = &pages[..cur_page_index];
            let pages_proceeding = &pages[cur_page_index + 1..];

            valid_pages = current_page_rules.valid_preceding_pages(pages_preceding)
                && current_page_rules.valid_proceeding_pages(pages_proceeding);

            cur_page_index += 1;
        }

        if valid_pages {
            let len = pages.len();
            let odd = len % 2;
            let middle = ((len - odd) / 2) + odd;
            println!("Len: {len}, Middle: {middle}");
            count += pages[middle];
        }
    }

    Ok(count)
}

fn problem05_part2() -> Result<i32, Error> {
    let mut count = 0;

    Ok(count)
}

#[derive(Debug)]
struct PageRules {
    comes_before: HashSet<i32>,
    comes_after: HashSet<i32>,
}

impl PageRules {
    fn new() -> Self {
        PageRules {
            comes_before: HashSet::new(),
            comes_after: HashSet::new(),
        }
    }

    /// Register that THIS page comes after the specified page
    fn is_preceded_by(&mut self, page: i32) {
        self.comes_after.insert(page);
    }

    /// Register that THIS page comes before the specified page
    fn is_proceeded_by(&mut self, page: i32) {
        self.comes_before.insert(page);
    }

    /// All of these pages must come before THIS page.
    /// If this page is supposed to come before any specified page then this returns false
    fn valid_preceding_pages(&self, pages: &[i32]) -> bool {
        for page in pages {
            if self.comes_before.contains(page) || !self.comes_after.contains(page) {
                return false;
            }
        }
        true
    }

    /// All of these pages must come after THIS page.
    fn valid_proceeding_pages(&self, pages: &[i32]) -> bool {
        for page in pages {
            if self.comes_after.contains(page) || !self.comes_before.contains(page) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Input {
    rules: HashMap<i32, PageRules>,
    page_lists: Vec<Vec<i32>>,
}

fn split_rules_and_pages(input: Vec<String>) -> Result<Input, Error> {
    // Have to parse all the rules here
    let mut rules = HashMap::new();

    let mut index = 0;
    let mut cur_line: &str = &input[index];
    while cur_line != "" {
        let pipe_index = cur_line.find('|').unwrap();
        let left_num: i32 = cur_line[..pipe_index].parse()?;
        let right_num: i32 = cur_line[pipe_index + 1..].parse()?;

        if !rules.contains_key(&left_num) {
            rules.insert(left_num, PageRules::new());
        }
        if !rules.contains_key(&right_num) {
            rules.insert(right_num, PageRules::new());
        }

        // Confusing piece here. The left number comes before the right
        // and the right comes after the left.
        rules
            .get_mut(&left_num)
            .expect("Initialized")
            .is_proceeded_by(right_num);
        rules
            .get_mut(&right_num)
            .expect("Initialized")
            .is_preceded_by(left_num);

        // Progress
        index += 1;
        cur_line = &input[index];
    }

    let mut page_lists = Vec::new();
    for line in &input[index + 1..] {
        let mut pages = Vec::new();
        for num_str in line.split(',') {
            pages.push(num_str.parse()?);
        }
        page_lists.push(pages);
    }

    Ok(Input { rules, page_lists })
}

pub fn problem05() -> Result<(), Error> {
    let input = parse_input("input/problem_05.txt")?;

    let parsed_input = split_rules_and_pages(input)?;

    let solution_one = problem05_part1(&parsed_input)?;
    println!("Problem 05 Part 1: {solution_one}");
    let solution_two = problem05_part2()?;
    println!("Problem 05 Part 2: {solution_two}");

    Ok(())
}
