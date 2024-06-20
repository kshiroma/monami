use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::rc::Rc;

//use crate::io::read_line;
use crate::server::config::RelayConnectionInfo;
use crate::http::http_request::HttpRequestInfo;
use crate::http::http_response::HttpResponseInfo;

pub struct Downstream {
    relay: Rc<RelayConnectionInfo>,
    request: Rc<HttpRequestInfo>,
    stream: TcpStream,
    pub buf_reader: BufReader<TcpStream>,
}

impl Downstream {
    pub fn new(relay: Rc<RelayConnectionInfo>, request: Rc<HttpRequestInfo>) -> Option<Downstream> {
        let result: std::io::Result<TcpStream> = relay.connect_relay();
        if result.is_err() {
            return None;
        }
        let stream = result.unwrap();
        let s = Box::new(stream);
        let read = s.try_clone().unwrap();
        let a = s.try_clone().unwrap();
        let buf_reader = BufReader::new(read);
        let downstream = Downstream {
            relay,
            request,
            stream: a,
            buf_reader,
        };
        return Some(downstream);
    }

    pub fn send_first_line(&self) {
        let mut stream = &self.stream;
        stream.write(self.request.http_first_line.method.as_bytes()).unwrap();
        stream.write(b" ").unwrap();

        stream.write(self.relay.path.as_bytes()).unwrap();
        stream.write(b" ").unwrap();
        stream.write(self.request.http_first_line.protool_version.as_bytes()).unwrap();
        stream.write(b"\r\n").unwrap();

        log::trace!("{}", self.request.http_first_line.method);
        log::trace!("{}", self.request.http_first_line.uri);
        log::trace!("{}", self.request.http_first_line.protool_version);
    }

    pub fn send_headers(&self) {
        let mut stream = &self.stream;
        let a = self.request.clone();
        let request = &a;
        let mut string = String::new();
        //Host
        let relay = self.relay.clone();
        if relay.host.is_empty() == false {
            string.push_str("Host: ");
            string.push_str(request.http_request_header.host.as_str());
            string.push_str("\r\n");
            log::debug!("host:{}",request.http_request_header.host);
            log::trace!("end send host.")
        }
        //Connection
        if request.http_request_header.keep_alive {}
        if request.http_request_header.content_length > 0 {
            string.push_str("Content-Length: ");
            string.push_str(request.http_request_header.content_length.to_string().as_str());
            string.push_str("\r\n");
        }
        //ヘッダー
        let headers = &a.http_request_header.headers;
        for header in headers {
            let name = &header.name;
            let value = &header.value;
            string.push_str(name);
            string.push_str(": ");
            string.push_str(value);
            string.push_str("\r\n");
        }
        //let relayInfo = &self.relay.relayInfo;
        let a = relay.clone();
        let a = &a.relayInfo;
        a.into_iter().for_each(|a| {
            string.push_str("X-MONAMI-RELAY-INFO");
            string.push_str(": ");
            string.push_str(a);
            string.push_str("\r\n");
        }
        );
        string.push_str("\r\n");
        stream.write(string.as_bytes()).unwrap();
        log::trace!("end send header.")
    }

    pub fn send_body(&mut self, reader: &mut dyn BufRead) {
        let mut unsend_data_length = self.request.http_request_header.content_length;
        let mut buf = [0; 4096 * 4];
        while unsend_data_length > 0 {
            let size = reader.read(&mut buf).unwrap();
            let d = size.to_string();
            let data_length: i64 = d.parse().unwrap();
            self.send(&buf[0..size]);
            log::trace!("request {} bytes",d);
            unsend_data_length = unsend_data_length - data_length;
        }
    }


    pub fn send(&mut self, buf: &[u8]) {
        //let mut stream = &self.stream;
        self.stream.write(buf).unwrap();
        //stream.write(buf).unwrap();
    }
    pub fn flush(&self) {
        let mut stream = &self.stream;
        stream.flush().unwrap();
    }


    pub fn read_http_response_info(&mut self) -> std::io::Result<HttpResponseInfo> {
        //let mut read = &self.stream;
        return crate::http::http_response::read_http_response_info(&mut self.buf_reader);
    }
}