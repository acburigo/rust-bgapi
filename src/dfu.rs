use std::io::Cursor;
use std::io::{Error, ErrorKind};

use MessageHeader;

pub enum Command {
    flash_set_address { address: u32 },
    flash_upload { data: Box<[u8]> },
    flash_upload_finish,
    reset { dfu: u8 },
}

pub enum Response {
    flash_set_address { result: u16 },
    flash_upload { result: u16 },
    flash_upload_finish { result: u16 },
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
    boot { version: u32 },
    boot_failure { reason: u16 },
}

impl Event {
    pub fn from_binary(cursor: &mut Cursor<&[u8]>, header: &MessageHeader) -> Result<Event, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}
