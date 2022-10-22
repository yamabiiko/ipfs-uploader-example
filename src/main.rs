use std::env;
use std::process;

use ipfs_uploader::Cfg;

#[tokio::main]
async fn main() {

    let cfg = Cfg::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = ipfs_uploader::run(cfg).await {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
