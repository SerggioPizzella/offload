use crate::prelude::*;
use futures::StreamExt;
use reqwest::Client;
use std::{fs, io};

pub mod prelude;

async fn is_worker_up(url: &str) -> bool {
    let client = Client::new();
    let resp = client.get(url).send().await;

    match resp {
        Ok(r) => r.status().is_success(),
        Err(_) => false,
    }
}

async fn detect_workers(workers: Vec<Worker>) -> Vec<Worker> {
    let mut good_workers: Vec<Worker> = Vec::new();

    for worker in workers {
        if is_worker_up(worker.url).await {
            good_workers.push(worker)
        }
    }

    good_workers
}

async fn send_task(client: &Client, worker: &Worker, task: &Task) -> Result<(), reqwest::Error> {
    let request_url = format!("{}/task/{}", worker.url, task.task_name);

    let response = client
        .post(request_url)
        .body(task.task_content.clone())
        .send()
        .await;

    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

async fn run_task<'a>(
    client: &'a Client,
    worker: &'a Worker,
    task: &'a Task,
    param: String,
) -> (&'a Worker, Result<String, reqwest::Error>) {
    let request_url = format!("{}/task/{}/run", worker.url, task.task_name);

    let response = client
        .post(request_url)
        .body(format!("{}", param))
        .send()
        .await;

    (worker, response.unwrap().text().await)
}

fn load_task(config: &Config) -> Task {
    let task_name = &config.app;

    let task_content = match fs::read_to_string(&task_name) {
        Ok(content) => content,
        Err(_) => panic!(),
    };

    Task::new(task_name.to_owned(), task_content)
}

fn load_params(config: &Config) -> Result<Vec<String>, io::Error> {
    let task_args = &config.starting_args;
    let task_content = fs::read_to_string(&task_args)?;

    let result: Vec<String> = task_content.lines().map(|line| line.to_owned()).collect();

    Ok(result)
}

async fn distribute_params(task: &Task, params: &mut Vec<String>, workers: & Vec<Worker>) {
    let client = Client::new();
    let mut futures = futures::stream::FuturesUnordered::new();
    for worker in workers {
        let param = match params.pop() {
            Some(p) => p,
            None => return
        };

        futures.push(run_task(&client, &worker, task, param.clone()));
    }

    while let Some((worker, result)) = futures.next().await {
        println!("result: {}", result.unwrap());
        let param = match params.pop() {
            Some(p) => p,
            None => return
        };
        futures.push(run_task(&client, worker, task, param));
    }
}

pub async fn run(config: Config) -> Result<(), io::Error> {
    let task = load_task(&config);
    let mut params = load_params(&config)?;
    
    let workers = vec![Worker::new("http://172.27.111.105:8000")];
    let workers = detect_workers(workers).await;

    let client = Client::new();

    for worker in &workers {
        send_task(&client, &worker, &task).await.unwrap();
    }

    distribute_params(&task, &mut params, &workers).await;

    Ok(())
}
