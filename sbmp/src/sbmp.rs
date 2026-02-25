use std::io;

pub const PROTOCOL_VERSION: u8 = 1;

pub mod content;
pub mod frame;
pub mod header;

pub use content::ContentType;
pub use frame::Frame;
pub use header::Header;

#[derive(Debug)]
pub enum SBMPError {
    ContentLenDiff,
    LengthConversion,
    UknownContentType(u8),
    WrongVersion,
    LargeFrame,
    IO(io::Error),
}

impl From<io::Error> for SBMPError {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}
