pub struct Config<'a> {
  pub app: &'a [u8],
  pub starting_args: String
}

impl<'a> Config<'a> {
  pub fn new(args: &'a [String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("insuficient argumnets");
    }

    let app = args[1].as_bytes();
    let starting_args = args[2].to_owned();

    Ok(Config{ app, starting_args })
  }
}

pub fn detect_workers() {
  let workers = vec!["localhost", "worker1", "worker2", "worker3"];

  let resp = reqwest::blocking::get(format!("http://{}/alive", workers[0]));
}