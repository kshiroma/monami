use std::io::prelude::*;
use std::rc::Rc;

use crate::server::config::RelayConnectionInfo;
use crate::server::http_response::HttpResponseInfo;

//use std::borrow::Borrow;

pub struct Downstream {
    response: HttpResponseInfo,
    relay: Rc<RelayConnectionInfo>,
    //writer: Rc<Write>,
}

impl Downstream {
    pub fn new(relay: Rc<RelayConnectionInfo>, response: HttpResponseInfo) -> Self {
        let downstream = Downstream {
            response,
            relay,
        };
        return downstream;
    }

    pub fn send_first_line(&self, writer: &mut dyn Write) {
        let mut string = String::new();
        string.push_str(self.response.http_first_line.protocol_version.as_str());
        string.push_str(" ");
        string.push_str(self.response.http_first_line.http_status_code.to_string().as_str());
        string.push_str(" ");
        string.push_str(self.response.http_first_line.http_status.as_str());
        string.push_str("\r\n");
        writer.write(string.as_bytes()).unwrap();
    }

    pub fn send_headers(&self, writer: &mut dyn Write) {
        let mut string = String::new();
        //if self.response.http_response_header.keep_alive {}
        string.push_str("Connection: close");
        string.push_str("\r\n");
        if self.response.http_response_header.content_length > 0 {
            string.push_str("Content-Length: ");
            string.push_str(self.response.http_response_header.content_length.to_string().as_str());
            string.push_str("\r\n");
        }
        let response = &self.response;
        for header in &response.http_response_header.headers {
            let name = &header.name;
            let value = &header.value;
            string.push_str(name);
            string.push_str(": ");
            string.push_str(value);
            string.push_str("\r\n");
        }
        string.push_str("X-MONAMI: monami");
        string.push_str("\r\n");
        //let relayInfo = self.relay.relayInfo;
        let relay = self.relay.clone();
        let a = &relay.relayInfo;
        a.into_iter().for_each(|b| {
            string.push_str("X-MONAMI-RELAY-INFO");
            string.push_str(": ");
            string.push_str(b.as_str());
            string.push_str("\r\n");
        }
        );
        let address = relay.get_address();
        a.into_iter().for_each(|b| {
            string.push_str("X-MONAMI-UPSTREAM");
            string.push_str(": ");
            string.push_str(address.as_str());
            string.push_str("\r\n");
        }
        );


        string.push_str("\r\n");
        writer.write(string.as_bytes()).unwrap();
        log::trace!("end send response header.")
    }

    pub fn send_body(&self, reader: &mut dyn Read, writer: &mut dyn Write) {
        log::trace!("start send_body");
        let data_length = self.response.http_response_header.content_length;
        log::trace!("let data_length = self.response.http_response_header.content_length;");
        let mut buf = [0; 4096 * 4];
        log::trace!(stringify!(let mut buf = [0; 4096*4];));
        if data_length > 0 {
            log::trace!("enter data_length>0");
            let mut unsent_data_length = self.response.http_response_header.content_length;
            log::trace!("unsent_data_length is {}",unsent_data_length);
            while unsent_data_length > 0 {
                let size = reader.read(&mut buf).unwrap();
                let d = size.to_string();
                let read_length: i64 = d.parse().unwrap();
                writer.write(&buf[0..size]).unwrap();
                log::trace!("response {} data",String::from_utf8_lossy(&buf[0..31]));
                unsent_data_length = unsent_data_length - read_length;
                log::trace!("unsent_data_length is {}",unsent_data_length);
            }
        } else if data_length == 0 {
            //何もしない
            log::trace!("response nothing");
        } else {
            let mut sent_data_length = 0;
            log::trace!("enter data_length = 0");
            //let mut zero_reesponse_count = 0;
            loop {
                log::trace!("reader.read(&mut buf).unwrap()");
                let size = reader.read(&mut buf).unwrap();
                if size == 0 {
                    //zero_reesponse_count += 1;
                    break;
                }
                let d = size.to_string();
                let data_length: i64 = d.parse().unwrap();
                writer.write(&buf[0..size]).unwrap();
                log::trace!("response data_length = 0 :{} {} ",d,&buf[size-1]);
                sent_data_length = sent_data_length + data_length;
                writer.flush().unwrap();
            }
        }
    }
}

