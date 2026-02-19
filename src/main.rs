use std::{sync::Arc, thread};

use chat::{Connection, Server};

fn main() {
    let (server, receiver) = Server::bind_server();
    let state_server = Arc::clone(&server.connection);

    thread::spawn(move || {
        server.run();
    });

    let mut counter = 0;
    for client in receiver {
        eprintln!("Receiver: new client {}", client.ip());
        counter += 1;

        if counter == 5 {
            *state_server.lock().unwrap() = Connection::Drop;
        }
    }
}
