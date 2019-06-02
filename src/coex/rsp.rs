use bytes::{Buf, BufMut};
use std::io::Cursor;

use parser::{FromBytes, ToBytes};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_counters {
    pub result: u16,
    pub counters: Box<[u8]>,
}

impl FromBytes for get_counters {
    fn from_bytes(data: &[u8]) -> get_counters {
        let mut cursor = Cursor::new(data);
        get_counters {
            result: cursor.get_u16_le(),
            counters: cursor.bytes().to_vec().into_boxed_slice(),
        }
    }
}

impl ToBytes for get_counters {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.put_u16_le(self.result);
        bytes.extend_from_slice(&self.counters);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_options {
    pub result: u16,
}

impl FromBytes for set_options {
    fn from_bytes(data: &[u8]) -> set_options {
        let mut cursor = Cursor::new(data);
        set_options {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_options {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}
