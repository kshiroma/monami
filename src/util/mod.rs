use std::io::prelude::*;

mod io {
    use std::io::{BufRead, Read};

    pub fn read_line(reader:&mut impl Read) -> String {
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
        string
    }

    pub fn read_line2(reader:&mut dyn BufRead) -> String {
        let mut string = String::new();
        reader.read_line(&mut string).unwrap();
        string.trim().to_string()
    }
}