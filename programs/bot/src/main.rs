#[macro_use]
extern crate serde_derive;

mod init;
mod userupdate;

fn main() {
    let config = init::load_config(&String::from("config.json")).expect("error loading config");
    let state = init::init(config).expect("error initiating state");
}
