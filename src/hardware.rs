use std::io::Cursor;
use std::io::{Error, ErrorKind};

use MessageHeader;

pub enum Command {
    set_lazy_soft_timer {
        time: u32,
        slack: u32,
        handle: u8,
        single_shot: u8,
    },
    set_soft_timer {
        time: u32,
        handle: u8,
        single_shot: u8,
    },
}

pub enum Response {
    set_lazy_soft_timer { result: u16 },
    set_soft_timer { result: u16 },
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
    soft_timer { handle: u8 },
}

impl Event {
    pub fn from_binary(cursor: &mut Cursor<&[u8]>, header: &MessageHeader) -> Result<Event, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}
