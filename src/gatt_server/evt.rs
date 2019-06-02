use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct attribute_value {
    pub connection: u8,
    pub attribute: u16,
    pub att_opcode: u8,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl FromBytes for attribute_value {
    fn from_bytes(data: &[u8]) -> attribute_value {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let attribute = cursor.get_u16_le();
        let att_opcode = cursor.get_u8();
        let offset = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        attribute_value {
            connection,
            attribute,
            att_opcode,
            offset,
            value,
        }
    }
}

impl ToBytes for attribute_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.attribute);
        bytes.put_u8(self.att_opcode);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct characteristic_status {
    pub connection: u8,
    pub characteristic: u16,
    pub status_flags: u8,
    pub client_config_flags: u16,
}

impl FromBytes for characteristic_status {
    fn from_bytes(data: &[u8]) -> characteristic_status {
        let mut cursor = Cursor::new(data);
        characteristic_status {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
            status_flags: cursor.get_u8(),
            client_config_flags: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for characteristic_status {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.status_flags);
        bytes.put_u16_le(self.client_config_flags);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct execute_write_completed {
    pub connection: u8,
    pub result: u16,
}

impl FromBytes for execute_write_completed {
    fn from_bytes(data: &[u8]) -> execute_write_completed {
        let mut cursor = Cursor::new(data);
        execute_write_completed {
            connection: cursor.get_u8(),
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for execute_write_completed {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct user_read_request {
    pub connection: u8,
    pub characteristic: u16,
    pub att_opcode: u8,
    pub offset: u16,
}

impl FromBytes for user_read_request {
    fn from_bytes(data: &[u8]) -> user_read_request {
        let mut cursor = Cursor::new(data);
        user_read_request {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
            att_opcode: cursor.get_u8(),
            offset: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for user_read_request {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.att_opcode);
        bytes.put_u16_le(self.offset);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct user_write_request {
    pub connection: u8,
    pub characteristic: u16,
    pub att_opcode: u8,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl FromBytes for user_write_request {
    fn from_bytes(data: &[u8]) -> user_write_request {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let att_opcode = cursor.get_u8();
        let offset = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        user_write_request {
            connection,
            characteristic,
            att_opcode,
            offset,
            value,
        }
    }
}

impl ToBytes for user_write_request {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.att_opcode);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}
