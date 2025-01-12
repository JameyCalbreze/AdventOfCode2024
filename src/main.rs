use std::{
    fs::File,
    io::Read,
    num::{ParseIntError, TryFromIntError},
};

use thiserror::Error;

mod problem01;
mod problem02;
mod problem03;
mod problem04;
mod problem05;
mod problem06;
mod structures;
mod utils;

#[derive(Debug, Error)]
pub enum Error {
    /// Couldn't read in the contents of the file
    #[error("Failed to parse file: {0}")]
    ParseError(String),

    #[error("Failed to pre-process input: {0}")]
    PreprocessError(String),

    /// Helpful for ? operator
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    /// Failure to initialize the grid for this problem
    #[error(transparent)]
    GridInitFailure(#[from] structures::grid::Error),

    /// Failed to convert usize to i32
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
}

/// Read the given file and return a vector of strings
/// where each line of the file is one string in the vector
pub fn parse_input(file_name: &str) -> Result<Vec<String>, Error> {
    let mut f = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => return Err(Error::ParseError(String::from(file_name))),
    };

    let mut raw_bytes: Vec<u8> = Vec::new();

    // Read in all bytes into our buffer
    match f.read_to_end(&mut raw_bytes) {
        Ok(_) => (),
        Err(_) => return Err(Error::ParseError(String::from(file_name))),
    };

    // Now we'll break each line into it's own string.
    let mut lines: Vec<String> = Vec::new();

    // Convert each line in to a string
    let mut cur_line: Vec<u8> = Vec::new();
    for character in raw_bytes {
        if character == b'\n' {
            lines.push(unsafe { String::from_utf8_unchecked(cur_line) });
            cur_line = Vec::new();
        } else {
            cur_line.push(character);
        }
    }

    // Avoid any empty last line
    if !cur_line.is_empty() {
        lines.push(unsafe { String::from_utf8_unchecked(cur_line) });
    }

    Ok(lines)
}

fn main() {
    let _ = problem01::problem01();
    let _ = problem02::problem02();
    let _ = problem03::problem03();
    let _ = problem04::problem04();
    let _ = problem05::problem05();
    match problem06::problem06() {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    println!("Hello, world!");
}
