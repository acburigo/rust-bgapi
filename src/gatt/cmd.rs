use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_characteristics {
    pub connection: u8,
    pub service: u32,
}

impl FromBytes for discover_characteristics {
    fn from_bytes(data: &[u8]) -> discover_characteristics {
        let mut cursor = Cursor::new(data);
        discover_characteristics {
            connection: cursor.get_u8(),
            service: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for discover_characteristics {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_characteristics_by_uuid {
    pub connection: u8,
    pub service: u32,
    pub uuid: [u8; 16],
}

impl FromBytes for discover_characteristics_by_uuid {
    fn from_bytes(data: &[u8]) -> discover_characteristics_by_uuid {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let service = cursor.get_u32_le();
        let mut uuid: [u8; 16] = Default::default();
        cursor.read_exact(&mut uuid).expect("Failed to read bytes.");
        discover_characteristics_by_uuid {
            connection,
            service,
            uuid,
        }
    }
}

impl ToBytes for discover_characteristics_by_uuid {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes.extend_from_slice(&self.uuid);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_descriptors {
    pub connection: u8,
    pub characteristic: u16,
}

impl FromBytes for discover_descriptors {
    fn from_bytes(data: &[u8]) -> discover_descriptors {
        let mut cursor = Cursor::new(data);
        discover_descriptors {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for discover_descriptors {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_primary_services {
    pub connection: u8,
}

impl FromBytes for discover_primary_services {
    fn from_bytes(data: &[u8]) -> discover_primary_services {
        let mut cursor = Cursor::new(data);
        discover_primary_services {
            connection: cursor.get_u8(),
        }
    }
}

impl ToBytes for discover_primary_services {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct discover_primary_services_by_uuid {
    pub connection: u8,
    pub uuid: [u8; 16],
}

impl FromBytes for discover_primary_services_by_uuid {
    fn from_bytes(data: &[u8]) -> discover_primary_services_by_uuid {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let mut uuid: [u8; 16] = Default::default();
        cursor.read_exact(&mut uuid).expect("Failed to read bytes.");
        discover_primary_services_by_uuid { connection, uuid }
    }
}

impl ToBytes for discover_primary_services_by_uuid {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.extend_from_slice(&self.uuid);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct execute_characteristic_value_write {
    pub connection: u8,
    pub flags: u8,
}

impl FromBytes for execute_characteristic_value_write {
    fn from_bytes(data: &[u8]) -> execute_characteristic_value_write {
        let mut cursor = Cursor::new(data);
        execute_characteristic_value_write {
            connection: cursor.get_u8(),
            flags: cursor.get_u8(),
        }
    }
}

impl ToBytes for execute_characteristic_value_write {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.flags);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct find_included_services {
    pub connection: u8,
    pub service: u32,
}

impl FromBytes for find_included_services {
    fn from_bytes(data: &[u8]) -> find_included_services {
        let mut cursor = Cursor::new(data);
        find_included_services {
            connection: cursor.get_u8(),
            service: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for find_included_services {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct prepare_characteristic_value_reliable_write {
    pub connection: u8,
    pub characteristic: u16,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl FromBytes for prepare_characteristic_value_reliable_write {
    fn from_bytes(data: &[u8]) -> prepare_characteristic_value_reliable_write {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let offset = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        prepare_characteristic_value_reliable_write {
            connection,
            characteristic,
            offset,
            value,
        }
    }
}

impl ToBytes for prepare_characteristic_value_reliable_write {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct prepare_characteristic_value_write {
    pub connection: u8,
    pub characteristic: u16,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl FromBytes for prepare_characteristic_value_write {
    fn from_bytes(data: &[u8]) -> prepare_characteristic_value_write {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let offset = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        prepare_characteristic_value_write {
            connection,
            characteristic,
            offset,
            value,
        }
    }
}

impl ToBytes for prepare_characteristic_value_write {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_characteristic_value {
    pub connection: u8,
    pub characteristic: u16,
}

impl FromBytes for read_characteristic_value {
    fn from_bytes(data: &[u8]) -> read_characteristic_value {
        let mut cursor = Cursor::new(data);
        read_characteristic_value {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for read_characteristic_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_characteristic_value_by_uuid {
    pub connection: u8,
    pub service: u32,
    pub uuid: [u8; 16],
}

impl FromBytes for read_characteristic_value_by_uuid {
    fn from_bytes(data: &[u8]) -> read_characteristic_value_by_uuid {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let service = cursor.get_u32_le();
        let mut uuid: [u8; 16] = Default::default();
        cursor.read_exact(&mut uuid).expect("Failed to read bytes.");
        read_characteristic_value_by_uuid {
            connection,
            service,
            uuid,
        }
    }
}

impl ToBytes for read_characteristic_value_by_uuid {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes.extend_from_slice(&self.uuid);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_characteristic_value_from_offset {
    pub connection: u8,
    pub characteristic: u16,
    pub offset: u16,
    pub maxlen: u16,
}

impl FromBytes for read_characteristic_value_from_offset {
    fn from_bytes(data: &[u8]) -> read_characteristic_value_from_offset {
        let mut cursor = Cursor::new(data);
        read_characteristic_value_from_offset {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
            offset: cursor.get_u16_le(),
            maxlen: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for read_characteristic_value_from_offset {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u16_le(self.offset);
        bytes.put_u16_le(self.maxlen);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_descriptor_value {
    pub connection: u8,
    pub descriptor: u16,
}

impl FromBytes for read_descriptor_value {
    fn from_bytes(data: &[u8]) -> read_descriptor_value {
        let mut cursor = Cursor::new(data);
        read_descriptor_value {
            connection: cursor.get_u8(),
            descriptor: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for read_descriptor_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.descriptor);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct read_multiple_characteristic_values {
    pub connection: u8,
    pub characteristic_list: Vec<u16>,
}

impl FromBytes for read_multiple_characteristic_values {
    fn from_bytes(data: &[u8]) -> read_multiple_characteristic_values {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let mut characteristic_list = Vec::new();
        while (cursor.position() as usize) < data.len() {
            characteristic_list.push(cursor.get_u16_le());
        }
        read_multiple_characteristic_values {
            connection,
            characteristic_list,
        }
    }
}

impl ToBytes for read_multiple_characteristic_values {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        for x in &self.characteristic_list {
            bytes.put_u16_le(*x)
        }
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct send_characteristic_confirmation {
    pub connection: u8,
}

impl FromBytes for send_characteristic_confirmation {
    fn from_bytes(data: &[u8]) -> send_characteristic_confirmation {
        let mut cursor = Cursor::new(data);
        send_characteristic_confirmation {
            connection: cursor.get_u8(),
        }
    }
}

impl ToBytes for send_characteristic_confirmation {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_characteristic_notification {
    pub connection: u8,
    pub characteristic: u16,
    pub flags: u8,
}

impl FromBytes for set_characteristic_notification {
    fn from_bytes(data: &[u8]) -> set_characteristic_notification {
        let mut cursor = Cursor::new(data);
        set_characteristic_notification {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
            flags: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_characteristic_notification {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.flags);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_max_mtu {
    pub max_mtu: u16,
}

impl FromBytes for set_max_mtu {
    fn from_bytes(data: &[u8]) -> set_max_mtu {
        let mut cursor = Cursor::new(data);
        set_max_mtu {
            max_mtu: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_max_mtu {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.max_mtu);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct write_characteristic_value {
    pub connection: u8,
    pub characteristic: u16,
    pub value: Vec<u8>,
}

impl FromBytes for write_characteristic_value {
    fn from_bytes(data: &[u8]) -> write_characteristic_value {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        write_characteristic_value {
            connection,
            characteristic,
            value,
        }
    }
}

impl ToBytes for write_characteristic_value {
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
pub struct write_characteristic_value_without_response {
    pub connection: u8,
    pub characteristic: u16,
    pub value: Vec<u8>,
}

impl FromBytes for write_characteristic_value_without_response {
    fn from_bytes(data: &[u8]) -> write_characteristic_value_without_response {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let characteristic = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        write_characteristic_value_without_response {
            connection,
            characteristic,
            value,
        }
    }
}

impl ToBytes for write_characteristic_value_without_response {
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
pub struct write_descriptor_value {
    pub connection: u8,
    pub descriptor: u16,
    pub value: Vec<u8>,
}

impl FromBytes for write_descriptor_value {
    fn from_bytes(data: &[u8]) -> write_descriptor_value {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let descriptor = cursor.get_u16_le();
        let mut value = Vec::new();
        cursor
            .read_to_end(&mut value)
            .expect("Failed to read bytes.");
        write_descriptor_value {
            connection,
            descriptor,
            value,
        }
    }
}

impl ToBytes for write_descriptor_value {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.descriptor);
        bytes.extend(self.value.iter());
        bytes
    }
}
