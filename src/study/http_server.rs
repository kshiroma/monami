////https://doc.rust-jp.rs/book/second-edition/ch20-02-multithreaded.html
//use std::fs::File;
//use std::io::prelude::*;
//use std::net::TcpListener;
//use std::net::TcpStream;
//use std::sync::Arc;
//use std::sync::mpsc;
//use std::sync::Mutex;
//use std::thread;
//use std::time::Duration;
//
//// struct Job {}
//type Job = Box<FnBox + Send + 'static>;
//
//
//trait FnBox {
//    fn call_box(self: Box<Self>);
//}
//
//pub struct ThreadPool {
//    workers: Vec<Worker>,
//    sender: mpsc::Sender<Job>,
//}
//
//pub struct Worker {
//    id: usize,
//    thread: thread::JoinHandle<()>,
//}
//
//
//impl<F: FnOnce()> FnBox for F {
//    fn call_box(self: Box<F>) {
//        (*self)();
//    }
//}
//
//impl ThreadPool {
//    pub fn new(size: usize) -> ThreadPool {
//        assert!(size > 0);
//
//        let (sender, receiver) = mpsc::channel();
//        let receiver = Arc::new(Mutex::new(receiver));
//
//        let mut workers = Vec::with_capacity(size);
//        for id in 0..size {
//            workers.push(Worker::new(id, Arc::clone(&receiver)));
//        }
//
//        ThreadPool {
//            workers,
//            sender,
//        }
//    }
//
//    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
//        let job = Box::new(f);
//        self.sender.send(job).unwrap();
//    }
//}
//
//
//impl Worker {
//    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//        let thread = thread::spawn(move || {
//            loop {
//                let job = receiver.lock().unwrap().recv().unwrap();
//                println!("Worker {} get a job; executing.", id);
//
//                job.call_box();
//            }
//        });
//        Worker {
//            id,
//            thread,
//        }
//    }
//}
//
//fn create_response(stream: &mut TcpStream) -> String {
//    let mut buffer = [0; 512];
//    stream.read(&mut buffer).unwrap();
//    let get = b"GET / HTTP/1.1\r\n";
//    let sleep = b"GET /sleep HTTP/1.1\r\n";
//
//    let (status_line, filepath) = if buffer.starts_with(get) {
//        ("HTTP/1.1 200 OK", "test/httpresponse/hello.html")
//    } else if buffer.starts_with(sleep) {
//        thread::sleep(Duration::from_secs(5));
//        ("HTTP/1.1 200 OK", "test/httpresponse/hello.html")
//    } else {
//        ("HTTP/1.1 404 NOT FOUND", "test/httpresponse/404.html")
//    };
//
//    let mut file = File::open(filepath).unwrap();
//    let mut content = String::new();
//    file.read_to_string(&mut content).unwrap();
//
//    format!("{} \r\n\r\n{}", status_line, content)
//}
//
//fn handle_connection(mut stream: TcpStream) {
//    let response = create_response(&mut stream);
//
//    stream.write(response.as_bytes()).unwrap();
//    stream.flush().unwrap();
//}
//
//
//#[test]
//fn main() {
//    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//    let pool = ThreadPool::new(4);
//    for stream in listener.incoming() {
//        let stream = stream.unwrap();
//
//        pool.execute(|| {
//            handle_connection(stream);
//        });
//    }
//}
//
