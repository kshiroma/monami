use crate::server::config::ServerConfig;
pub mod config;
pub mod http_request;
pub mod http_response;
mod worker;
mod upstream;
mod downstream;

pub fn listen(config: ServerConfig, port: i32) -> std::io::Result<()> {
    let rc = std::sync::Arc::new(config);
    let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))?;
    //listener.set_nonblocking(true);
    for stream in listener.incoming() {
        let rc0 = rc.clone();
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("An error occurred while accepting a connection:{}", e);
                continue;
            }
        };
        std::thread::spawn(move || -> std::io::Result<()> {
            log::debug!("worker start");
            let worker = worker::Worker::new(rc0);
            let result = worker.handle(stream);
            log::debug!("worker end.");
            return result;
        });
    }
    Ok(())
}

#[test]
fn test() {
    use crate::routing_sample::create_sample_config;
    let config = create_sample_config();
    listen(config, 80).unwrap();
}

