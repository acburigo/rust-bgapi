use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct flash_set_address {
    pub result: Error,
}

impl From<&[u8]> for flash_set_address {
    fn from(data: &[u8]) -> flash_set_address {
        let mut cursor = Cursor::new(data);
        flash_set_address {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for flash_set_address {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct flash_upload {
    pub result: Error,
}

impl From<&[u8]> for flash_upload {
    fn from(data: &[u8]) -> flash_upload {
        let mut cursor = Cursor::new(data);
        flash_upload {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for flash_upload {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct flash_upload_finish {
    pub result: Error,
}

impl From<&[u8]> for flash_upload_finish {
    fn from(data: &[u8]) -> flash_upload_finish {
        let mut cursor = Cursor::new(data);
        flash_upload_finish {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for flash_upload_finish {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
