use std::io::{Read, Write};

struct Buffer {
    buf: [u8; 4098],
    buf_size: usize,
    pos: i64,
    offset: i64,
    length: i64,
}

struct UpIn {
    buf: Buffer,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            buf: [0; 4098],
            buf_size: 4098,
            pos: 0,
            offset: 0,
            length: 0,
        }
    }
}


impl UpIn {
    fn new() -> Box<UpIn> {
        let mut buf = Buffer::new();
        Box::new(UpIn {
            buf: buf
        })
    }
}

struct UpOut {
    buf: Buffer,
}

struct DownIn {
    buf: Buffer,
}

struct DownOut {
    buf: Buffer,
}

pub enum IncomingReadError {
    IOError,
    ProtocolError,
    Timeout,
}

pub enum OutgoingWriteError {
    IOError,
    Timeout,
}


pub trait MonamiStream {}


pub trait Incoming {
    fn read(&self) -> Result<(), IncomingReadError>;
}

pub trait Outgoing {
    fn write() -> Result<(), OutgoingWriteError>;
}


/*pub trait Upstream {
    //relay
}

pub trait Downstream {
    //
}
*/


struct Upstream {
    read: Box<Read>,
    write: Box<Write>,
}

impl Upstream {
    fn new(read: Box<Read>, write: Box<Write>) -> Box<Upstream> {
        Box::new(Upstream {
            read,
            write,
        })
    }


    fn read(self) {}
}


impl Incoming for UpIn {
    fn read(&self) -> Result<(), IncomingReadError> {
        self.buf.buf;
        Ok(())
    }
}

impl UpIn {}

impl Outgoing for UpOut {
    fn write() -> Result<(), OutgoingWriteError> {
        Ok(())
    }
}


impl Incoming for DownIn {
    fn read(&self) -> Result<(), IncomingReadError> {
        Ok(())
    }
}


impl Outgoing for DownOut {
    fn write() -> Result<(), OutgoingWriteError> {
        Ok(())
    }
}

