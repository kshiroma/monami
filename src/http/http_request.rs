use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;

use crate::http::http_header::{HttpHeaderEntry, parse};
use crate::io::*;

pub struct HttpRequestInfo {
    pub http_first_line: HttpRequestFirstLine,
    pub http_request_header: HttpRequestHeader,
}

impl HttpRequestInfo {
    fn new(first_line_string: HttpRequestFirstLine, header_lines: HttpRequestHeader) -> Self {
        HttpRequestInfo {
            http_first_line: first_line_string,
            http_request_header: header_lines,
        }
    }
}

pub struct HttpRequestFirstLine {
    pub method: String,
    pub uri: String,
    pub protool_version: String,
    pub request: String,
}


impl HttpRequestFirstLine {
    pub fn new(first_line: String) -> Option<Self> {
        let mut array = first_line.split_whitespace();
        let method = array.next();
        let uri = array.next();
        let protoco_version = array.next();
        if method.is_none() || uri.is_none() || protoco_version.is_none() {
            log::error!("first line {}",first_line);
            return None;
        }
        return Some(HttpRequestFirstLine {
            method: String::from(method.unwrap()),
            uri: String::from(uri.unwrap()),
            protool_version: String::from(protoco_version.unwrap()),
            request: first_line,
        });
    }
}


pub struct HttpRequestHeader {
    pub host: String,
    pub content_length: i64,
    pub keep_alive: bool,
    pub headers: Vec<HttpHeaderEntry>,
}


impl HttpRequestHeader {
    pub fn empty() -> std::io::Result<Self> {
        let headers0: Vec<HttpHeaderEntry> = Vec::new();
        return Ok(HttpRequestHeader {
            host: "".to_string(),
            content_length: -1,
            keep_alive: false,
            headers: headers0,
        });
    }

    pub fn new(header_lines: Vec<String>) -> std::io::Result<Self> {
        let mut e = HttpRequestHeader::empty()?;
        for line in header_lines {
            e.add_string(line)?;
        }
        return Ok(e);
    }

    pub fn add_string(&mut self, header_line: String) -> std::io::Result<()> {
        if header_line.is_empty() {
            return Ok(());
        }
        let header = parse(header_line).expect("Bad_Request");
        if header.name.eq_ignore_ascii_case("Host") {
            if self.host.is_empty() {
                self.host = header.value;
            }
        } else if header.name.eq_ignore_ascii_case("X-Forwarded-Host") {
            self.host = header.value;
        } else if header.name.eq_ignore_ascii_case("Content-Length") {
            self.content_length = header.value.parse().unwrap_or(-1);
        } else if header.name.eq_ignore_ascii_case("Connection") {
            if header.value.eq_ignore_ascii_case("keep-alive") {
                self.keep_alive = true;
            }
        } else {
            self.headers.push(header);
        }
        return Ok(());
    }
}

pub fn read_http_request(reader: &mut dyn BufRead) -> std::result::Result<HttpRequestInfo,&str> {
    let first_line_string = read_line2(reader);
    let first_line: Option<HttpRequestFirstLine> = HttpRequestFirstLine::new(first_line_string);
    if (first_line.is_none()) {
        return Err("bad");
    }
    let first_line = first_line.unwrap();
    log::trace!("{}", "begin read header");
    let headers = read_header2(reader).unwrap();
    log::trace!("read {} headers", headers.headers.len());
    let info = HttpRequestInfo::new(first_line, headers);

    return Ok(info);
}

pub fn read_header(reader: &mut dyn Read) -> std::io::Result<HttpRequestHeader> {
    let mut headers: HttpRequestHeader = HttpRequestHeader::empty()?;
    loop {
        let line = read_line(reader);
        if line.is_empty() {
            break;
        }
        headers.add_string(line)?;
    }
    return Ok(headers);
}

pub fn read_header2(reader: &mut dyn BufRead) -> std::io::Result<HttpRequestHeader> {
    let mut headers: HttpRequestHeader = HttpRequestHeader::empty()?;
    loop {
        log::trace!("begin read_line  ");
        let line = read_line2(reader);
        log::trace!("read {}",line);
        if line.is_empty() {
            log::trace!("empty");
            break;
        }
        headers.add_string(line)?;
    }
    return Ok(headers);
}


#[test]
fn test_http_request_request_header() {
    let vec = vec![
        "Host: localhost".to_string(),
        "X-Forwarded-Host: locallocalh".to_string(),
        "User-Agent: curl/7.55.1".to_string(),
        "Accept: */*".to_string(),
        "Content-Length: 7".to_string(),
        "Connection: keep-alive".to_string(),
        "Content-Type: application/x-www-form-urlencoded".to_string(),
        "Content-Type: aaa:bbb".to_string(),
    ];
    let header = HttpRequestHeader::new(vec).unwrap();

    println!("HOST : {} ", header.host);
    println!("KeepAlive: {} ", header.keep_alive);
    println!("content_length: {} ", header.content_length);
}

#[test]
fn test_read_first_line() -> std::io::Result<()> {
    use std::fs;
    use std::fs::File;
//use std::io::Read;
    let path = "test/httprequest/requets_post.txt";
    let _string = fs::read_to_string(path).unwrap();

    let mut reader = File::open(path).unwrap();
    let first_line = read_line(&mut reader);
    assert_eq!(first_line, "POST /bbb/ddd HTTP/1.1");

    let headers = read_header(&mut reader);

    return Ok(());
}