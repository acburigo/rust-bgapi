use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_set_address {
    pub address: u32,
}

impl FromBytes for flash_set_address {
    fn from_bytes(data: &[u8]) -> flash_set_address {
        let mut cursor = Cursor::new(data);
        flash_set_address {
            address: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for flash_set_address {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.address);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_upload {
    pub data: Vec<u8>,
}

impl FromBytes for flash_upload {
    fn from_bytes(data: &[u8]) -> flash_upload {
        flash_upload {
            data: data.to_vec(),
        }
    }
}

impl ToBytes for flash_upload {
    fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct flash_upload_finish {}

impl FromBytes for flash_upload_finish {
    fn from_bytes(_: &[u8]) -> flash_upload_finish {
        flash_upload_finish {}
    }
}

impl ToBytes for flash_upload_finish {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct reset {
    pub dfu: u8,
}

impl FromBytes for reset {
    fn from_bytes(data: &[u8]) -> reset {
        let mut cursor = Cursor::new(data);
        reset {
            dfu: cursor.get_u8(),
        }
    }
}

impl ToBytes for reset {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.dfu);
        bytes
    }
}
