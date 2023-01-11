use rocket::{Data, State};
use rocket::data::ToByteUnit;

use std::io;
use std::path::Path;
use std::sync::atomic::Ordering;

use offload::prelude::*;
use crate::id_seed::IdSeed;


#[post("/<name>", data = "<content>")]
pub async fn upload_task(name: String, content: Data<'_>, id_seed: &State<IdSeed>) -> Result<String, std::io::Error> {
	let id = id_seed.seed.load(Ordering::Relaxed);
	id_seed.seed.fetch_add(1, Ordering::Relaxed);
    
	let filepath = format!("uploads/{}-{}", name, id);

	content.open(5.megabytes()).into_file(Path::new(&filepath)).await?;

	Ok(filepath)
}

#[post("/args/<name>", data = "<args>")]
pub async fn upload_args(name: String, args: Data<'_>, id_seed: &State<IdSeed>) -> Result<(), std::io::Error>{
	let id = id_seed.seed.load(Ordering::Relaxed);
	id_seed.seed.fetch_add(1, Ordering::Relaxed);

	let filepath = format!("uploads/args-{}-{}", name, id);

	args.open(5.megabytes()).into_file(Path::new(&filepath)).await?;

	Ok(())
}

#[post("/<task_name>/<args_name>")]
pub async fn run_task(task_name: String, args_name:String) -> Result<(), io::Error> {
	let config = Config::new(task_name, args_name);

    offload::run(config).await?;

	Ok(())
}
