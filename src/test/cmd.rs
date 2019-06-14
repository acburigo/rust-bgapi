use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use num_traits::FromPrimitive;
use parser::ToBytes;
use std::io::Cursor;
use test::{PacketType, Phy};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_end {}

impl dtm_end {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::test,
            message_id: 0x02,
        };
        let payload = dtm_end {};
        let payload = MessagePayload::cmd_test_dtm_end(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for dtm_end {
    fn from(_: &[u8]) -> dtm_end {
        dtm_end {}
    }
}

impl ToBytes for dtm_end {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_rx {
    pub channel: u8,
    pub phy: Phy,
}

impl dtm_rx {
    pub fn new(channel: u8, phy: Phy) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::test,
            message_id: 0x01,
        };
        let payload = dtm_rx { channel, phy };
        let payload = MessagePayload::cmd_test_dtm_rx(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for dtm_rx {
    fn from(data: &[u8]) -> dtm_rx {
        let mut cursor = Cursor::new(data);
        dtm_rx {
            channel: cursor.get_u8(),
            phy: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
        }
    }
}

impl ToBytes for dtm_rx {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.channel);
        bytes.put_u8(self.phy.clone() as u8);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_tx {
    pub packet_type: PacketType,
    pub length: u8,
    pub channel: u8,
    pub phy: Phy,
}

impl dtm_tx {
    pub fn new(packet_type: PacketType, length: u8, channel: u8, phy: Phy) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::test,
            message_id: 0x00,
        };
        let payload = dtm_tx {
            packet_type,
            length,
            channel,
            phy,
        };
        let payload = MessagePayload::cmd_test_dtm_tx(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for dtm_tx {
    fn from(data: &[u8]) -> dtm_tx {
        let mut cursor = Cursor::new(data);
        dtm_tx {
            packet_type: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
            length: cursor.get_u8(),
            channel: cursor.get_u8(),
            phy: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
        }
    }
}

impl ToBytes for dtm_tx {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.packet_type.clone() as u8);
        bytes.put_u8(self.length);
        bytes.put_u8(self.channel);
        bytes.put_u8(self.phy.clone() as u8);
        bytes
    }
}
