use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_counters {
    pub reset: u8,
}

impl get_counters {
    pub fn new(reset: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::coex,
            message_id: 0x01,
        };
        let payload = get_counters { reset };
        let payload = MessagePayload::cmd_coex_get_counters(payload);
        Message { header, payload }
    }
}

impl FromBytes for get_counters {
    fn from_bytes(data: &[u8]) -> get_counters {
        let mut cursor = Cursor::new(data);
        get_counters {
            reset: cursor.get_u8(),
        }
    }
}

impl ToBytes for get_counters {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.reset);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_options {
    pub mask: u32,
    pub options: u32,
}

impl set_options {
    pub fn new(mask: u32, options: u32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x08,
            message_class: MessageClass::coex,
            message_id: 0x00,
        };
        let payload = set_options { mask, options };
        let payload = MessagePayload::cmd_coex_set_options(payload);
        Message { header, payload }
    }
}

impl FromBytes for set_options {
    fn from_bytes(data: &[u8]) -> set_options {
        let mut cursor = Cursor::new(data);
        set_options {
            mask: cursor.get_u32_le(),
            options: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for set_options {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.mask);
        bytes.put_u32_le(self.options);
        bytes
    }
}
