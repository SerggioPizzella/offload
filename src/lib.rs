use std::fs;

use reqwest::StatusCode;

pub struct Config {
  pub app: String,
  pub starting_args: String
}

struct Worker  {
  url: &'static str
}

struct Task {
  task_name: String,
  task_content: String
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("insuficient argumnets");
    }

    let app = args[1].to_owned();
    let starting_args = args[2].to_owned();

    Ok(Config{ app, starting_args })
  }
}

impl Worker {
  fn new(url: &'static str) -> Worker {
    Worker{ url }
  }
}

impl Task {
  fn new(task_name: String, task_content: String) -> Task {
    Task { task_name, task_content }
  }
}

fn detect_workers(workers: Vec<Worker>) -> Vec<Worker>{
  let mut good_workers : Vec<Worker> = Vec::new();

  for worker in workers {
    let resp = reqwest::blocking::get(format!("{}/alive", worker.url));
  
    match resp {
      Ok(resp) if resp.status() == StatusCode::OK => {
        println!("Found worker: {}", worker.url);
        good_workers.push(worker.into());
      },
      Ok(resp) => println!("Found worker: {}, but it responded with: {}", worker.url, resp.status()),
      Err(err) => eprintln!("Error reaching worker: {}: {:#}", worker.url, err)
    }
  }

  good_workers
}
 
fn send_task(worker: &Worker, task: &Task) {
  let request_url = format!("{}/task/{}", worker.url, task.task_name);

  let client = reqwest::blocking::Client::new();
  let response = client.post(request_url)
    .body(task.task_content.clone())
    .send();

  match response {
    Ok(_)  => println!("task: {}, sent to: {}", task.task_name, worker.url),
    Err(err) => println!("failed to send task: {}, to worker: {}. : {}", task.task_name, worker.url, err)
  }
}

fn load_task(config: Config) -> Task {
  let task_name = config.app;
  
  if let Ok(task_content) = fs::read_to_string(&task_name) {
    Task::new(task_name, task_content)
  } else {
    panic!()
  }
}
pub fn run(config: Config) {
  let workers = vec![
    Worker::new("http://localhost:8000"),
    Worker::new("http://worker1:8000"),
    Worker::new("http://worker2:8000"),
    Worker::new("http://worker3:8000"),
  ];

  let task = load_task(config);
  let workers = detect_workers(workers);

  for worker in workers {
    send_task(&worker, &task);
  }
}