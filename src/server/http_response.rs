use std::io::{BufRead, Read, Write};

use crate::http::http_header::{HttpHeaderEntry, parse};
use crate::server::http_request::HttpRequestInfo;
use crate::io::read_line;
use std::rc::Rc;

pub trait Response {
    fn response(self:Box<Self>, request: HttpRequestInfo, reader: &mut dyn BufRead, writer: &mut dyn Write) -> std::io::Result<()>;
}

pub struct HttpResponseInfo {
    pub http_first_line: HttpResponseFirstLine,
    pub http_response_header: HttpResponseHeader,
}

impl HttpResponseInfo {
    pub fn new(http_first_line: HttpResponseFirstLine, http_response_header: HttpResponseHeader) -> Self {
        HttpResponseInfo {
            http_first_line,
            http_response_header,
        }
    }
}


pub struct HttpResponseFirstLine {
    pub protocol_version: String,
    pub http_status_code: i32,
    pub http_status: String,
    pub resonse: String,
}

impl HttpResponseFirstLine {
    pub fn new(first_line: String) -> Self {
        let mut array = first_line.split_whitespace();

        HttpResponseFirstLine {
            protocol_version: String::from(array.next().unwrap_or_default()),
            http_status_code: String::from(array.next().unwrap()).parse().unwrap(),
            http_status: String::from(array.next().unwrap_or_default()),
            resonse: first_line,
        }
    }
}

pub struct HttpResponseHeader {
    pub content_length: i64,
    pub headers: Vec<HttpHeaderEntry>,
    pub keep_alive: bool,
}

impl HttpResponseHeader {
    pub fn empty() -> std::io::Result<Self> {
        let headers0: Vec<HttpHeaderEntry> = Vec::new();
        return Ok(HttpResponseHeader {
            content_length: -1,
            headers: headers0,
            keep_alive: false,
        });
    }

    pub fn new(header_lines: Vec<String>) -> std::io::Result<Self> {
        let mut e = HttpResponseHeader::empty()?;
        for line in header_lines {
            e.add_string(line)?;
        }
        return Ok(e);
    }

    pub fn add_string(&mut self, header_line: String) -> std::io::Result<()> {
        if header_line.is_empty() {
            return Ok(());
        }
        let header = parse(header_line).expect("Bad Request");
        if header.name.eq_ignore_ascii_case("Content-Length") {
            self.content_length = header.value.parse().unwrap_or(-1);
        } else if header.name.eq_ignore_ascii_case("Connection") {
            if header.value.eq_ignore_ascii_case("Content-Length") {}
        } else {
            self.headers.push(header);
        }
        return Ok(());
    }
}

pub fn read_header(reader: &mut dyn Read) -> std::io::Result<HttpResponseHeader> {
    let mut headers: HttpResponseHeader = HttpResponseHeader::empty()?;
    loop {
        let line = read_line(reader);
        if line.is_empty() {
            break;
        }
        headers.add_string(line)?;
    }
    return Ok(headers);
}

pub fn read_http_response_info(read: &mut dyn Read) -> std::io::Result<HttpResponseInfo> {
    let first_string = read_line(read);
    let str = first_string.clone();
    let first_line = HttpResponseFirstLine::new(first_string);
    println!("begin read response header of {}", str);
    let headers = read_header(read).unwrap();

    return Ok(HttpResponseInfo::new(first_line, headers));
}
//#[test]
//pub fn test_read_http_reponse() {
//    let path = "test/httpresponse/response_a.txt";
//    //let _string = std::fs::read_to_string(path).unwrap();
//    let mut reader = std::fs::File::open(path).unwrap();
//    let response = read_http_response(&mut reader).unwrap();
//    assert_eq!("OK", response.http_first_line.http_status);
//    assert_eq!(200, response.http_first_line.http_status_code);
//    assert_eq!("HTTP/1.1", response.http_first_line.protocol_version);
//
//    assert_eq!(5055, response.http_response_header.content_length)
//}