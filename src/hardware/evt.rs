use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct soft_timer {
    pub handle: u8,
}

impl FromBytes for soft_timer {
    fn from_bytes(data: &[u8]) -> soft_timer {
        let mut cursor = Cursor::new(data);
        soft_timer {
            handle: cursor.get_u8(),
        }
    }
}

impl ToBytes for soft_timer {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes
    }
}
