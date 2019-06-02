use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct ps_erase {
    pub key: u16,
}

impl FromBytes for ps_erase {
    fn from_bytes(data: &[u8]) -> ps_erase {
        let mut cursor = Cursor::new(data);
        ps_erase {
            key: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for ps_erase {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.key);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct ps_erase_all {}

impl FromBytes for ps_erase_all {
    fn from_bytes(_: &[u8]) -> ps_erase_all {
        ps_erase_all {}
    }
}

impl ToBytes for ps_erase_all {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct ps_load {
    pub key: u16,
}

impl FromBytes for ps_load {
    fn from_bytes(data: &[u8]) -> ps_load {
        let mut cursor = Cursor::new(data);
        ps_load {
            key: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for ps_load {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.key);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct ps_save {
    pub key: u16,
    pub value: Vec<u8>,
}

impl FromBytes for ps_save {
    fn from_bytes(data: &[u8]) -> ps_save {
        let mut cursor = Cursor::new(data);
        let key = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        ps_save { key, value }
    }
}

impl ToBytes for ps_save {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.key);
        bytes.extend(self.value.iter());
        bytes
    }
}
