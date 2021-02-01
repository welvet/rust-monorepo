use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let vec = reader
        .lines()
        .map(|l| l.expect("Unable to parse line"))
        .collect();

    Ok(vec)
}
