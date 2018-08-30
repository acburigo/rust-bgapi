use std::io::Cursor;
use std::io::{Error, ErrorKind};

use MessageHeader;

pub enum Command {
    find_attribute {
        start: u16,
        atype: Box<[u8]>,
    },
    read_attribute_type {
        attribute: u16,
    },
    read_attribute_value {
        attribute: u16,
        offset: u16,
    },
    send_characteristic_notification {
        connection: u8,
        characteristic: u16,
        value: Box<[u8]>,
    },
    send_user_read_response {
        connection: u8,
        characteristic: u16,
        att_errorcode: u8,
        value: Box<[u8]>,
    },
    send_user_write_response {
        connection: u8,
        characteristic: u16,
        att_errorcode: u8,
    },
    set_capabilities {
        caps: u32,
        reserved: u32,
    },
    write_attribute_value {
        attribute: u16,
        offset: u16,
        value: Box<[u8]>,
    },
}

pub enum Response {
    find_attribute { result: u16, attribute: u16 },
    read_attribute_type { result: u16, atype: Box<[u8]> },
    read_attribute_value { result: u16, value: Box<[u8]> },
    send_characteristic_notification { result: u16, sent_len: u16 },
    send_user_read_response { result: u16, sent_len: u16 },
    send_user_write_response { result: u16 },
    set_capabilities { result: u16 },
    write_attribute_value { result: u16 },
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
    attribute_value {
        connection: u8,
        attribute: u16,
        att_opcode: u8,
        offset: u16,
        value: Box<[u8]>,
    },
    characteristic_status {
        connection: u8,
        characteristic: u16,
        status_flags: u8,
        client_config_flags: u16,
    },
    execute_write_completed {
        connection: u8,
        result: u16,
    },
    user_read_request {
        connection: u8,
        characteristic: u16,
        att_opcode: u8,
        offset: u16,
    },
    user_write_request {
        connection: u8,
        characteristic: u16,
        att_opcode: u8,
        offset: u16,
        value: Box<[u8]>,
    },
}

impl Event {
    pub fn from_binary(cursor: &mut Cursor<&[u8]>, header: &MessageHeader) -> Result<Event, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}

pub enum CharacteristicStatusFlag {
    client_config = 1, // Characteristic client configuration has been changed.
    confirmation = 2,  // Characteristic confirmation has been received.
}
