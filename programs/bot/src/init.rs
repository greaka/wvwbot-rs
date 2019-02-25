extern crate ratelimit;
extern crate serde_json;

use crate::userupdate::worker::*;
use std::time::Duration;
use std::{fs, thread};

pub struct State {
    pub config: Config,
    workers: Vec<Worker>,
    loggers: Vec<discord_logger::DiscordLogger>,
}

#[derive(Serialize, Deserialize, Clone)]
struct RatelimitConfig {
    capacity: u32,
    quantum: u32,
    interval: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct LogConfig {
    webhook_id: u64,
    webhook_token: String,
    log_levels: Vec<log::Level>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    worker_count: u32,
    ratelimit: RatelimitConfig,
    logs: Vec<LogConfig>,
}

pub fn load_config(location: &str) -> Result<Config, std::io::Error> {
    let config_string = fs::read_to_string(location)?;

    let config: Config = serde_json::from_str(&config_string)?;

    Ok(config)
}

pub fn init(config: Config) -> Result<State, &'static str> {
    let mut ratelimit = init_ratelimit(config.ratelimit.clone());

    let workers = init_workers(&config, &mut ratelimit);

    let loggers = init_logger(&config.logs)?;

    Ok(State {
        config,
        workers,
        loggers,
    })
}

fn init_logger(
    config: &Vec<LogConfig>,
) -> Result<Vec<discord_logger::DiscordLogger>, &'static str> {
    let loggers = config
        .iter()
        .map(|conf| {
            discord_logger::DiscordLogger::new(
                conf.webhook_id,
                &conf.webhook_token,
                conf.log_levels.clone(),
            )
        })
        .collect();
    Ok(loggers)
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

fn init_workers(config: &Config, handle: &mut ratelimit::Handle) -> Vec<Worker> {
    let mut workers = vec![];
    for _ in 0..config.worker_count {
        let worker_handle = handle.clone();
        workers.push(Worker::new(worker_handle))
    }
    workers
}
