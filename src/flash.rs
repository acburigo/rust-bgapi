use std::io::Cursor;
use std::io::{Error, ErrorKind};

use MessageHeader;

pub enum Command {
    ps_erase { key: u16 },
    ps_erase_all,
    ps_load { key: u16 },
    ps_save { key: u16, value: Box<[u8]> },
}

pub enum Response {
    ps_erase { result: u16 },
    ps_erase_all { result: u16 },
    ps_load { result: u16, value: Box<[u8]> },
    ps_save { result: u16 },
}

impl Response {
    pub fn from_binary(
        cursor: &mut Cursor<&[u8]>,
        header: &MessageHeader,
    ) -> Result<Response, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}
