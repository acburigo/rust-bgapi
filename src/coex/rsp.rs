use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct get_counters {
    pub result: Error,
    pub counters: Box<[u8]>,
}

impl From<&[u8]> for get_counters {
    fn from(data: &[u8]) -> get_counters {
        let mut cursor = Cursor::new(data);
        get_counters {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            counters: cursor.bytes().to_vec().into_boxed_slice(),
        }
    }
}

impl Into<Vec<u8>> for get_counters {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.extend_from_slice(&self.counters);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_options {
    pub result: Error,
}

impl From<&[u8]> for set_options {
    fn from(data: &[u8]) -> set_options {
        let mut cursor = Cursor::new(data);
        set_options {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_options {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
