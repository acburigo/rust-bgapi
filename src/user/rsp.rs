use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct message_to_target {
    pub result: Error,
    pub data: Vec<u8>,
}

impl From<&[u8]> for message_to_target {
    fn from(data: &[u8]) -> message_to_target {
        let mut cursor = Cursor::new(data);
        let result = FromPrimitive::from_u16(cursor.get_u16_le()).unwrap();
        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .expect("Failed to read bytes.");
        message_to_target { result, data }
    }
}

impl Into<Vec<u8>> for message_to_target {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.extend(self.data.iter());
        bytes
    }
}
