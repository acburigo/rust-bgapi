use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct close {
    pub result: u16,
}

impl FromBytes for close {
    fn from_bytes(data: &[u8]) -> close {
        let mut cursor = Cursor::new(data);
        close {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for close {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct disable_slave_latency {
    pub result: u16,
}

impl FromBytes for disable_slave_latency {
    fn from_bytes(data: &[u8]) -> disable_slave_latency {
        let mut cursor = Cursor::new(data);
        disable_slave_latency {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for disable_slave_latency {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_rssi {
    pub result: u16,
}

impl FromBytes for get_rssi {
    fn from_bytes(data: &[u8]) -> get_rssi {
        let mut cursor = Cursor::new(data);
        get_rssi {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for get_rssi {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_parameters {
    pub result: u16,
}

impl FromBytes for set_parameters {
    fn from_bytes(data: &[u8]) -> set_parameters {
        let mut cursor = Cursor::new(data);
        set_parameters {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_parameters {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_phy {
    pub result: u16,
}

impl FromBytes for set_phy {
    fn from_bytes(data: &[u8]) -> set_phy {
        let mut cursor = Cursor::new(data);
        set_phy {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_phy {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}
