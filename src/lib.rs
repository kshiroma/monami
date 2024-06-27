extern crate rand;
extern crate regex;

use env_logger;
use log::logger;
use std::env;
use crate::server::config::ServerConfig;

pub mod http;
pub mod server;
pub mod io;
pub mod util;
pub mod streams;
pub mod threads;

mod Monami {
    use std::io::Write;
    use std::net::TcpStream;
    use std::time::Duration;
    use crate::ServerConfig;
    use std::sync::mpsc;
    use std::sync::mpsc::{Sender, Receiver};

    pub fn start(port: i32) -> std::io::Result<()> {
        //let rc = std::sync::Arc::new(config);
        let (tx, rx) = mpsc::channel();
        //let rx1 = mpsc::Receiver::clone(&rx);
        let mut work = true;
        let handle = std::thread::spawn(move || -> std::io::Result<()> {
            log::trace!("manager_thread");
            while work {
                std::thread::sleep(Duration::from_millis(500));
                //let result = rx.recv();
                let receive = rx.recv().unwrap();
                log::trace!("receive: {}", receive);
            }
            Ok(())
        });

        let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))?;
        for stream in listener.incoming() {
            //let rc0 = rc.clone();
            let stream = match stream {
                Ok(stream) => stream,
                Err(e) => {
                    log::error!("An error occurred while accepting a connection:{}", e);
                    continue;
                }
            };
            let tx1 = mpsc::Sender::clone(&tx);
            std::thread::spawn(|| -> std::io::Result<()> {
                log::trace!("worker start");
                //let worker = worker::Worker::new(rc0);
                let result = handle_connection(stream, tx1);
                log::trace!("worker end.");
                return Ok(());
            });
        }
        Ok(())
    }


    fn handle_connection(mut stream: TcpStream, sender: Sender<String>) {
        // --snip--
        let mut buffer = [0; 1024];
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else if buffer.starts_with(sleep) {
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };
        sender.send(String::from(status_line)).unwrap();
        let response = "HTTP/1.1 200 OK \r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.write("<html><bod>Hello World</body></html>".as_bytes()).unwrap();
        stream.flush().unwrap();
        // --snip--
    }
}

#[test]
fn test() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    Monami::start(80);
}


mod WebServer {
    use std::{env, thread};
    use std::net::{TcpListener, TcpStream};
    use std::io::prelude::*;
    use std::ops::Deref;
    use std::sync::{Arc, mpsc, Mutex};
    use futures::StreamExt;


    //Managerから


    #[test]
    fn m(addr: &str) {
        //ログ設定
        env::set_var("RUST_LOG", "trace");
        env_logger::init();

        // TcpListenerの動作制御。複数スレッドで共有
        let work = Arc::new(Mutex::new(true));
        let work0 = Arc::clone(&work);
        let work1 = Arc::clone(&work);

        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(4);
        let mut count = 0;

        for stream in listener.incoming().take_while(|r| {
            let w = work0.lock().unwrap();
            *w
        }) {
            count += 1;
            let a = count;//値がコピーされているので、スレッドにmoveしてもよい
            let stream = stream.unwrap();
            thread::spawn(move || { handle_connection(stream, a) });
            //pool.execute(|| handle_connection(stream));
            if (count > 3) {
                let mut w = work1.lock().unwrap();
                *w = false;
            }
        }
    }

    fn start() {}

    fn handle_connection(mut stream: TcpStream, count: i32) {
        let mut buffer = [0; 1024];
        let len: usize = stream.read(&mut buffer).unwrap();
        log::debug!("Request: {}", String::from_utf8_lossy(&buffer[0..len]));
        let response = "HTTP/1.1 200 OK \r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        write!(stream, "<html><bod>Hello World {}</body></html>", count);
        //stream.write("<html><bod>Hello World</body></html>".as_bytes()).unwrap();
        stream.flush().unwrap();
    }


    trait FnBox {
        fn call_box(self: Box<Self>);
    }

    impl<F: FnOnce()> FnBox for F {
        fn call_box(self: Box<F>) {
            (*self)()
        }
    }

    struct Worker {
        id: usize,
        thread: std::thread::JoinHandle<()>,
    }

    type Job = Box<FnBox + Send + 'static>;


    pub struct ThreadPool {
        //threads: Vec<std::thread::JoinHandle<()>>,
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }

    impl ThreadPool {
        pub fn new(size: usize) -> Self {
            assert!(size > 0);
            //let mut threads = Vec::with_capacity(size);
            let mut workers = Vec::with_capacity(size);
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Self {
                workers,
                sender,
            }
        }
        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = std::thread::spawn(move || {
                while let Ok(job) = receiver.lock().unwrap().recv() {
                    log::debug!("worker {} gat a job; executing.",id);
                    job.call_box();
                }
            });

            Worker {
                id,
                thread,
            }
        }
    }
}