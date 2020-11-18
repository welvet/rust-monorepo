use chrono::Local;
use util;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    for _ in 1..10 {
        let start = Local::now();
        println!("{}", util::simple_add(fibonacci(42)));
        println!("{}", Local::now() - start);
    }
}
