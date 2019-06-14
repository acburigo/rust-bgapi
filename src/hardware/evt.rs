use bytes::{Buf, BufMut};
use parser::ToBytes;
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct soft_timer {
    pub handle: u8,
}

impl From<&[u8]> for soft_timer {
    fn from(data: &[u8]) -> soft_timer {
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
