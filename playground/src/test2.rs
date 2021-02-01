use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let mut handle: Option<JoinHandle<()>> = None;
    for i in 1..10000 {
        handle = Some(thread::spawn(|| {
            for _ in 1..100 {
                // println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_secs(1));
            }
        }));
        println!("Created thread {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    if let Some(h) = handle {
        h.join().unwrap();
    }
}
