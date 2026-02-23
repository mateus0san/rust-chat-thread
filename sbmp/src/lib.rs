use std::io::{self, BufReader, Read};
// Simple Binary Messaging Protocol
// +------------+------------+------------+----------------+
// | 1 byte     | 1 byte     | 4 bytes    | N bytes        |
// | version    | type       | length     | payload        |
// +------------+------------+------------+----------------+

pub enum SBMPError {
    WrongVersion,
    LargeFrame,
    IO(io::Error),
}

impl From<io::Error> for SBMPError {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

#[repr(u8)]
pub enum ContentType {
    UTF8 = 0,
}

pub struct Header {
    pub content_type: ContentType,
    pub content_lenght: u32,
}

pub struct Frame {
    pub header: Header,
    pub payload: Vec<u8>,
}

pub struct FrameReader<R: Read> {
    reader: BufReader<R>,
}


impl<R: Read> FrameReader<R> {
    pub fn new(stream: R) -> Self {
        Self {
            reader: BufReader::new(stream),
        }
    }

    pub fn read_frame(&mut self) -> io::Result<Vec<u8>> {
        let len = self.read_header()?;

        if len > 8 * 1024 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Frame too large",
            ));
        }

        let mut frame = vec![0u8; len as usize];

        self.reader.read_exact(&mut frame)?;

        Ok(frame)
    }

    fn read_header(&mut self) -> Result<Header, SBMPError> {
        // version, 1 byte
        let mut version = [0u8; 1];
        self.reader.read_exact(&mut version)?;
        if version[0] == 
        // content-type, 1 byte
        let mut version = [0u8; 1];
        self.reader.read_exact(&mut version)?;

        //
        let mut header = [0u8; 4];

        Ok(u32::from_be_bytes(header))
    }
}
