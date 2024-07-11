mod backend;
mod frontend;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {}
