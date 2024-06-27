use std::io::prelude::*;

mod io {
    use std::io::{Read};

    /// \r,\n,\r\nのいずれかによる終了を検知して文字列を返す
    pub fn read_line(reader: &mut impl Read) -> std::io::Result<String> {
        // そもそも、BufReadを受け取る実装に変更するか。
        let mut buffer: Vec<u8> = Vec::with_capacity(1024);
        loop {
            let mut data = [0; 1];
            let mut prev_is_cr = false;
            let no_use = reader.read(&mut data).unwrap();
            let a = data[0];
            if a == b'\n' {
                break;
            }
            if prev_is_cr {
                buffer.push(b'\r');
            }
            prev_is_cr = a == b'\r';
            if prev_is_cr {} else {
                buffer.push(a);
            }
        }
        let string = String::from_utf8(buffer).unwrap();
        Ok(string)
    }
}