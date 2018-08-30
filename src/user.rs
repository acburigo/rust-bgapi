use std::io::Cursor;
use std::io::{Error, ErrorKind};

use MessageHeader;

pub enum Command {
    message_to_target { data: Box<[u8]> },
}

pub enum Response {
    message_to_target { result: u16, data: Box<[u8]> },
}

impl Response {
    pub fn from_binary(
        cursor: &mut Cursor<&[u8]>,
        header: &MessageHeader,
    ) -> Result<Response, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}

pub enum Event {
    message_to_host { data: Box<[u8]> },
}

impl Event {
    pub fn from_binary(cursor: &mut Cursor<&[u8]>, header: &MessageHeader) -> Result<Event, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}
