use std::io::{self, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub enum Message {
    Broadcast(String),
    NewClient(Client),
    Drop(SocketAddr),
}

pub struct Client {
    ip: SocketAddr,
    writer: TcpStream,
}

impl Drop for Client {
    fn drop(&mut self) {
        eprintln!("INFO: dropping client with address {}", self.ip);
        let _ = self.writer.shutdown(std::net::Shutdown::Both);
    }
}

impl Client {
    pub fn try_new(stream: TcpStream) -> io::Result<Client> {
        let ip = stream.peer_addr()?;

        stream.set_read_timeout(Some(Duration::from_mins(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(15)))?;

        Ok(Client::new(stream, ip))
    }

    fn new(writer: TcpStream, ip: SocketAddr) -> Self {
        Self { ip, writer }
    }

    pub fn write(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_all(s.as_bytes())
    }

    pub fn ip(&self) -> SocketAddr {
        self.ip
    }
}

pub enum ConnectionEnd {
    Normal,
    ReceiverDropped,
}
