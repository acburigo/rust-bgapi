use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct discover_characteristics {
    pub connection: u8,
    pub service: u32,
}

impl discover_characteristics {
    pub fn new(connection: u8, service: u32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05,
            message_class: MessageClass::gatt,
            message_id: 0x03,
        };
        let payload = discover_characteristics {
            connection,
            service,
        };
        let payload = MessagePayload::cmd_gatt_discover_characteristics(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for discover_characteristics {
    fn from(data: &[u8]) -> discover_characteristics {
        let mut cursor = Cursor::new(data);
        discover_characteristics {
            connection: cursor.get_u8(),
            service: cursor.get_u32_le(),
        }
    }
}

impl Into<Vec<u8>> for discover_characteristics {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct discover_characteristics_by_uuid {
    pub connection: u8,
    pub service: u32,
    pub uuid: [u8; 16],
}

impl discover_characteristics_by_uuid {
    pub fn new(connection: u8, service: u32, uuid: [u8; 16]) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05 + (uuid.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x04,
        };
        let payload = discover_characteristics_by_uuid {
            connection,
            service,
            uuid,
        };
        let payload = MessagePayload::cmd_gatt_discover_characteristics_by_uuid(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for discover_characteristics_by_uuid {
    fn from(data: &[u8]) -> discover_characteristics_by_uuid {
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

impl Into<Vec<u8>> for discover_characteristics_by_uuid {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes.extend_from_slice(&self.uuid);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct discover_descriptors {
    pub connection: u8,
    pub characteristic: u16,
}

impl discover_descriptors {
    pub fn new(connection: u8, characteristic: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::gatt,
            message_id: 0x06,
        };
        let payload = discover_descriptors {
            connection,
            characteristic,
        };
        let payload = MessagePayload::cmd_gatt_discover_descriptors(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for discover_descriptors {
    fn from(data: &[u8]) -> discover_descriptors {
        let mut cursor = Cursor::new(data);
        discover_descriptors {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for discover_descriptors {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct discover_primary_services {
    pub connection: u8,
}

impl discover_primary_services {
    pub fn new(connection: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::gatt,
            message_id: 0x01,
        };
        let payload = discover_primary_services { connection };
        let payload = MessagePayload::cmd_gatt_discover_primary_services(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for discover_primary_services {
    fn from(data: &[u8]) -> discover_primary_services {
        let mut cursor = Cursor::new(data);
        discover_primary_services {
            connection: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for discover_primary_services {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct discover_primary_services_by_uuid {
    pub connection: u8,
    pub uuid: [u8; 16],
}

impl discover_primary_services_by_uuid {
    pub fn new(connection: u8, uuid: [u8; 16]) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01 + (uuid.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x02,
        };
        let payload = discover_primary_services_by_uuid { connection, uuid };
        let payload = MessagePayload::cmd_gatt_discover_primary_services_by_uuid(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for discover_primary_services_by_uuid {
    fn from(data: &[u8]) -> discover_primary_services_by_uuid {
        let mut cursor = Cursor::new(data);
        let connection = cursor.get_u8();
        let mut uuid: [u8; 16] = Default::default();
        cursor.read_exact(&mut uuid).expect("Failed to read bytes.");
        discover_primary_services_by_uuid { connection, uuid }
    }
}

impl Into<Vec<u8>> for discover_primary_services_by_uuid {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.extend_from_slice(&self.uuid);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct execute_characteristic_value_write {
    pub connection: u8,
    pub flags: u8,
}

impl execute_characteristic_value_write {
    pub fn new(connection: u8, flags: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x0c,
        };
        let payload = execute_characteristic_value_write { connection, flags };
        let payload = MessagePayload::cmd_gatt_execute_characteristic_value_write(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for execute_characteristic_value_write {
    fn from(data: &[u8]) -> execute_characteristic_value_write {
        let mut cursor = Cursor::new(data);
        execute_characteristic_value_write {
            connection: cursor.get_u8(),
            flags: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for execute_characteristic_value_write {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.flags);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct find_included_services {
    pub connection: u8,
    pub service: u32,
}

impl find_included_services {
    pub fn new(connection: u8, service: u32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05,
            message_class: MessageClass::gatt,
            message_id: 0x10,
        };
        let payload = find_included_services {
            connection,
            service,
        };
        let payload = MessagePayload::cmd_gatt_find_included_services(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for find_included_services {
    fn from(data: &[u8]) -> find_included_services {
        let mut cursor = Cursor::new(data);
        find_included_services {
            connection: cursor.get_u8(),
            service: cursor.get_u32_le(),
        }
    }
}

impl Into<Vec<u8>> for find_included_services {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct prepare_characteristic_value_reliable_write {
    pub connection: u8,
    pub characteristic: u16,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl prepare_characteristic_value_reliable_write {
    pub fn new(connection: u8, characteristic: u16, offset: u16, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05 + (value.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x13,
        };
        let payload = prepare_characteristic_value_reliable_write {
            connection,
            characteristic,
            offset,
            value,
        };
        let payload = MessagePayload::cmd_gatt_prepare_characteristic_value_reliable_write(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for prepare_characteristic_value_reliable_write {
    fn from(data: &[u8]) -> prepare_characteristic_value_reliable_write {
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

impl Into<Vec<u8>> for prepare_characteristic_value_reliable_write {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct prepare_characteristic_value_write {
    pub connection: u8,
    pub characteristic: u16,
    pub offset: u16,
    pub value: Vec<u8>,
}

impl prepare_characteristic_value_write {
    pub fn new(connection: u8, characteristic: u16, offset: u16, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05 + (value.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x0b,
        };
        let payload = prepare_characteristic_value_write {
            connection,
            characteristic,
            offset,
            value,
        };
        let payload = MessagePayload::cmd_gatt_prepare_characteristic_value_write(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for prepare_characteristic_value_write {
    fn from(data: &[u8]) -> prepare_characteristic_value_write {
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

impl Into<Vec<u8>> for prepare_characteristic_value_write {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct read_characteristic_value {
    pub connection: u8,
    pub characteristic: u16,
}

impl read_characteristic_value {
    pub fn new(connection: u8, characteristic: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::gatt,
            message_id: 0x07,
        };
        let payload = read_characteristic_value {
            connection,
            characteristic,
        };
        let payload = MessagePayload::cmd_gatt_read_characteristic_value(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for read_characteristic_value {
    fn from(data: &[u8]) -> read_characteristic_value {
        let mut cursor = Cursor::new(data);
        read_characteristic_value {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for read_characteristic_value {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct read_characteristic_value_by_uuid {
    pub connection: u8,
    pub service: u32,
    pub uuid: [u8; 16],
}

impl read_characteristic_value_by_uuid {
    pub fn new(connection: u8, service: u32, uuid: [u8; 16]) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05 + (uuid.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x08,
        };
        let payload = read_characteristic_value_by_uuid {
            connection,
            service,
            uuid,
        };
        let payload = MessagePayload::cmd_gatt_read_characteristic_value_by_uuid(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for read_characteristic_value_by_uuid {
    fn from(data: &[u8]) -> read_characteristic_value_by_uuid {
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

impl Into<Vec<u8>> for read_characteristic_value_by_uuid {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.service);
        bytes.extend_from_slice(&self.uuid);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct read_characteristic_value_from_offset {
    pub connection: u8,
    pub characteristic: u16,
    pub offset: u16,
    pub maxlen: u16,
}

impl read_characteristic_value_from_offset {
    pub fn new(connection: u8, characteristic: u16, offset: u16, maxlen: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x07,
            message_class: MessageClass::gatt,
            message_id: 0x12,
        };
        let payload = read_characteristic_value_from_offset {
            connection,
            characteristic,
            offset,
            maxlen,
        };
        let payload = MessagePayload::cmd_gatt_read_characteristic_value_from_offset(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for read_characteristic_value_from_offset {
    fn from(data: &[u8]) -> read_characteristic_value_from_offset {
        let mut cursor = Cursor::new(data);
        read_characteristic_value_from_offset {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
            offset: cursor.get_u16_le(),
            maxlen: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for read_characteristic_value_from_offset {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u16_le(self.offset);
        bytes.put_u16_le(self.maxlen);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct read_descriptor_value {
    pub connection: u8,
    pub descriptor: u16,
}

impl read_descriptor_value {
    pub fn new(connection: u8, descriptor: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::gatt,
            message_id: 0x0e,
        };
        let payload = read_descriptor_value {
            connection,
            descriptor,
        };
        let payload = MessagePayload::cmd_gatt_read_descriptor_value(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for read_descriptor_value {
    fn from(data: &[u8]) -> read_descriptor_value {
        let mut cursor = Cursor::new(data);
        read_descriptor_value {
            connection: cursor.get_u8(),
            descriptor: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for read_descriptor_value {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.descriptor);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct read_multiple_characteristic_values {
    pub connection: u8,
    pub characteristic_list: Vec<u16>,
}

impl read_multiple_characteristic_values {
    pub fn new(connection: u8, characteristic_list: Vec<u16>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01 + (2 * characteristic_list.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x11,
        };
        let payload = read_multiple_characteristic_values {
            connection,
            characteristic_list,
        };
        let payload = MessagePayload::cmd_gatt_read_multiple_characteristic_values(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for read_multiple_characteristic_values {
    fn from(data: &[u8]) -> read_multiple_characteristic_values {
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

impl Into<Vec<u8>> for read_multiple_characteristic_values {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        for x in &self.characteristic_list {
            bytes.put_u16_le(*x)
        }
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct send_characteristic_confirmation {
    pub connection: u8,
}

impl send_characteristic_confirmation {
    pub fn new(connection: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::gatt,
            message_id: 0x0d,
        };
        let payload = send_characteristic_confirmation { connection };
        let payload = MessagePayload::cmd_gatt_send_characteristic_confirmation(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for send_characteristic_confirmation {
    fn from(data: &[u8]) -> send_characteristic_confirmation {
        let mut cursor = Cursor::new(data);
        send_characteristic_confirmation {
            connection: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for send_characteristic_confirmation {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_characteristic_notification {
    pub connection: u8,
    pub characteristic: u16,
    pub flags: u8,
}

impl set_characteristic_notification {
    pub fn new(connection: u8, characteristic: u16, flags: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt,
            message_id: 0x05,
        };
        let payload = set_characteristic_notification {
            connection,
            characteristic,
            flags,
        };
        let payload = MessagePayload::cmd_gatt_set_characteristic_notification(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_characteristic_notification {
    fn from(data: &[u8]) -> set_characteristic_notification {
        let mut cursor = Cursor::new(data);
        set_characteristic_notification {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
            flags: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for set_characteristic_notification {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.put_u8(self.flags);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_max_mtu {
    pub max_mtu: u16,
}

impl set_max_mtu {
    pub fn new(max_mtu: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x00,
        };
        let payload = set_max_mtu { max_mtu };
        let payload = MessagePayload::cmd_gatt_set_max_mtu(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_max_mtu {
    fn from(data: &[u8]) -> set_max_mtu {
        let mut cursor = Cursor::new(data);
        set_max_mtu {
            max_mtu: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for set_max_mtu {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.max_mtu);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct write_characteristic_value {
    pub connection: u8,
    pub characteristic: u16,
    pub value: Vec<u8>,
}

impl write_characteristic_value {
    pub fn new(connection: u8, characteristic: u16, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03 + (value.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x09,
        };
        let payload = write_characteristic_value {
            connection,
            characteristic,
            value,
        };
        let payload = MessagePayload::cmd_gatt_write_characteristic_value(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for write_characteristic_value {
    fn from(data: &[u8]) -> write_characteristic_value {
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

impl Into<Vec<u8>> for write_characteristic_value {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct write_characteristic_value_without_response {
    pub connection: u8,
    pub characteristic: u16,
    pub value: Vec<u8>,
}

impl write_characteristic_value_without_response {
    pub fn new(connection: u8, characteristic: u16, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03 + (value.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x0a,
        };
        let payload = write_characteristic_value_without_response {
            connection,
            characteristic,
            value,
        };
        let payload = MessagePayload::cmd_gatt_write_characteristic_value_without_response(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for write_characteristic_value_without_response {
    fn from(data: &[u8]) -> write_characteristic_value_without_response {
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

impl Into<Vec<u8>> for write_characteristic_value_without_response {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.characteristic);
        bytes.extend(self.value.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct write_descriptor_value {
    pub connection: u8,
    pub descriptor: u16,
    pub value: Vec<u8>,
}

impl write_descriptor_value {
    pub fn new(connection: u8, descriptor: u16, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03 + (value.len() as u8),
            message_class: MessageClass::gatt,
            message_id: 0x0f,
        };
        let payload = write_descriptor_value {
            connection,
            descriptor,
            value,
        };
        let payload = MessagePayload::cmd_gatt_write_descriptor_value(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for write_descriptor_value {
    fn from(data: &[u8]) -> write_descriptor_value {
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

impl Into<Vec<u8>> for write_descriptor_value {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.descriptor);
        bytes.extend(self.value.iter());
        bytes
    }
}
