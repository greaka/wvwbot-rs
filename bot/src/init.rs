extern crate ratelimit;
extern crate serde_json;

use crate::userupdate::worker::*;
use std::time::Duration;
use std::{fs, thread};

pub struct State {
    pub config: Config,
    workers: Vec<Worker>,
}

#[derive(Serialize, Deserialize, Clone)]
struct RatelimitConfig {
    capacity: u32,
    quantum: u32,
    interval: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    worker_count: u32,
    ratelimit: RatelimitConfig,
}

pub fn load_config(location: &str) -> Result<Config, std::io::Error> {
    let config_string = fs::read_to_string(location)?;

    let config: Config = serde_json::from_str(&config_string)?;

    Ok(config)
}

pub fn init(config: Config) -> Result<State, &'static str> {
    let mut ratelimit = init_ratelimit(config.ratelimit.clone());

    let workers = init_workers(&config, &mut ratelimit).expect("failed to init workers");

    Ok(State { config, workers })
}

fn init_ratelimit(config: RatelimitConfig) -> ratelimit::Handle {
    let mut ratelimit = ratelimit::Builder::new()
        .capacity(config.capacity)
        .quantum(config.quantum)
        .interval(Duration::from_millis(config.interval))
        .build();

    let handle = ratelimit.make_handle();
    thread::spawn(move || ratelimit.run());

    handle
}

fn init_workers(
    config: &Config,
    handle: &mut ratelimit::Handle,
) -> Result<Vec<Worker>, &'static str> {
    let mut workers = vec![];
    for _ in 0..config.worker_count {
        let worker_handle = handle.clone();
        workers.push(Worker::new(worker_handle))
    }
    Ok(workers)
}
