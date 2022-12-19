use offload::prelude;
use std::{env, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config = prelude::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumnets: {}", err);
        process::exit(1);
    });

    match offload::run(config).await {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("failed: {}", err);
            process::exit(1);
        }
    };
}
