use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_characteristics {
    pub result: Error,
}

impl FromBytes for discover_characteristics {
    fn from_bytes(data: &[u8]) -> discover_characteristics {
        let mut cursor = Cursor::new(data);
        discover_characteristics {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for discover_characteristics {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_characteristics_by_uuid {
    pub result: Error,
}

impl FromBytes for discover_characteristics_by_uuid {
    fn from_bytes(data: &[u8]) -> discover_characteristics_by_uuid {
        let mut cursor = Cursor::new(data);
        discover_characteristics_by_uuid {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for discover_characteristics_by_uuid {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_descriptors {
    pub result: Error,
}

impl FromBytes for discover_descriptors {
    fn from_bytes(data: &[u8]) -> discover_descriptors {
        let mut cursor = Cursor::new(data);
        discover_descriptors {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for discover_descriptors {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_primary_services {
    pub result: Error,
}

impl FromBytes for discover_primary_services {
    fn from_bytes(data: &[u8]) -> discover_primary_services {
        let mut cursor = Cursor::new(data);
        discover_primary_services {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for discover_primary_services {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_primary_services_by_uuid {
    pub result: Error,
}

impl FromBytes for discover_primary_services_by_uuid {
    fn from_bytes(data: &[u8]) -> discover_primary_services_by_uuid {
        let mut cursor = Cursor::new(data);
        discover_primary_services_by_uuid {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for discover_primary_services_by_uuid {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct execute_characteristic_value_write {
    pub result: Error,
}

impl FromBytes for execute_characteristic_value_write {
    fn from_bytes(data: &[u8]) -> execute_characteristic_value_write {
        let mut cursor = Cursor::new(data);
        execute_characteristic_value_write {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for execute_characteristic_value_write {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct find_included_services {
    pub result: Error,
}

impl FromBytes for find_included_services {
    fn from_bytes(data: &[u8]) -> find_included_services {
        let mut cursor = Cursor::new(data);
        find_included_services {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for find_included_services {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct prepare_characteristic_value_reliable_write {
    pub result: Error,
    pub sent_len: u16,
}

impl FromBytes for prepare_characteristic_value_reliable_write {
    fn from_bytes(data: &[u8]) -> prepare_characteristic_value_reliable_write {
        let mut cursor = Cursor::new(data);
        prepare_characteristic_value_reliable_write {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            sent_len: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for prepare_characteristic_value_reliable_write {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.sent_len);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct prepare_characteristic_value_write {
    pub result: Error,
    pub sent_len: u16,
}

impl FromBytes for prepare_characteristic_value_write {
    fn from_bytes(data: &[u8]) -> prepare_characteristic_value_write {
        let mut cursor = Cursor::new(data);
        prepare_characteristic_value_write {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            sent_len: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for prepare_characteristic_value_write {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.sent_len);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_characteristic_value {
    pub result: Error,
}

impl FromBytes for read_characteristic_value {
    fn from_bytes(data: &[u8]) -> read_characteristic_value {
        let mut cursor = Cursor::new(data);
        read_characteristic_value {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for read_characteristic_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_characteristic_value_by_uuid {
    pub result: Error,
}

impl FromBytes for read_characteristic_value_by_uuid {
    fn from_bytes(data: &[u8]) -> read_characteristic_value_by_uuid {
        let mut cursor = Cursor::new(data);
        read_characteristic_value_by_uuid {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for read_characteristic_value_by_uuid {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_characteristic_value_from_offset {
    pub result: Error,
}

impl FromBytes for read_characteristic_value_from_offset {
    fn from_bytes(data: &[u8]) -> read_characteristic_value_from_offset {
        let mut cursor = Cursor::new(data);
        read_characteristic_value_from_offset {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for read_characteristic_value_from_offset {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_descriptor_value {
    pub result: Error,
}

impl FromBytes for read_descriptor_value {
    fn from_bytes(data: &[u8]) -> read_descriptor_value {
        let mut cursor = Cursor::new(data);
        read_descriptor_value {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for read_descriptor_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_multiple_characteristic_values {
    pub result: Error,
}

impl FromBytes for read_multiple_characteristic_values {
    fn from_bytes(data: &[u8]) -> read_multiple_characteristic_values {
        let mut cursor = Cursor::new(data);
        read_multiple_characteristic_values {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for read_multiple_characteristic_values {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct send_characteristic_confirmation {
    pub result: Error,
}

impl FromBytes for send_characteristic_confirmation {
    fn from_bytes(data: &[u8]) -> send_characteristic_confirmation {
        let mut cursor = Cursor::new(data);
        send_characteristic_confirmation {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for send_characteristic_confirmation {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_characteristic_notification {
    pub result: Error,
}

impl FromBytes for set_characteristic_notification {
    fn from_bytes(data: &[u8]) -> set_characteristic_notification {
        let mut cursor = Cursor::new(data);
        set_characteristic_notification {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_characteristic_notification {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_max_mtu {
    pub result: Error,
    pub max_mtu: u16,
}

impl FromBytes for set_max_mtu {
    fn from_bytes(data: &[u8]) -> set_max_mtu {
        let mut cursor = Cursor::new(data);
        set_max_mtu {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            max_mtu: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_max_mtu {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.max_mtu);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct write_characteristic_value {
    pub result: Error,
}

impl FromBytes for write_characteristic_value {
    fn from_bytes(data: &[u8]) -> write_characteristic_value {
        let mut cursor = Cursor::new(data);
        write_characteristic_value {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for write_characteristic_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct write_characteristic_value_without_response {
    pub result: Error,
    pub sent_len: u16,
}

impl FromBytes for write_characteristic_value_without_response {
    fn from_bytes(data: &[u8]) -> write_characteristic_value_without_response {
        let mut cursor = Cursor::new(data);
        write_characteristic_value_without_response {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            sent_len: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for write_characteristic_value_without_response {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.sent_len);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct write_descriptor_value {
    pub result: Error,
}

impl FromBytes for write_descriptor_value {
    fn from_bytes(data: &[u8]) -> write_descriptor_value {
        let mut cursor = Cursor::new(data);
        write_descriptor_value {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for write_descriptor_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
