use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_completed {
    pub result: u16,
    pub number_of_packets: u16,
}

impl FromBytes for dtm_completed {
    fn from_bytes(data: &[u8]) -> dtm_completed {
        let mut cursor = Cursor::new(data);
        dtm_completed {
            result: cursor.get_u16_le(),
            number_of_packets: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for dtm_completed {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes.put_u16_le(self.number_of_packets);
        bytes
    }
}