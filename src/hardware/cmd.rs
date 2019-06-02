use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_lazy_soft_timer {
    pub time: u32,
    pub slack: u32,
    pub handle: u8,
    pub single_shot: u8,
}

impl FromBytes for set_lazy_soft_timer {
    fn from_bytes(data: &[u8]) -> set_lazy_soft_timer {
        let mut cursor = Cursor::new(data);
        set_lazy_soft_timer {
            time: cursor.get_u32_le(),
            slack: cursor.get_u32_le(),
            handle: cursor.get_u8(),
            single_shot: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_lazy_soft_timer {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.time);
        bytes.put_u32_le(self.slack);
        bytes.put_u8(self.handle);
        bytes.put_u8(self.single_shot);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_soft_timer {
    pub time: u32,
    pub handle: u8,
    pub single_shot: u8,
}

impl FromBytes for set_soft_timer {
    fn from_bytes(data: &[u8]) -> set_soft_timer {
        let mut cursor = Cursor::new(data);
        set_soft_timer {
            time: cursor.get_u32_le(),
            handle: cursor.get_u8(),
            single_shot: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_soft_timer {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.time);
        bytes.put_u8(self.handle);
        bytes.put_u8(self.single_shot);
        bytes
    }
}
