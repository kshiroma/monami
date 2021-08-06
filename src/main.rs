extern crate rand;
extern crate regex;

use std::env;

use env_logger;

use crate::routing_sample::routing_config;

//pub mod study;
pub mod http;
pub mod server;
//pub mod study;
pub mod io;
pub mod routing_sample;


fn main() {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    let config = create_sample_config();
    server::listen(config, 80).unwrap();
}
