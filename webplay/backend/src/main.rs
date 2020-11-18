use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let location = args[1].clone();

    let hello = warp::path!("hello").map(|| format!("Hello, {}!", ""));

    let files = warp::fs::dir(location);

    warp::serve(hello.or(files))
        .tls()
        .cert_path("keys/cert.pem")
        .key_path("keys/key.pem")
        .run(([0, 0, 0, 0], 8080))
        .await;
}
