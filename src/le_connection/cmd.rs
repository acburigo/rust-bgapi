use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct close {
    pub connection: u8,
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
