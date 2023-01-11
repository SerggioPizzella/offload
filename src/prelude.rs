pub struct Config {
    pub app: String,
    pub starting_args: String,
}

pub struct Worker {
    pub url: &'static str,
}

pub struct Task {
    pub task_name: String,
    pub task_content: String,
}

#[allow(unused)]
struct SubTask<'a> {
    pub task_name: &'a str,
    pub start_param: String,
    pub end_param: String,
}

impl Config {
	pub fn new(task: String, args: String) -> Config {
		Config { app: task, starting_args: args }
	}

    pub fn new_from_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("insuficient argumnets");
        }

        let task = args[1].to_owned();
        let starting_args = args[2].to_owned();

        Ok(Config { app: task, starting_args })
    }
}

impl Worker {
    pub fn new(url: &'static str) -> Worker {
        Worker { url }
    }
}

impl Task {
    pub fn new(task_name: String, task_content: String) -> Task {
        Task {
            task_name,
            task_content,
        }
    }
}