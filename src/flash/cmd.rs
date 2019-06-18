use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct ps_erase {
    pub key: u16,
}

impl ps_erase {
    pub fn new(key: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::flash,
            message_id: 0x04,
        };
        let payload = ps_erase { key };
        let payload = MessagePayload::cmd_flash_ps_erase(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for ps_erase {
    fn from(data: &[u8]) -> ps_erase {
        let mut cursor = Cursor::new(data);
        ps_erase {
            key: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for ps_erase {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.key);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct ps_erase_all {}

impl ps_erase_all {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::flash,
            message_id: 0x01,
        };
        let payload = ps_erase_all {};
        let payload = MessagePayload::cmd_flash_ps_erase_all(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for ps_erase_all {
    fn from(_: &[u8]) -> ps_erase_all {
        ps_erase_all {}
    }
}

impl Into<Vec<u8>> for ps_erase_all {
    fn into(self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct ps_load {
    pub key: u16,
}

impl ps_load {
    pub fn new(key: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::flash,
            message_id: 0x03,
        };
        let payload = ps_load { key };
        let payload = MessagePayload::cmd_flash_ps_load(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for ps_load {
    fn from(data: &[u8]) -> ps_load {
        let mut cursor = Cursor::new(data);
        ps_load {
            key: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for ps_load {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.key);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct ps_save {
    pub key: u16,
    pub value: Vec<u8>,
}

impl ps_save {
    pub fn new(key: u16, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02 + (value.len() as u8),
            message_class: MessageClass::flash,
            message_id: 0x02,
        };
        let payload = ps_save { key, value };
        let payload = MessagePayload::cmd_flash_ps_save(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for ps_save {
    fn from(data: &[u8]) -> ps_save {
        let mut cursor = Cursor::new(data);
        let key = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        ps_save { key, value }
    }
}

impl Into<Vec<u8>> for ps_save {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.key);
        bytes.extend(self.value.iter());
        bytes
    }
}
