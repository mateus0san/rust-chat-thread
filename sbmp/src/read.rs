use crate::collections::*;
use std::io::Read;

pub struct FrameReader<R: Read> {
    reader: R,
}

impl<R: Read> FrameReader<R> {
    pub fn new(stream: R) -> Self {
        Self { reader: stream }
    }

    pub fn read_frame(&mut self) -> Result<Frame, SBMPError> {
        let header = self.read_header()?;

        let mut payload = vec![0u8; header.content_len()];
        self.reader.read_exact(&mut payload)?;

        Frame::try_new(header, payload)
    }

    fn read_header(&mut self) -> Result<Header, SBMPError> {
        // version, 1 byte
        let mut version = [0u8; 1];
        self.reader.read_exact(&mut version)?;
        if version[0] != PROTOCOL_VERSION {
            return Err(SBMPError::WrongVersion);
        }

        // content-type, 1 byte
        let mut content_type = [0u8; 1];
        self.reader.read_exact(&mut content_type)?;
        let content_type = ContentType::try_from(content_type[0])?;

        // content-length, 4 bytes
        let mut content_length = [0u8; 4];
        self.reader.read_exact(&mut content_length)?;
        let content_length = u32::from_be_bytes(content_length);

        Header::try_new(content_type, content_length)
    }
}
