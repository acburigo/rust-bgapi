use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_set_address {
    pub address: u32,
}

impl flash_set_address {
    pub fn new(address: u32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::dfu,
            message_id: 0x01,
        };
        let payload = flash_set_address { address };
        let payload = MessagePayload::cmd_dfu_flash_set_address(payload);
        Message { header, payload }
    }
}

impl FromBytes for flash_set_address {
    fn from_bytes(data: &[u8]) -> flash_set_address {
        let mut cursor = Cursor::new(data);
        flash_set_address {
            address: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for flash_set_address {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.address);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_upload {
    pub data: Vec<u8>,
}

impl flash_upload {
    pub fn new(data: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: data.len() as u8,
            message_class: MessageClass::dfu,
            message_id: 0x02,
        };
        let payload = flash_upload { data };
        let payload = MessagePayload::cmd_dfu_flash_upload(payload);
        Message { header, payload }
    }
}

impl FromBytes for flash_upload {
    fn from_bytes(data: &[u8]) -> flash_upload {
        flash_upload {
            data: data.to_vec(),
        }
    }
}

impl ToBytes for flash_upload {
    fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_upload_finish {}

impl flash_upload_finish {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::dfu,
            message_id: 0x03,
        };
        let payload = flash_upload_finish {};
        let payload = MessagePayload::cmd_dfu_flash_upload_finish(payload);
        Message { header, payload }
    }
}

impl FromBytes for flash_upload_finish {
    fn from_bytes(_: &[u8]) -> flash_upload_finish {
        flash_upload_finish {}
    }
}

impl ToBytes for flash_upload_finish {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct reset {
    pub dfu: u8,
}

impl reset {
    pub fn new(dfu: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::dfu,
            message_id: 0x00,
        };
        let payload = reset { dfu };
        let payload = MessagePayload::cmd_dfu_reset(payload);
        Message { header, payload }
    }
}

impl FromBytes for reset {
    fn from_bytes(data: &[u8]) -> reset {
        let mut cursor = Cursor::new(data);
        reset {
            dfu: cursor.get_u8(),
        }
    }
}

impl ToBytes for reset {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.dfu);
        bytes
    }
}
