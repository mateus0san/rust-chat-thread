use std::{
    collections::{HashMap, hash_map::Entry},
    net::TcpStream,
};

pub struct Client {
    name: String,
    stream: TcpStream,
}

impl Client {
    pub fn new(name: Text, stream: TcpStream) -> Self {
        Self {
            name: name.into_inner(),
            stream,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct Text(String);

#[derive(Debug)]
pub enum TextError {
    EmptyText,
}

impl Text {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Text {
    pub fn try_new(mut name: String) -> Result<Self, TextError> {
        name.retain(|c| !c.is_control());

        if name.is_empty() {
            return Err(TextError::EmptyText);
        }

        Ok(Self(name))
    }
}

pub enum Visibility {
    Public,
    Private,
}

pub struct Room {
    name: String,
    password: Option<String>,
    visibility: Visibility,
    clients: HashMap<String, Client>,
}

pub enum RoomError {
    RoomFull,
    AlreadyExists,
}

impl Room {
    const MAX_CLIENTS: usize = 32;
    pub fn new(name: Text, password: Option<Text>, visibility: Visibility) -> Self {
        let password = password.map(|p| p.into_inner());

        Self {
            name: name.into_inner(),
            password,
            visibility,
            clients: HashMap::with_capacity(Self::MAX_CLIENTS),
        }
    }

    pub fn try_add(&mut self, key: String, client: Client) -> Result<(), RoomError> {
        if self.clients.len() >= Self::MAX_CLIENTS {
            return Err(RoomError::RoomFull);
        }

        match self.clients.entry(key) {
            Entry::Occupied(_) => Err(RoomError::AlreadyExists),
            Entry::Vacant(entry) => {
                entry.insert(client);
                Ok(())
            }
        }
    }
}

#[derive(Default)]
pub struct Rooms {
    rooms: HashMap<String, Room>,
}

pub enum RoomsError {
    RoomsFull,
    AlreadyExists,
}

impl Rooms {
    const MAX_ROOMS: usize = 5;

    pub fn try_add(&mut self, key: String, room: Room) -> Result<(), RoomsError> {
        if self.rooms.len() >= Self::MAX_ROOMS {
            return Err(RoomsError::RoomsFull);
        }

        match self.rooms.entry(key) {
            Entry::Occupied(_) => Err(RoomsError::AlreadyExists),
            Entry::Vacant(entry) => {
                entry.insert(room);
                Ok(())
            }
        }
    }
}
