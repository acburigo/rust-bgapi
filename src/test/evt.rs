use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use parser::ToBytes;
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_completed {
    pub result: Error,
    pub number_of_packets: u16,
}

impl From<&[u8]> for dtm_completed {
    fn from(data: &[u8]) -> dtm_completed {
        let mut cursor = Cursor::new(data);
        dtm_completed {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            number_of_packets: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for dtm_completed {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.number_of_packets);
        bytes
    }
}
