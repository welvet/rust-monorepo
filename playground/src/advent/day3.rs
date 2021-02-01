mod utils;

fn main() {
    let input = utils::read_input("playground/data/advent/day3.txt").unwrap();
    let count = count_slopes(input.clone(), 1, 1)
        * count_slopes(input.clone(), 3, 1)
        * count_slopes(input.clone(), 5, 1)
        * count_slopes(input.clone(), 7, 1)
        * count_slopes(input.clone(), 1, 2);

    println!("Result: {}", count)
}

fn count_slopes(input: Vec<String>, right: usize, bottom: usize) -> i64 {
    let mut count = 0;
    let mut offset = right;

    for (line_pos, line) in input.iter().skip(bottom).enumerate() {
        if line_pos % bottom != 0 {
            continue;
        }

        let char_pos = offset % line.len();
        if line.chars().nth(char_pos).unwrap() == '#' {
            count += 1;
        }
        offset += right;
    }

    count
}
