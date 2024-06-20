use std::net::{Shutdown, TcpStream};

pub struct MonamiWorker {
    //処理スレッドと//Managerとのやり取りをするスレッド
}


impl MonamiWorker {
    pub fn handle(&self, stream: Box<TcpStream>) -> std::io::Result<()> {
        let read = stream.clone();
    }
}