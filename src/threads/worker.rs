use std::net::{Shutdown, TcpStream};

pub struct MonamiWorker {
    //処理スレッドと//Managerとのやり取りをするスレッド
    // JoinHandler
    // managerとのReceiver
}


impl MonamiWorker {
    pub fn handle(&self, stream: Box<TcpStream>) -> std::io::Result<()> {
        Ok(())
    }
}