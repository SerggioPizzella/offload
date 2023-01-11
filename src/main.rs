#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::config::Environment;

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{Data, State};

#[post("/<name>", data = "<content>")]
fn upload(name: String, content: Data, id_seed: State<IdSeed>) -> Result<String, std::io::Error>{
	let id = id_seed.seed.load(Ordering::Relaxed);
	id_seed.seed.fetch_add(1, Ordering::Relaxed);
    
	let filepath = format!("upload/{}-{}", name, id);

	content.stream_to_file(Path::new(&filepath))?;

	Ok(filepath)
}
struct IdSeed {
	seed: AtomicUsize
}

impl IdSeed {
	fn new(value: usize) -> IdSeed {
		IdSeed { seed: AtomicUsize::new(value) }
	}
}

fn main() {
    let ip = local_ip_address::local_ip().unwrap().to_string();

    let config = rocket::Config::build(Environment::Development)
        .address(ip)
        .port(8000)
        .unwrap();

    rocket::custom(config)
		.manage(IdSeed::new(0))
        .mount("/upload", routes![upload])
        .launch();
}
