use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct boot {
    pub version: u32,
}

impl FromBytes for boot {
    fn from_bytes(data: &[u8]) -> boot {
        let mut cursor = Cursor::new(data);
        boot {
            version: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for boot {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.version);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct boot_failure {
    pub reason: u16,
}

impl FromBytes for boot_failure {
    fn from_bytes(data: &[u8]) -> boot_failure {
        let mut cursor = Cursor::new(data);
        boot_failure {
            reason: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for boot_failure {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.reason);
        bytes
    }
}
