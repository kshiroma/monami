use crate::server::config::ServerConfig;
pub mod config;
pub mod http_request;
pub mod http_response;
mod worker;
mod downstream;
mod aaaaaaa;

pub fn listen(config: ServerConfig, port: i32) -> std::io::Result<()> {
    let rc = std::sync::Arc::new(config);
    let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))?;
    //listener.set_nonblocking(true);
    for stream in listener.incoming() {
        let rc0 = rc.clone();
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                log::error!("An error occurred while accepting a connection:{}", e);
                continue;
            }
        };
        std::thread::spawn( || -> std::io::Result<()> {
            log::trace!("worker start");
            let worker = worker::Worker::new(rc0);
            let result = worker.handle(stream);
            log::trace!("worker end.");
            return result;
        });
    }
    Ok(())
}
