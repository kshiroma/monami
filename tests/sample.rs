extern crate monami;

use std::env;

use env_logger;
use log::logger;

use monami::server;
use monami::server::config::{RelayConnectionInfo, RoutingRule, ServerConfig};
use monami::http::http_request::HttpRequestInfo;

#[test]
fn main_sample() {
    // テスト実行
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let config = create_sample_config();
    server::listen(config, 6731).unwrap();
}


fn create_sample_config() -> ServerConfig {
    let mut config = ServerConfig::new();
    config.add(RoutingRule::new("set_routing_number".to_string(), set_routing_number));
    config.add(RoutingRule::new("routing".to_string(), routing));
    return config;
}

fn set_routing_number(config: &ServerConfig, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let path = "/set_routing_number/";
    if request.http_first_line.uri.starts_with(path) {
        let number = request.http_first_line.uri.replace(path, "");
        let number = number.parse().unwrap();
        config.set_routing_number(number);
        return Some(RelayConnectionInfo::new3("monami-self-response", 0, "", "set_routing_number", true));
    }
    return None;
}

fn routing(config: &ServerConfig, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    const HOST_A: &str = "localhost";
    const PORT_A: i32 = 8000;

    const HOST_B: &str = "localhost";
    const PORT_B: i32 = 8000;


    let path: &str = &request.http_first_line.uri;

    let relay = if true {
        let i = config.get_count();
        let n = config.get_routing_number();
        match n {
            1 =>
                Some(RelayConnectionInfo::new2(
                    HOST_A,
                    PORT_A,
                    path,
                    "1__",
                )),
            2 =>
                Some(RelayConnectionInfo::new2(
                    HOST_B,
                    PORT_B,
                    path,
                    "2__",
                )),
            _ =>
                if i % 2 == 0 {
                    Some(RelayConnectionInfo::new2(
                        HOST_A,
                        PORT_A,
                        path,
                        "0_0",
                    ))
                } else {
                    Some(RelayConnectionInfo::new2(
                        HOST_B,
                        PORT_B,
                        path,
                        "0_1")
                    )
                },
        }
    } else {
        None
    };
    return relay;
}
