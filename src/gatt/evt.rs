use bytes::{Buf, BufMut};
use error::Error;
use gatt::AttOpcode;
use num_traits::FromPrimitive;
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct characteristic {
    pub connection: u8,
    pub characteristic: u16,
    pub properties: u8,
    pub uuid: Vec<u8>,
}

impl From<&[u8]> for characteristic {
    fn from(data: &[u8]) -> characteristic {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let properties = cursor.get_u8();
        let mut uuid: Vec<u8> = Vec::new();
        cursor
            .read_to_end(&mut uuid)
            .expect("Failed to read bytes.");
        characteristic {
            connection,
            characteristic,
            properties,
            uuid,
        }
    }
}

impl Into<Vec<u8>> for characteristic {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.properties);
        bytes.extend(self.uuid.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct characteristic_value {
    pub connection: u8,
    pub characteristic: u16,
    pub att_opcode: AttOpcode,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl From<&[u8]> for characteristic_value {
    fn from(data: &[u8]) -> characteristic_value {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let att_opcode = FromPrimitive::from_u8(cursor.get_u8()).unwrap();
        let offset = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        characteristic_value {
            connection,
            characteristic,
            att_opcode,
            offset,
            value,
        }
    }
}

impl Into<Vec<u8>> for characteristic_value {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.att_opcode.clone() as u8);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct descriptor {
    pub connection: u8,
    pub descriptor: u16,
    pub uuid: Vec<u8>,
}

impl From<&[u8]> for descriptor {
    fn from(data: &[u8]) -> descriptor {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let descriptor = cursor.get_u16_le();
        let mut uuid: Vec<u8> = Vec::new();
        cursor
            .read_to_end(&mut uuid)
            .expect("Failed to read bytes.");
        descriptor {
            connection,
            descriptor,
            uuid,
        }
    }
}

impl Into<Vec<u8>> for descriptor {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.descriptor);
        bytes.extend(self.uuid.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct descriptor_value {
    pub connection: u8,
    pub descriptor: u16,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl From<&[u8]> for descriptor_value {
    fn from(data: &[u8]) -> descriptor_value {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let descriptor = cursor.get_u16_le();
        let offset = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        descriptor_value {
            connection,
            descriptor,
            offset,
            value,
        }
    }
}

impl Into<Vec<u8>> for descriptor_value {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.descriptor);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct mtu_exchanged {
    pub connection: u8,
    pub mtu: u16,
}

impl From<&[u8]> for mtu_exchanged {
    fn from(data: &[u8]) -> mtu_exchanged {
        let mut cursor = Cursor::new(data);
        mtu_exchanged {
            connection: cursor.get_u8(),
            mtu: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for mtu_exchanged {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.mtu);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct procedure_completed {
    pub connection: u8,
    pub result: Error,
}

impl From<&[u8]> for procedure_completed {
    fn from(data: &[u8]) -> procedure_completed {
        let mut cursor = Cursor::new(data);
        procedure_completed {
            connection: cursor.get_u8(),
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for procedure_completed {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct service {
    pub connection: u8,
    pub service: u32,
    pub uuid: Vec<u8>,
}

impl From<&[u8]> for service {
    fn from(data: &[u8]) -> service {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let service = cursor.get_u32_le();
        let mut uuid: Vec<u8> = Vec::new();
        cursor
            .read_to_end(&mut uuid)
            .expect("Failed to read bytes.");
        service {
            connection,
            service,
            uuid,
        }
    }
}

impl Into<Vec<u8>> for service {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes.extend(self.uuid.iter());
        bytes
    }
}
