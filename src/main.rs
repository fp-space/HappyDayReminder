use chrono::*;
use config::load_config;
use tyme4rs::tyme::lunar::{self, LunarDay};
use tyme4rs::tyme::solar::SolarDay;

mod config;

fn main() {
    let config = load_config("config.yml").unwrap();
    
}
