use std::{env, process};
use offload::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumnets: {}", err);
        process::exit(1);
    });

    offload::run(config);
}
