use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct find_attribute {
    pub result: Error,
    pub attribute: u16,
}

impl FromBytes for find_attribute {
    fn from_bytes(data: &[u8]) -> find_attribute {
        let mut cursor = Cursor::new(data);
        find_attribute {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            attribute: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for find_attribute {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.attribute);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_attribute_type {
    pub result: Error,
    pub atype: Vec<u8>,
}

impl FromBytes for read_attribute_type {
    fn from_bytes(data: &[u8]) -> read_attribute_type {
        let mut cursor = Cursor::new(data);
        let result = FromPrimitive::from_u16(cursor.get_u16_le()).unwrap();
        let mut atype = Vec::new();
        cursor
            .read_to_end(&mut atype)
            .expect("Failed to read bytes.");
        read_attribute_type { result, atype }
    }
}

impl ToBytes for read_attribute_type {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.extend(self.atype.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_attribute_value {
    pub result: Error,
    pub value: Vec<u8>,
}

impl FromBytes for read_attribute_value {
    fn from_bytes(data: &[u8]) -> read_attribute_value {
        let mut cursor = Cursor::new(data);
        let result = FromPrimitive::from_u16(cursor.get_u16_le()).unwrap();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        read_attribute_value { result, value }
    }
}

impl ToBytes for read_attribute_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct send_characteristic_notification {
    pub result: Error,
    pub sent_len: u16,
}

impl FromBytes for send_characteristic_notification {
    fn from_bytes(data: &[u8]) -> send_characteristic_notification {
        let mut cursor = Cursor::new(data);
        send_characteristic_notification {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            sent_len: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for send_characteristic_notification {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.sent_len);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct send_user_read_response {
    pub result: Error,
    pub sent_len: u16,
}

impl FromBytes for send_user_read_response {
    fn from_bytes(data: &[u8]) -> send_user_read_response {
        let mut cursor = Cursor::new(data);
        send_user_read_response {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            sent_len: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for send_user_read_response {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.sent_len);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct send_user_write_response {
    pub result: Error,
}

impl FromBytes for send_user_write_response {
    fn from_bytes(data: &[u8]) -> send_user_write_response {
        let mut cursor = Cursor::new(data);
        send_user_write_response {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for send_user_write_response {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_capabilities {
    pub result: Error,
}

impl FromBytes for set_capabilities {
    fn from_bytes(data: &[u8]) -> set_capabilities {
        let mut cursor = Cursor::new(data);
        set_capabilities {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_capabilities {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct write_attribute_value {
    pub result: Error,
}

impl FromBytes for write_attribute_value {
    fn from_bytes(data: &[u8]) -> write_attribute_value {
        let mut cursor = Cursor::new(data);
        write_attribute_value {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for write_attribute_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
