use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ps_erase {
    pub result: Error,
}

impl From<&[u8]> for ps_erase {
    fn from(data: &[u8]) -> ps_erase {
        let mut cursor = Cursor::new(data);
        ps_erase {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for ps_erase {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ps_erase_all {
    pub result: Error,
}

impl From<&[u8]> for ps_erase_all {
    fn from(data: &[u8]) -> ps_erase_all {
        let mut cursor = Cursor::new(data);
        ps_erase_all {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for ps_erase_all {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ps_load {
    pub result: Error,
    pub value: Vec<u8>,
}

impl From<&[u8]> for ps_load {
    fn from(data: &[u8]) -> ps_load {
        let mut cursor = Cursor::new(data);
        let result = FromPrimitive::from_u16(cursor.get_u16_le()).unwrap();
        let mut value = Vec::new();
        cursor.get_u8();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        ps_load { result, value }
    }
}

impl Into<Vec<u8>> for ps_load {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u8(self.value.len() as u8);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ps_save {
    pub result: Error,
}

impl From<&[u8]> for ps_save {
    fn from(data: &[u8]) -> ps_save {
        let mut cursor = Cursor::new(data);
        ps_save {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for ps_save {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
