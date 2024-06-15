use std::io::BufReader;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::rc::Rc;
use std::sync::Arc;

use crate::http::http_status::{bad_request, not_found, service_unavailable, set_routing_number};
use crate::server::config::{RelayConnectionInfo, ServerConfig};
use crate::server::aaaaaaa::Aaaaaaaa;
use crate::server::http_request::read_http_request;
use crate::server::http_response::Response;
use crate::server::upstream::Upstream;

pub struct Worker {
    config: Arc<ServerConfig>,
}

impl Worker {
    pub fn new(config: Arc<ServerConfig>) -> Self {
        Worker {
            config,
        }
    }

    pub fn handle(&self, stream: TcpStream) -> std::io::Result<()> {
        let mut stream_box = Box::new(stream);
        let read = stream_box.try_clone().unwrap();
        let mut write = stream_box.try_clone().unwrap();
        let mut buf_reader = BufReader::new(read);
        let result = self.handle_read_writer(&mut buf_reader, &mut write);
        //終わり
        stream_box.flush().unwrap();
        stream_box.shutdown(Shutdown::Both).unwrap();
        //reader.shutdown(Shutdown::Both);
        log::trace!("shutdown stream");
        return Ok(());
    }

    fn handle_read_writer(&self, reader: &mut dyn BufRead, writer: &mut dyn Write) -> std::io::Result<()> {
        let request = read_http_request(reader);
        if request.is_err() {
            bad_request(writer).unwrap();
            return Ok(());
        }

        let request = request.unwrap();
        let relay: Option<RelayConnectionInfo> = self.config.route(&request);
        if relay.is_none() {
            log::info!("not found relay connection {}", request.http_first_line.uri);
            not_found(writer).unwrap();
            return Ok(());
        }
        let relay = relay.unwrap();
        let response = relay.response;
        {
            //let relayInfo = &relay.relayInfo;

            if response {
                //if relayInfo.eq_ignore_ascii_case("set_routing_number") {
                return set_routing_number(writer, self.config.get_routing_number());
                //}
            }
        }
        log::info!("relay connection host is {}:{}", relay.host, relay.port);
        //
        let b_relay = std::rc::Rc::new(relay).clone();
        let b_relay2 = b_relay.clone();
        let b_request = std::rc::Rc::new(request).clone();
        let upstream_op = Upstream::new(b_relay, b_request);
        if upstream_op.is_none() {
            log::info!("can not connect upstream {}", b_relay2.host);
            service_unavailable(writer).unwrap();
            return Ok(());
        }

        self.config.add_count();
        let mut upstream = upstream_op.unwrap();

        upstream.send_first_line();
        log::trace!("upstream.sendFirstLine()");
        upstream.send_headers();
        log::trace!("upstream.sendHeader()");
        upstream.send_body(reader);
        log::trace!("upstream.sendBody(reader);");
        upstream.flush();
        log::trace!("upstream.flush();");
        let response_info = upstream.read_http_response_info().unwrap();
        log::trace!("let response_info = upstream.read_http_response_info().unwrap();");

        let aaaaaaa = Aaaaaaaa::new(b_relay2, response_info);
        log::trace!("let aaaaaaa = Downstream::new(response_info);");
        aaaaaaa.send_first_line(writer);
        log::trace!("aaaaaaa.sendFirstLine(writer);");
        aaaaaaa.send_headers(writer);
        log::trace!("aaaaaaa.sendHeaders(writer);");
        aaaaaaa.send_body(&mut upstream.buf_reader, writer);
        log::trace!("aaaaaaa.sendBody(&mut upstream.stream, writer);");
        writer.flush().unwrap();
        log::trace!("writer.flush();");
        return Ok(());
    }
}


