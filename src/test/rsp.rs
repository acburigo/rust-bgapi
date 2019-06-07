use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_end {
    pub result: Error,
}

impl FromBytes for dtm_end {
    fn from_bytes(data: &[u8]) -> dtm_end {
        let mut cursor = Cursor::new(data);
        dtm_end {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for dtm_end {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_rx {
    pub result: Error,
}

impl FromBytes for dtm_rx {
    fn from_bytes(data: &[u8]) -> dtm_rx {
        let mut cursor = Cursor::new(data);
        dtm_rx {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for dtm_rx {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_tx {
    pub result: Error,
}

impl FromBytes for dtm_tx {
    fn from_bytes(data: &[u8]) -> dtm_tx {
        let mut cursor = Cursor::new(data);
        dtm_tx {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for dtm_tx {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
