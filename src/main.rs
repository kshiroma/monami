extern crate rand;
extern crate regex;

use std::env;

use env_logger;

use crate::routing_config::create_sample_config;
use crate::server::config::ServerConfig;

pub mod http;
pub mod server;
pub mod io;
pub mod routing_config;


fn main() {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    let config = create_sample_config();
    server::listen(config, 6731).unwrap();
}
