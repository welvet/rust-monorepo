mod utils;

use std::ops::Range;
use std::str;

fn main() {
    let mut cnt = 0;

    let input = utils::read_input("playground/data/advent/day2.txt").unwrap();

    for line in input {
        let (rules, value) = two_parts(&line, ":");
        let (len, ssymbol) = two_parts(&rules, " ");
        let (sfrom, sto) = two_parts(&len, "-");

        let from = sfrom
            .parse::<u32>()
            .expect(&format!("Unable to parse {}", sfrom));
        let to = sto
            .parse::<u32>()
            .expect(&format!("Unable to parse {}", sto));

        let symbol = ssymbol.chars().next().unwrap();
        if verify2(
            &value,
            Range {
                start: from,
                end: to,
            },
            symbol,
        ) {
            cnt += 1;
        } else {
            println!("Failed {}", line)
        }
    }

    println!("Result: {}", cnt);
}

fn two_parts(input: &str, delim: &str) -> (String, String) {
    let input_parts: Vec<&str> = input.trim().split(delim).collect();
    let p1 = input_parts.get(0).unwrap().trim();
    let p2 = input_parts.get(1).unwrap().trim();

    return (p1.parse().unwrap(), p2.parse().unwrap());
}

#[allow(dead_code)]
fn verify(input: &str, r: Range<u32>, symbol: char) -> bool {
    let count = input.chars().filter(|c| *c == symbol).count();
    r.contains(&(count as u32))
}

fn verify2(input: &str, r: Range<u32>, symbol: char) -> bool {
    let mut found_a = false;
    let mut found_b = false;

    for (pos, c) in input.chars().enumerate() {
        let upos = (pos as u32) + 1;

        if upos == r.start {
            found_a = c == symbol
        } else if upos == r.end {
            found_b = c == symbol
        }
    }

    found_a ^ found_b
}

#[test]
fn test_two_parts() {
    let (p1, p2) = two_parts("asd: ss", ":");

    assert_eq!(p1, "asd");
    assert_eq!(p2, "ss");
}

#[test]
fn test_verify2() {
    assert!(verify2("abcde", Range { start: 1, end: 3 }, 'a'));
    assert!(!verify2("cdefg", Range { start: 1, end: 3 }, 'b'));
    assert!(!verify2("ccccccccc", Range { start: 2, end: 9 }, 'c'));
    assert!(verify2("sgns", Range { start: 2, end: 4 }, 's'));
}
