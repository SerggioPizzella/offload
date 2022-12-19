pub struct Config {
    pub app: String,
    pub starting_args: String,
}

pub(crate) struct Worker {
    pub(crate) url: &'static str,
}

pub(crate) struct Task {
    pub(crate) task_name: String,
    pub(crate) task_content: String,
}

#[allow(unused)]
struct SubTask<'a> {
    pub(crate) task_name: &'a str,
    pub(crate) start_param: String,
    pub(crate) end_param: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("insuficient argumnets");
        }

        let app = args[1].to_owned();
        let starting_args = args[2].to_owned();

        Ok(Config { app, starting_args })
    }
}

impl Worker {
    pub(crate) fn new(url: &'static str) -> Worker {
        Worker { url }
    }
}

impl Task {
    pub(crate) fn new(task_name: String, task_content: String) -> Task {
        Task {
            task_name,
            task_content,
        }
    }
}

#[allow(unused)]
impl<'a> SubTask<'a> {
    pub(crate) fn new(task: &'a Task, start_param: String, end_param: String) -> SubTask<'a> {
        SubTask::<'a> {
            task_name: &task.task_name,
            start_param,
            end_param,
        }
    }
}
