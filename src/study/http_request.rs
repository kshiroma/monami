use std::fs::File;
use std::io::prelude::*;
//use std::net::TcpListener;
use std::net::TcpStream;

#[test]
fn test() {
    let mut stream = TcpStream::connect("localhost:8080").unwrap();
    let mut file = File::open("test/httprequest/requets_post.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    //println!("{}", content);
    stream.write(content.as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut buffer = [0; 4096 *4];
    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));
}
