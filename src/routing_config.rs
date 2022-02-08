use crate::server::config::{RelayConnectionInfo, RoutingRule, ServerConfig};
use crate::server::http_request::HttpRequestInfo;

pub fn create_sample_config() -> ServerConfig {
    let mut config = ServerConfig::new();
    config.add(RoutingRule::new("set_routing_number".to_string(), set_routing_number));
    config.add(RoutingRule::new("routing".to_string(), premium));
    config.add(RoutingRule::new("freesia".to_string(), freesia));
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

const JT0100: &str = "dev-jt0100";
const JT0001: &str = "dev-jt0001";

fn freesia(config: &ServerConfig, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let path: &str = if request.http_first_line.uri.eq_ignore_ascii_case("/favicon.ico") {
        "/cattleya/favicon.ico"
    } else {
        &request.http_first_line.uri
    };
    const relayInfo: &str = "freesia";
    let relay = if true {
        let n = config.get_routing_number();
        match n {
            0 => jt0100(path,relayInfo),
            1 => jt0001(path,relayInfo),
            2 => jt0100(path,relayInfo),
            _ => None
        }
    } else {
        None
    };
    return relay;
}

fn premium(config: &ServerConfig, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let path: &str = if request.http_first_line.uri.eq_ignore_ascii_case("/favicon.ico") {
        "/cattleya/favicon.ico"
    } else {
        &request.http_first_line.uri
    };

    let relay = if true {
        let i = config.get_count();
        let n = config.get_routing_number();
        match n {
            1 =>
                Some(RelayConnectionInfo::new2(
                    jt0001,
                    8000,
                    path,
                    "1__",
                )),
            _ =>
                if i % 2 == 0 {
                    Some(RelayConnectionInfo::new2(
                        jt0001,
                        8000,
                        path,
                        "0_0",
                    ))
                } else {
                    Some(RelayConnectionInfo::new2(
                        jt0100,
                        8000,
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



fn jt0001(path:&str , relayInfo: &str) -> Opton<RelayConnectionInfo> {
    Some(RelayConnectionInfo::new2(
        JT0001,
        8000,
        path,
        relayInfo,
    ))
}

fn jt0100(path:&str , relayInfo: &str) -> Option<RelayConnectionInfo> {
    Some(RelayConnectionInfo::new2(
        JT1000,
        8000,
        path,
        relayInfo,
    ))
}


fn routing_milliondollar(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let prefix: &str = "million-dollar";

    let host = &request.http_request_header.host;
    let pattern = prefix.to_string() + ".";
    let pattern = pattern.as_str();
    let conjunction: &str = &request.http_first_line.uri;
    let relay = if host.starts_with(pattern) {
        Some(RelayConnectionInfo::new2(
            "localhost",
            1234,
            conjunction,
            "million_dollar",
        ))
    } else {
        None
    };
    return relay;
}


fn routing_timer(request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
    let prefix: &str = "timer";
    let path = "/cattleya";
    let host = &request.http_request_header.host;
    let pattern = prefix.to_string() + ".";
    let pattern = pattern.as_str();
    let conjunction: &str = if request.http_first_line.uri.contains('?') { "&" } else { "?" };
    let relay = if host.starts_with(pattern) {
        Some(RelayConnectionInfo::new2(
            "dev-timer",
            8000,
            (path.to_string() + conjunction + "targetUser=" + prefix).as_str(),
            "chronotrigger",
        ))
    } else {
        None
    };
    return relay;
}


#[test]
fn test() {
    use std::io::Read;
    use std::io::Write;
    let relay =
        RelayConnectionInfo::new1(
            "localhost",
            8080,
            "/cattleya/view/login?targetUser=wakuden");
    println!("relay host is {}", relay.get_address());

    let mut stream = &relay.connect_relay().unwrap();
    stream.write(b"GET ").unwrap();
    stream.write(&relay.path.into_bytes()).unwrap();
    stream.write(b" HTTP/1.1\r\n").unwrap();

    stream.write(b"Host: localhost:8000\r\n").unwrap();
    stream.write(b"User-Agent: curl/7.55.1\r\n").unwrap();
    stream.write(b"Accept: */*\r\n\r\n").unwrap();
    stream.flush().unwrap();
    let mut data = [0; 4096 * 4];
    stream.read(&mut data).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}


#[test]
fn test_get_address() {
    let relay = RelayConnectionInfo::new1(
        "localhost",
        8080,
        "/cattleya/view/login?targetUser=wakuden");
    assert_eq!("localhost:8080", relay.get_address());
    let relay = RelayConnectionInfo::new1(
        "localhost",
        80,
        "/cattleya/view/login?targetUser=wakuden");
    assert_eq!("localhost", relay.get_address());
    let relay = RelayConnectionInfo::new1(
        "localhost",
        0,
        "/cattleya/view/login?targetUser=wakuden",
    );
    assert_eq!("localhost", relay.get_address());
}


#[test]
fn test_get_user_name() {}