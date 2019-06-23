use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct boot {
    pub version: u32,
}

impl From<&[u8]> for boot {
    fn from(data: &[u8]) -> boot {
        let mut cursor = Cursor::new(data);
        boot {
            version: cursor.get_u32_le(),
        }
    }
}

impl Into<Vec<u8>> for boot {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.version);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct boot_failure {
    pub reason: Error,
}

impl From<&[u8]> for boot_failure {
    fn from(data: &[u8]) -> boot_failure {
        let mut cursor = Cursor::new(data);
        boot_failure {
            reason: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for boot_failure {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.reason.clone() as u16);
        bytes
    }
}
