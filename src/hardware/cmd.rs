use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_lazy_soft_timer {
    pub time: u32,
    pub slack: u32,
    pub handle: u8,
    pub single_shot: u8,
}

impl set_lazy_soft_timer {
    pub fn new(time: u32, slack: u32, handle: u8, single_shot: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x0a,
            message_class: MessageClass::hardware,
            message_id: 0x0c,
        };
        let payload = set_lazy_soft_timer {
            time,
            slack,
            handle,
            single_shot,
        };
        let payload = MessagePayload::cmd_hardware_set_lazy_soft_timer(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_lazy_soft_timer {
    fn from(data: &[u8]) -> set_lazy_soft_timer {
        let mut cursor = Cursor::new(data);
        set_lazy_soft_timer {
            time: cursor.get_u32_le(),
            slack: cursor.get_u32_le(),
            handle: cursor.get_u8(),
            single_shot: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for set_lazy_soft_timer {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.time);
        bytes.put_u32_le(self.slack);
        bytes.put_u8(self.handle);
        bytes.put_u8(self.single_shot);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_soft_timer {
    pub time: u32,
    pub handle: u8,
    pub single_shot: u8,
}

impl set_soft_timer {
    pub fn new(time: u32, handle: u8, single_shot: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x06,
            message_class: MessageClass::hardware,
            message_id: 0x00,
        };
        let payload = set_soft_timer {
            time,
            handle,
            single_shot,
        };
        let payload = MessagePayload::cmd_hardware_set_soft_timer(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_soft_timer {
    fn from(data: &[u8]) -> set_soft_timer {
        let mut cursor = Cursor::new(data);
        set_soft_timer {
            time: cursor.get_u32_le(),
            handle: cursor.get_u8(),
            single_shot: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for set_soft_timer {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.time);
        bytes.put_u8(self.handle);
        bytes.put_u8(self.single_shot);
        bytes
    }
}
