use std::io::prelude::*;

pub fn read_line(reader:&mut dyn Read) -> String {
        let mut line: Vec<u8> = Vec::new();
        loop {
            let mut data = [0; 1];
            let mut prev_is_cr = false;
            let _ = reader.read(&mut data).unwrap_or(2);
            let a:u8 = data[0];
            if a == b'\n' {
                break;
            }
            if prev_is_cr {
                line.push(b'\r');
            }
            prev_is_cr = a == b'\r';
            if prev_is_cr {} else {
                line.push(data[0]);
            }
        }
        let string = String::from_utf8(line).unwrap();
    //println!("{}", string);
    string
}

pub fn read_line2(reader:&mut dyn BufRead) -> String {
    let mut string = String::new();
    reader.read_line(&mut string).unwrap();
    string.trim().to_string()
}

#[test]
fn test_read_first_line() {
    use std::fs;
    use std::fs::File;
//use std::io::Read;
    let path = "test/httprequest/requets_get.txt";
    let _string = fs::read_to_string(path).unwrap();

    let mut file = File::open(path).unwrap();
    let first_line = read_line(&mut file);
    assert_eq!(first_line, "GET /favicon.ico HTTP/1.1");
}


#[test]
fn test() {
    let aaa = "aaa".to_string();
    println!("{}",aaa);
}
