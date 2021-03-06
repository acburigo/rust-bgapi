use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct get_bt_address {}

impl get_bt_address {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::system,
            message_id: 0x03,
        };
        let payload = get_bt_address {};
        let payload = MessagePayload::cmd_system_get_bt_address(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for get_bt_address {
    fn from(_: &[u8]) -> get_bt_address {
        get_bt_address {}
    }
}

impl Into<Vec<u8>> for get_bt_address {
    fn into(self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct get_counters {
    pub reset: u8,
}

impl get_counters {
    pub fn new(reset: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::system,
            message_id: 0x0f,
        };
        let payload = get_counters { reset };
        let payload = MessagePayload::cmd_system_get_counters(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for get_counters {
    fn from(data: &[u8]) -> get_counters {
        let mut cursor = Cursor::new(data);
        get_counters {
            reset: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for get_counters {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.reset);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct get_random_data {
    pub length: u8,
}

impl get_random_data {
    pub fn new(length: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::system,
            message_id: 0x0b,
        };
        let payload = get_random_data { length };
        let payload = MessagePayload::cmd_system_get_random_data(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for get_random_data {
    fn from(data: &[u8]) -> get_random_data {
        let mut cursor = Cursor::new(data);
        get_random_data {
            length: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for get_random_data {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.length);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct halt {
    pub halt: u8,
}

impl halt {
    pub fn new(halt: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::system,
            message_id: 0x0c,
        };
        let payload = halt { halt };
        let payload = MessagePayload::cmd_system_halt(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for halt {
    fn from(data: &[u8]) -> halt {
        let mut cursor = Cursor::new(data);
        halt {
            halt: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for halt {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.halt);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct hello {}

impl hello {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::system,
            message_id: 0x00,
        };
        let payload = hello {};
        let payload = MessagePayload::cmd_system_hello(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for hello {
    fn from(_: &[u8]) -> hello {
        hello {}
    }
}

impl Into<Vec<u8>> for hello {
    fn into(self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct reset {
    pub dfu: u8,
}

impl reset {
    pub fn new(dfu: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::system,
            message_id: 0x01,
        };
        let payload = reset { dfu };
        let payload = MessagePayload::cmd_system_reset(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for reset {
    fn from(data: &[u8]) -> reset {
        let mut cursor = Cursor::new(data);
        reset {
            dfu: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for reset {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.dfu);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_bt_address {
    pub address: [u8; 6],
}

impl set_bt_address {
    pub fn new(address: [u8; 6]) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: address.len() as u8,
            message_class: MessageClass::system,
            message_id: 0x04,
        };
        let payload = set_bt_address { address };
        let payload = MessagePayload::cmd_system_set_bt_address(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_bt_address {
    fn from(data: &[u8]) -> set_bt_address {
        let mut cursor = Cursor::new(data);
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        address.reverse();
        set_bt_address { address }
    }
}

impl Into<Vec<u8>> for set_bt_address {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.address.iter().rev());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_device_name {
    pub dtype: u8,
    pub name: Vec<u8>,
}

impl set_device_name {
    pub fn new(dtype: u8, name: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01 + (1 + name.len() as u8),
            message_class: MessageClass::system,
            message_id: 0x0d,
        };
        let payload = set_device_name { dtype, name };
        let payload = MessagePayload::cmd_system_set_device_name(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_device_name {
    fn from(data: &[u8]) -> set_device_name {
        let mut cursor = Cursor::new(data);
        let dtype = cursor.get_u8();
        let mut name = Vec::new();
        cursor.get_u8();
        cursor
            .read_to_end(&mut name)
            .expect("Failed to read bytes.");
        set_device_name { dtype, name }
    }
}

impl Into<Vec<u8>> for set_device_name {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.dtype);
        bytes.put_u8(self.name.len() as u8);
        bytes.extend(self.name.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_tx_power {
    pub power: i16,
}

impl set_tx_power {
    pub fn new(power: i16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x0a,
        };
        let payload = set_tx_power { power };
        let payload = MessagePayload::cmd_system_set_tx_power(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_tx_power {
    fn from(data: &[u8]) -> set_tx_power {
        let mut cursor = Cursor::new(data);
        set_tx_power {
            power: cursor.get_i16_le(),
        }
    }
}

impl Into<Vec<u8>> for set_tx_power {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_i16_le(self.power);
        bytes
    }
}
