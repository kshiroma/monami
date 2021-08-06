use std::io::{BufRead, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::ptr::null;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use chrono::Local;

use crate::http::http_status::HttpStatus;
use crate::server::downstream::Downstream;
use crate::server::http_request::HttpRequestInfo;
use crate::server::http_response::Response;
use crate::server::upstream::Upstream;

pub struct RoutingRule {
    name: String,
    routing_rule: fn(&ServerConfig, &HttpRequestInfo) -> Option<RelayConnectionInfo>,
}


pub struct RelayConnectionInfo {
    pub host: String,
    pub port: i32,
    pub path: String,
    pub relayInfo: Option<String>,
    pub response: bool,
}

impl RelayConnectionInfo {
    //
    pub fn new1(host: &str, port: i32, path: &str) -> RelayConnectionInfo {
        return RelayConnectionInfo::new2(host, port, path, "");
    }
    //
    pub fn new2(host: &str, port: i32, path: &str, relayInfo: &str) -> RelayConnectionInfo {
        return RelayConnectionInfo::new3(host, port, path, relayInfo, false);
    }
    //
    pub fn new3(host: &str, port: i32, path: &str, relayInfo: &str, response: bool) -> RelayConnectionInfo {
        return RelayConnectionInfo {
            host: host.to_string(),
            port,
            path: path.to_string(),
            relayInfo: if relayInfo.is_empty() { None } else { Some(relayInfo.to_string()) },
            response,
        };
    }
    //
    pub fn response(&self) -> String {
        return "".to_string();
    }

    pub fn get_address(&self) -> String {
        let mut host = (&self.host).to_string();
        let port = &self.port;
        let port = *(port);
        if port > 1 && port != 80 {
            host.push(':');
            host = host + &port.to_string();
        }
        return host;
    }

    pub fn connect_relay(&self) -> std::io::Result<TcpStream> {
        let host = self.get_address();
        return std::net::TcpStream::connect(host);
    }
}

//impl Response for RelayConnectionInfo {
//    fn response(self:Box<Self>, request: HttpRequestInfo, reader: &mut dyn BufRead, writer: &mut dyn Write) -> std::io::Result<()> {
//        //let relay = self;
//        //log::info!("relay connection host is {}:{}", relay.host, relay.port);
//        //
//        //let b_relay = relay.clone();
//        let b_request = Box::new(request);
//        let mut upstream = Upstream::new(self, b_request).unwrap();
//
//        upstream.send_first_line();
//        log::trace!("upstream.sendFirstLine()");
//        upstream.send_headers();
//        log::trace!("upstream.sendHeader()");
//        upstream.send_body(reader);
//        log::trace!("upstream.sendBody(reader);");
//        upstream.flush();
//        log::trace!("upstream.flush();");
//        let response_info = upstream.read_http_response_info().unwrap();
//        log::trace!("let response_info = upstream.read_http_response_info().unwrap();");
//
//        let downstream = Downstream::new(response_info);
//        log::trace!("let downstream = Downstream::new(response_info);");
//        downstream.send_first_line(writer);
//        log::trace!("downstream.sendFirstLine(writer);");
//        downstream.send_headers(writer);
//        log::trace!("downstream.sendHeaders(writer);");
//        downstream.send_body(&mut upstream.buf_reader, writer);
//        log::trace!("downstream.sendBody(&mut upstream.stream, writer);");
//        writer.flush().unwrap();
//        log::trace!("writer.flush();");
//        return Ok(());
//    }
//}

impl RoutingRule {
    pub fn new(name: String, routing_rule: fn(&ServerConfig, &HttpRequestInfo) -> Option<RelayConnectionInfo>) -> Self {
        RoutingRule {
            name,
            routing_rule,
        }
    }

    pub fn route(&self, config: &ServerConfig, requet: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
        let func: fn(&ServerConfig, &HttpRequestInfo) -> Option<RelayConnectionInfo> = self.routing_rule;
        return func(config, requet);
    }
}

pub struct ServerConfig {
    routing_rules: Vec<RoutingRule>,
    pub count: Mutex<i32>,
    pub routing_number: Mutex<i32>,
}

impl ServerConfig {
    pub fn new() -> Self {
        let vec: Vec<RoutingRule> = Vec::new();
        ServerConfig {
            routing_rules: vec,
            count: Mutex::new(0),
            routing_number: Mutex::new(1),
        }
    }

    pub fn add(&mut self, rule: RoutingRule) {
        self.routing_rules.push(rule);
    }

    pub fn find_routing_rule(&self, request: &HttpRequestInfo) -> Option<&RoutingRule> {
        for rule in self.routing_rules.iter() {
            if let Some(_) = (rule.routing_rule)(&self, request) {
                return Some(rule);
            }
        }
        return None;
    }

    pub fn route(&self, request: &HttpRequestInfo) -> Option<RelayConnectionInfo> {
        for rule in self.routing_rules.iter() {
            log::trace!("checking {}", rule.name);
            if let Some(r) = (rule.routing_rule)(&self, request) {
                return Some(r);
            }
        }
        return None;
    }

//    pub fn route(&self, request: &HttpRequestInfo) -> Option<Box<Response>> {
//        for rule in self.routing_rules.iter() {
//            log::trace!("checking {}", rule.name);
//            if let Some(r) = (rule.routing_rule)(&self, request) {
//                return Some(Box::new(r));
//            }
//        }
//        return None;
//    }

    pub fn add_count(&self) -> i32 {
        let mut m = self.count.lock().unwrap();
        *m = *m + 1;
        return *m;
    }

    pub fn get_count(&self) -> i32 {
        let m = self.count.lock().unwrap();
        return *m;
    }
    pub fn set_routing_number(&self, number: i32) -> i32 {
        let mut m = self.routing_number.lock().unwrap();
        *m = number;
        return *m;
    }
    pub fn get_routing_number(&self) -> i32 {
        let m = self.routing_number.lock().unwrap();
        return *m;
    }
}

enum HttpResponse {
    NotFound
}

struct SetNumber {
    routing_number: i32,
}

impl Response for SetNumber {
    fn response(self:Box<Self>, request: HttpRequestInfo, reader: &mut dyn BufRead, writer: &mut dyn Write) -> std::io::Result<()> {
        let status = HttpStatus::Ok;
        let code = status.get().unwrap();
        let string = status.get_as_string().unwrap();
        write!(writer, "HTTP/1.1 {} {}\r\n", code, string)?;
        write!(writer, "Date: {} \r\n", Local::now())?;
        let buf1 = b"<html><body><h1>Set Number</h1>";
        let buf2 = format!("<span>{}</span>", self.routing_number);
        let buf2 = buf2.as_bytes();
        let buf3 = b"</body></html>";
        let length = buf1.len() + buf2.len() + buf3.len();
        write!(writer, "Content-Length: {}", length)?;
        write!(writer, "\r\n")?;
        write!(writer, "\r\n")?;
        writer.write(buf1)?;
        writer.write(buf2)?;
        writer.write(buf3)?;
        write!(writer, "\r\n")?;
        return Ok(());
    }
}


impl Response for HttpResponse {
    fn response(self:Box<Self>, request: HttpRequestInfo, reader: &mut dyn BufRead, writer: &mut dyn Write) -> std::io::Result<()> {
        let status = HttpStatus::NotFound;
        let code = status.get().unwrap();
        let string = status.get_as_string().unwrap();
        write!(writer, "HTTP/1.1 {} {}\r\n", code, string)?;
        write!(writer, "Date: {} \r\n", Local::now())?;
        let buf = b"<html><body><h1>Not Found</h1></body></html>";
        let length = buf.len();
        write!(writer, "Content-Length: {}", length)?;
        write!(writer, "\r\n")?;
        write!(writer, "\r\n")?;
        writer.write(buf)?;
        write!(writer, "\r\n")?;
        return Ok(());
    }
}