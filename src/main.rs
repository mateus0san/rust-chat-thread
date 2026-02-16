use std::{
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use chat::{Client, Rooms, Text};

const ADDRESS: &str = "0.0.0.0:1337";

fn main() {
    let (sender, receiver) = mpsc::channel();
    let listener = TcpListener::bind(ADDRESS).expect("Could not bind address");

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(s) = stream
                && sender.send(s).is_err()
            {
                eprintln!("INFO: receiver from listener disconnected.");
                break;
            }
        }
    });

    let rooms = Arc::new(Mutex::new(Rooms::default()));

    while let Ok(stream) = receiver.recv() {
        let rooms = Arc::clone(&rooms);

        thread::spawn(move || {
            let Ok(addr) = stream.peer_addr() else {
                return;
            };

            let text_name = match get_client_name(&stream) {
                Ok(name) => Text::try_new(name),
                Err(e) => {
                    eprintln!("ERROR: {addr} {e}",);
                    return;
                }
            };

            match text_name {
                Ok(name) => start_chat(rooms, Client::new(name, stream)),
                Err(_) => {
                    let name = Text::try_new(String::from("guest")).expect("guest must be valid");
                    start_chat(rooms, Client::new(name, stream))
                }
            }
        });
    }
}

fn get_client_name(stream: &TcpStream) -> Result<String, std::io::Error> {
    const USERNAME_LEN: usize = 25;

    let mut name = String::with_capacity(USERNAME_LEN);
    let mut buf_reader = BufReader::new(stream)
        .take(u64::try_from(USERNAME_LEN).expect("USERNAME_LEN may convert to u64"));

    buf_reader.read_line(&mut name)?;

    Ok(name)
}

fn start_chat(_rooms: Arc<Mutex<Rooms>>, _client: Client) {
    todo!()
}
