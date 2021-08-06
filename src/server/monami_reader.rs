struct MonamiReader {
    read: Read,
    stringBuffer: String,
    buf: [u8; 4096 * 4],
    length: i64,
    position: i64,
    bufferedReader: BufReader,
}


impl MonamiReader {
    pub fn new(read: &mut dyn Read) -> Self {
        MonamiReader {
            read: read,
            stringBuffer: String::new(),
            buf: [0; 4096 * 4],
            length: 0,
            position: 0,
            bufferedReader: BufferedReader::new(read),
        }
    }
    //
    fn read(&mut self) -> i64 {
        let slice = buf[self.position..];
        let length = self.read.read(slice).unwrap_or(-1);
        self.length = length + length;
        length
    }
    //
    pub fn read_line(&mut self) -> String {
        let mut line = String::new();
        bufferedReader.read_line(&mut line);
        return line;
    }
}