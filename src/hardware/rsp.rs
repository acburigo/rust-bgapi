use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_lazy_soft_timer {
    pub result: Error,
}

impl From<&[u8]> for set_lazy_soft_timer {
    fn from(data: &[u8]) -> set_lazy_soft_timer {
        let mut cursor = Cursor::new(data);
        set_lazy_soft_timer {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_lazy_soft_timer {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_soft_timer {
    pub result: Error,
}

impl From<&[u8]> for set_soft_timer {
    fn from(data: &[u8]) -> set_soft_timer {
        let mut cursor = Cursor::new(data);
        set_soft_timer {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_soft_timer {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
