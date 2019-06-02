use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_set_address {
    pub result: u16,
}

impl FromBytes for flash_set_address {
    fn from_bytes(data: &[u8]) -> flash_set_address {
        let mut cursor = Cursor::new(data);
        flash_set_address {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for flash_set_address {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_upload {
    pub result: u16,
}

impl FromBytes for flash_upload {
    fn from_bytes(data: &[u8]) -> flash_upload {
        let mut cursor = Cursor::new(data);
        flash_upload {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for flash_upload {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_upload_finish {
    pub result: u16,
}

impl FromBytes for flash_upload_finish {
    fn from_bytes(data: &[u8]) -> flash_upload_finish {
        let mut cursor = Cursor::new(data);
        flash_upload_finish {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for flash_upload_finish {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}
