use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct find_attribute {
    pub start: u16,
    pub atype: Vec<u8>,
}

impl FromBytes for find_attribute {
    fn from_bytes(data: &[u8]) -> find_attribute {
        let mut cursor = Cursor::new(data);
        let start = cursor.get_u16_le();
        let mut atype = Vec::new();
        cursor
            .read_to_end(&mut atype)
            .expect("Failed to read bytes.");
        find_attribute { start, atype }
    }
}

impl ToBytes for find_attribute {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.start);
        bytes.extend(self.atype.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_attribute_type {
    pub attribute: u16,
}

impl FromBytes for read_attribute_type {
    fn from_bytes(data: &[u8]) -> read_attribute_type {
        let mut cursor = Cursor::new(data);
        read_attribute_type {
            attribute: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for read_attribute_type {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.attribute);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_attribute_value {
    pub attribute: u16,
    pub offset: u16,
}

impl FromBytes for read_attribute_value {
    fn from_bytes(data: &[u8]) -> read_attribute_value {
        let mut cursor = Cursor::new(data);
        read_attribute_value {
            attribute: cursor.get_u16_le(),
            offset: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for read_attribute_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.attribute);
        bytes.put_u16_le(self.offset);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct send_characteristic_notification {
    pub connection: u8,
    pub characteristic: u16,
    pub value: Vec<u8>,
}

impl FromBytes for send_characteristic_notification {
    fn from_bytes(data: &[u8]) -> send_characteristic_notification {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        send_characteristic_notification {
            connection,
            characteristic,
            value,
        }
    }
}

impl ToBytes for send_characteristic_notification {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct send_user_read_response {
    pub connection: u8,
    pub characteristic: u16,
    pub att_errorcode: u8,
    pub value: Vec<u8>,
}

impl FromBytes for send_user_read_response {
    fn from_bytes(data: &[u8]) -> send_user_read_response {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let att_errorcode = cursor.get_u8();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        send_user_read_response {
            connection,
            characteristic,
            att_errorcode,
            value,
        }
    }
}

impl ToBytes for send_user_read_response {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.att_errorcode);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct send_user_write_response {
    pub connection: u8,
    pub characteristic: u16,
    pub att_errorcode: u8,
}

impl FromBytes for send_user_write_response {
    fn from_bytes(data: &[u8]) -> send_user_write_response {
        let mut cursor = Cursor::new(data);
        send_user_write_response {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
            att_errorcode: cursor.get_u8(),
        }
    }
}

impl ToBytes for send_user_write_response {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.att_errorcode);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_capabilities {
    pub caps: u32,
    pub reserved: u32,
}

impl FromBytes for set_capabilities {
    fn from_bytes(data: &[u8]) -> set_capabilities {
        let mut cursor = Cursor::new(data);
        set_capabilities {
            caps: cursor.get_u32_le(),
            reserved: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for set_capabilities {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.caps);
        bytes.put_u32_le(self.reserved);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct write_attribute_value {
    pub attribute: u16,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl FromBytes for write_attribute_value {
    fn from_bytes(data: &[u8]) -> write_attribute_value {
        let mut cursor = Cursor::new(data);
        let attribute = cursor.get_u16_le();
        let offset = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        write_attribute_value {
            attribute,
            offset,
            value,
        }
    }
}

impl ToBytes for write_attribute_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.attribute);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}
