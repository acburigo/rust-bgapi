use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_bt_address {}

impl FromBytes for get_bt_address {
    fn from_bytes(_: &[u8]) -> get_bt_address {
        get_bt_address {}
    }
}

impl ToBytes for get_bt_address {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_counters {
    pub reset: u8,
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
pub struct get_random_data {
    pub length: u8,
}

impl FromBytes for get_random_data {
    fn from_bytes(data: &[u8]) -> get_random_data {
        let mut cursor = Cursor::new(data);
        get_random_data {
            length: cursor.get_u8(),
        }
    }
}

impl ToBytes for get_random_data {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.length);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct halt {
    pub halt: u8,
}

impl FromBytes for halt {
    fn from_bytes(data: &[u8]) -> halt {
        let mut cursor = Cursor::new(data);
        halt {
            halt: cursor.get_u8(),
        }
    }
}

impl ToBytes for halt {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.halt);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
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

impl FromBytes for hello {
    fn from_bytes(_: &[u8]) -> hello {
        hello {}
    }
}

impl ToBytes for hello {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct reset {
    pub dfu: u8,
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

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_bt_address {
    pub address: [u8; 6],
}

impl FromBytes for set_bt_address {
    fn from_bytes(data: &[u8]) -> set_bt_address {
        let mut cursor = Cursor::new(data);
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        set_bt_address { address }
    }
}

impl ToBytes for set_bt_address {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.address);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_device_name {
    pub dtype: u8,
    pub name: Vec<u8>,
}

impl FromBytes for set_device_name {
    fn from_bytes(data: &[u8]) -> set_device_name {
        let mut cursor = Cursor::new(data);
        let dtype = cursor.get_u8();
        let mut name = Vec::new();
        cursor
            .read_to_end(&mut name)
            .expect("Failed to read bytes.");
        set_device_name { dtype, name }
    }
}

impl ToBytes for set_device_name {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.dtype);
        bytes.extend(self.name.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_tx_power {
    pub power: i16,
}

impl FromBytes for set_tx_power {
    fn from_bytes(data: &[u8]) -> set_tx_power {
        let mut cursor = Cursor::new(data);
        set_tx_power {
            power: cursor.get_i16_le(),
        }
    }
}

impl ToBytes for set_tx_power {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_i16_le(self.power);
        bytes
    }
}
