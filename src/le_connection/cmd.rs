use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct close {
    pub connection: u8,
}

impl close {
    pub fn new(connection: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::le_connection,
            message_id: 0x04,
        };
        let payload = close { connection };
        let payload = MessagePayload::cmd_le_connection_close(payload);
        Message { header, payload }
    }
}

impl FromBytes for close {
    fn from_bytes(data: &[u8]) -> close {
        let mut cursor = Cursor::new(data);
        close {
            connection: cursor.get_u8(),
        }
    }
}

impl ToBytes for close {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct disable_slave_latency {
    pub connection: u8,
    pub disable: u8,
}

impl disable_slave_latency {
    pub fn new(connection: u8, disable: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x02,
        };
        let payload = disable_slave_latency {
            connection,
            disable,
        };
        let payload = MessagePayload::cmd_le_connection_disable_slave_latency(payload);
        Message { header, payload }
    }
}

impl FromBytes for disable_slave_latency {
    fn from_bytes(data: &[u8]) -> disable_slave_latency {
        let mut cursor = Cursor::new(data);
        disable_slave_latency {
            connection: cursor.get_u8(),
            disable: cursor.get_u8(),
        }
    }
}

impl ToBytes for disable_slave_latency {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.disable);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_rssi {
    pub connection: u8,
}

impl get_rssi {
    pub fn new(connection: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::le_connection,
            message_id: 0x01,
        };
        let payload = get_rssi { connection };
        let payload = MessagePayload::cmd_le_connection_get_rssi(payload);
        Message { header, payload }
    }
}

impl FromBytes for get_rssi {
    fn from_bytes(data: &[u8]) -> get_rssi {
        let mut cursor = Cursor::new(data);
        get_rssi {
            connection: cursor.get_u8(),
        }
    }
}

impl ToBytes for get_rssi {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_parameters {
    pub connection: u8,
    pub min_interval: u16,
    pub max_interval: u16,
    pub latency: u16,
    pub timeout: u16,
}

impl set_parameters {
    pub fn new(
        connection: u8,
        min_interval: u16,
        max_interval: u16,
        latency: u16,
        timeout: u16,
    ) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x09,
            message_class: MessageClass::le_connection,
            message_id: 0x00,
        };
        let payload = set_parameters {
            connection,
            min_interval,
            max_interval,
            latency,
            timeout,
        };
        let payload = MessagePayload::cmd_le_connection_set_parameters(payload);
        Message { header, payload }
    }
}

impl FromBytes for set_parameters {
    fn from_bytes(data: &[u8]) -> set_parameters {
        let mut cursor = Cursor::new(data);
        set_parameters {
            connection: cursor.get_u8(),
            min_interval: cursor.get_u16_le(),
            max_interval: cursor.get_u16_le(),
            latency: cursor.get_u16_le(),
            timeout: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_parameters {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.min_interval);
        bytes.put_u16_le(self.max_interval);
        bytes.put_u16_le(self.latency);
        bytes.put_u16_le(self.timeout);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_phy {
    pub connection: u8,
    pub phy: u8,
}

impl set_phy {
    pub fn new(connection: u8, phy: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x03,
        };
        let payload = set_phy { connection, phy };
        let payload = MessagePayload::cmd_le_connection_set_phy(payload);
        Message { header, payload }
    }
}

impl FromBytes for set_phy {
    fn from_bytes(data: &[u8]) -> set_phy {
        let mut cursor = Cursor::new(data);
        set_phy {
            connection: cursor.get_u8(),
            phy: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_phy {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.phy);
        bytes
    }
}
