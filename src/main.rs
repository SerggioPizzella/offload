mod id_seed;
mod end_points;
mod prelude;

#[macro_use] extern crate rocket;

use id_seed::IdSeed;
use end_points::*;

#[launch]
fn launch() -> _ {
    let ip = local_ip_address::local_ip().unwrap().to_string();

    let config = rocket::Config::figment()
        .merge(("ip", ip))
        .merge(("port", 8000));

    rocket::custom(config)
		.manage(IdSeed::new(0))
        .mount("/upload", routes![upload_task, upload_args])
        .mount("/run", routes![run_task])
}
