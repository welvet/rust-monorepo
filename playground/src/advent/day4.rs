use std::collections::HashMap;

mod utils;

fn main() {
    let input = utils::read_input("playground/data/advent/day4.txt").unwrap();
    let fields: Vec<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|&x| x.to_string())
        .collect();

    let count = split_to_passports(input)
        .iter()
        .map(|x| split_to_attributes(x.to_owned()))
        .filter(|x| is_passport_valid(fields.clone(), x.to_owned()))
        .count();

    println!("Result: {}", count)
}

fn split_to_passports(input: Vec<String>) -> Vec<String> {
    let mut result = vec![];
    let mut curr: Vec<String> = vec![];

    for line in input {
        if line.trim().len() == 0 {
            result.push(curr.join(" "));
            curr = vec![];
        } else {
            curr.push(line);
        }
    }

    result.push(curr.join(" "));

    result
}

fn split_to_attributes(line: String) -> HashMap<String, String> {
    let pairs: HashMap<String, String> = line
        .split(" ")
        .map(|p| {
            let mut parts = p.split(":");
            (
                parts.next().unwrap().to_owned(),
                parts.next().unwrap().to_owned(),
            )
        })
        .into_iter()
        .collect();

    pairs
}

fn is_passport_valid(fields: Vec<String>, input: HashMap<String, String>) -> bool {
    for f in fields {
        if input.get(f.as_str()).is_none() {
            return false;
        }
    }

    true
}
