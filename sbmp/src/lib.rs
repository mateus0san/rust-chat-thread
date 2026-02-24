// Simple Binary Messaging Protocol
// +------------+------------+------------+----------------+
// | 1 byte     | 1 byte     | 4 bytes    | N bytes        |
// | version    | type       | length     | payload        |
// +------------+------------+------------+----------------+

pub mod read;
pub mod write;

pub mod collections {
    use std::io;

    pub const PROTOCOL_VERSION: u8 = 1;

    pub struct Frame {
        pub header: Header,
        pub payload: Vec<u8>,
    }

    impl Frame {
        fn new(header: Header, payload: Vec<u8>) -> Self {
            Self { header, payload }
        }

        pub fn try_new(header: Header, payload: Vec<u8>) -> Result<Self, SBMPError> {
            if header.content_length == payload.len() {
                Ok(Frame::new(header, payload))
            } else {
                Err(SBMPError::ContentLenDiff)
            }
        }
    }

    pub enum SBMPError {
        ContentLenDiff,
        UsizetoU32,
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

    pub struct Header {
        content_type: ContentType,
        content_length: usize,
    }

    impl Header {
        fn new(content_type: ContentType, content_length: usize) -> Self {
            Self {
                content_type,
                content_length,
            }
        }

        pub fn try_new(content_type: ContentType, content_length: u32) -> Result<Self, SBMPError> {
            const KB: u32 = 1024;
            const MB: u32 = KB * 1024;

            let max = match content_type {
                ContentType::UTF8 => 4 * KB,
                ContentType::Binary => 4 * MB,
            };

            if content_length > max {
                return Err(SBMPError::LargeFrame);
            }

            let Ok(content_length) = usize::try_from(content_length) else {
                return Err(SBMPError::UsizetoU32);
            };

            Ok(Header::new(content_type, content_length))
        }

        pub fn content_type(&self) -> &ContentType {
            &self.content_type
        }

        pub fn content_len(&self) -> usize {
            self.content_length
        }
    }

    #[repr(u8)]
    pub enum ContentType {
        UTF8,
        Binary,
    }

    impl TryFrom<u8> for ContentType {
        type Error = SBMPError;

        fn try_from(content_type: u8) -> Result<Self, Self::Error> {
            let content_type = match content_type {
                0 => ContentType::UTF8,
                1 => ContentType::Binary,
                _ => return Err(SBMPError::UknownContentType(content_type)),
            };

            Ok(content_type)
        }
    }
}
