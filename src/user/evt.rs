use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct message_to_host {
    pub data: Vec<u8>,
}

impl FromBytes for message_to_host {
    fn from_bytes(data: &[u8]) -> message_to_host {
        let mut cursor = Cursor::new(data);
        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .expect("Failed to read bytes.");
        message_to_host { data }
    }
}

impl ToBytes for message_to_host {
    fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}
