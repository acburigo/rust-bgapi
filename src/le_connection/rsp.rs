use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct close {
    pub result: Error,
}

impl From<&[u8]> for close {
    fn from(data: &[u8]) -> close {
        let mut cursor = Cursor::new(data);
        close {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for close {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct disable_slave_latency {
    pub result: Error,
}

impl From<&[u8]> for disable_slave_latency {
    fn from(data: &[u8]) -> disable_slave_latency {
        let mut cursor = Cursor::new(data);
        disable_slave_latency {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for disable_slave_latency {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct get_rssi {
    pub result: Error,
}

impl From<&[u8]> for get_rssi {
    fn from(data: &[u8]) -> get_rssi {
        let mut cursor = Cursor::new(data);
        get_rssi {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for get_rssi {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_parameters {
    pub result: Error,
}

impl From<&[u8]> for set_parameters {
    fn from(data: &[u8]) -> set_parameters {
        let mut cursor = Cursor::new(data);
        set_parameters {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_parameters {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_phy {
    pub result: Error,
}

impl From<&[u8]> for set_phy {
    fn from(data: &[u8]) -> set_phy {
        let mut cursor = Cursor::new(data);
        set_phy {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_phy {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
