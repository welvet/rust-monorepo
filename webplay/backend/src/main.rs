use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let location = args[1].clone();

    let hello = warp::path!("hello").map(|| format!("Hello, {}!", ""));

    let files = warp::fs::dir(location);

    println!("Running server on 8080");

    warp::serve(hello.or(files)).run(([0, 0, 0, 0], 8080)).await;
}
