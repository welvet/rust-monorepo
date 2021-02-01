use std::collections::HashSet;
use std::io;

fn main() {
    let mut nums: HashSet<i32> = HashSet::new();

    loop {
        let mut buffer = String::new();
        let result = io::stdin().read_line(&mut buffer);
        if result.is_err() {
            println!("Error: {}", result.err().unwrap())
        }

        if buffer.len() == 0 {
            break;
        }

        let cur = buffer.trim().parse::<i32>().unwrap();
        if nums.contains(&(2020 - cur)) {
            println!("Result: {}", &(cur * (2020 - cur)))
        }

        nums.insert(cur);
    }
}
