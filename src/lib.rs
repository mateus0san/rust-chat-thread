use std::io::{self, BufRead, BufReader, Read, Write};
use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

pub struct Client {
    username: String,
    writer: TcpStream,
    reader: BufReader<TcpStream>,
    ip: String,
}

#[derive(Debug)]
pub enum ClientError {
    IO(io::Error),
    Validation(std::string::FromUtf8Error),
}

impl From<io::Error> for ClientError {
    fn from(error: io::Error) -> Self {
        ClientError::IO(error)
    }
}

impl From<std::string::FromUtf8Error> for ClientError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ClientError::Validation(error)
    }
}

impl Client {
    pub fn try_new(stream: TcpStream) -> Result<Self, ClientError> {
        let ip = stream.peer_addr()?.to_string();

        let writer = stream.try_clone()?;
        let reader = BufReader::new(stream);

        Ok(Client::new(writer, reader, ip))
    }

    pub fn set_username(&mut self) -> Result<(), ClientError> {
        self.writer
            .write_all("Type your username (max 16 characters): ".as_bytes())?;

        const MAX_USERNAME_LEN: usize = 16;
        let mut username = String::with_capacity(MAX_USERNAME_LEN);

        self.reader
            .by_ref()
            .take(MAX_USERNAME_LEN as u64)
            .read_line(&mut username)?;

        self.username = username.trim().to_string();

        Ok(())
    }

    fn new(writer: TcpStream, reader: BufReader<TcpStream>, ip: String) -> Self {
        Self {
            username: String::from("Guest"),
            writer,
            reader,
            ip,
        }
    }

    pub fn write(&mut self, msg: &str) -> Result<(), ClientError> {
        self.writer.write_all(msg.as_bytes())?;

        Ok(())
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }
}

pub struct Server {
    listener: TcpListener,
    addr: SocketAddr,
    sender: mpsc::Sender<Client>,
    pub connection: Arc<Mutex<Connection>>,
}

pub enum Connection {
    Drop,
    Accept,
    End,
}

impl Connection {
    pub fn drop(mut stream: TcpStream) {
        let _ = stream.write_all("The server is full".as_bytes());
        drop(stream);
        eprintln!("INFO: Dropping connection");
        thread::sleep(Duration::from_secs(5));
    }
}

impl Server {
    fn new(listener: TcpListener, addr: SocketAddr, sender: mpsc::Sender<Client>) -> Self {
        Self {
            listener,
            addr,
            sender,
            connection: Arc::new(Mutex::new(Connection::Accept)),
        }
    }

    pub fn bind_server() -> (Self, mpsc::Receiver<Client>) {
        let addr = "0.0.0.0:1337";
        eprintln!("Binding server at {addr}");

        let listener = TcpListener::bind(addr).expect("ERROR: Failed to bind server");
        let server_address = listener
            .local_addr()
            .expect("ERROR: Failed to get server address");

        let (sender, receiver) = mpsc::channel();

        (Server::new(listener, server_address, sender), receiver)
    }

    pub fn run(&self) {
        eprintln!("Serving on {}", self.addr);

        for stream in self.listener.incoming() {
            let Ok(stream) = stream else {
                let _ = dbg!(stream);
                continue;
            };

            let connection = self.connection.lock().unwrap();
            match *connection {
                Connection::Accept => (),
                Connection::End => return,
                Connection::Drop => {
                    Connection::drop(stream);
                    continue;
                }
            }

            let connection = Arc::clone(&self.connection);
            let sender = self.sender.clone();
            thread::spawn(move || {
                if let Ok(client) = Self::handle_client(stream)
                    && sender.send(client).is_err()
                {
                    *connection.lock().unwrap() = Connection::End;
                }
            });
        }
    }

    fn handle_client(stream: TcpStream) -> Result<Client, ()> {
        match Client::try_new(stream) {
            Ok(client) => {
                eprintln!("New client at the address {}", client.ip());
                Ok(client)
            }
            Err(ClientError::IO(e)) => {
                eprintln!("ERROR: IO error from client_handle {e}");
                Err(())
            }
            Err(e) => {
                eprintln!("ERROR: from client_handle {e:#?}");
                Err(())
            }
        }
    }
}
