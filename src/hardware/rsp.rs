use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_lazy_soft_timer {
    pub result: u16,
}

impl FromBytes for set_lazy_soft_timer {
    fn from_bytes(data: &[u8]) -> set_lazy_soft_timer {
        let mut cursor = Cursor::new(data);
        set_lazy_soft_timer {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_lazy_soft_timer {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_soft_timer {
    pub result: u16,
}

impl FromBytes for set_soft_timer {
    fn from_bytes(data: &[u8]) -> set_soft_timer {
        let mut cursor = Cursor::new(data);
        set_soft_timer {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_soft_timer {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}
