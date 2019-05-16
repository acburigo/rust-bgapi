use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_gatt_discover_characteristics(
            rsp::discover_characteristics::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_gatt_discover_characteristics_by_uuid(
            rsp::discover_characteristics_by_uuid::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x06,
        } => Ok(MessagePayload::rsp_gatt_discover_descriptors(
            rsp::discover_descriptors::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_gatt_discover_primary_services(
            rsp::discover_primary_services::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_gatt_discover_primary_services_by_uuid(
            rsp::discover_primary_services_by_uuid::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x0c,
        } => Ok(MessagePayload::rsp_gatt_execute_characteristic_value_write(
            rsp::execute_characteristic_value_write::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x10,
        } => Ok(MessagePayload::rsp_gatt_find_included_services(
            rsp::find_included_services::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x04,
            message_class: 0x09,
            message_id: 0x13,
        } => Ok(
            MessagePayload::rsp_gatt_prepare_characteristic_value_reliable_write(
                rsp::prepare_characteristic_value_reliable_write::from_bytes(buffer),
            ),
        ),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x04,
            message_class: 0x09,
            message_id: 0x0b,
        } => Ok(MessagePayload::rsp_gatt_prepare_characteristic_value_write(
            rsp::prepare_characteristic_value_write::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x07,
        } => Ok(MessagePayload::rsp_gatt_read_characteristic_value(
            rsp::read_characteristic_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x08,
        } => Ok(MessagePayload::rsp_gatt_read_characteristic_value_by_uuid(
            rsp::read_characteristic_value_by_uuid::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x12,
        } => Ok(
            MessagePayload::rsp_gatt_read_characteristic_value_from_offset(
                rsp::read_characteristic_value_from_offset::from_bytes(buffer),
            ),
        ),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x0e,
        } => Ok(MessagePayload::rsp_gatt_read_descriptor_value(
            rsp::read_descriptor_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x11,
        } => Ok(
            MessagePayload::rsp_gatt_read_multiple_characteristic_values(
                rsp::read_multiple_characteristic_values::from_bytes(buffer),
            ),
        ),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x0d,
        } => Ok(MessagePayload::rsp_gatt_send_characteristic_confirmation(
            rsp::send_characteristic_confirmation::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x05,
        } => Ok(MessagePayload::rsp_gatt_set_characteristic_notification(
            rsp::set_characteristic_notification::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x04,
            message_class: 0x09,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_gatt_set_max_mtu(
            rsp::set_max_mtu::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x09,
        } => Ok(MessagePayload::rsp_gatt_write_characteristic_value(
            rsp::write_characteristic_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x04,
            message_class: 0x09,
            message_id: 0x0a,
        } => Ok(
            MessagePayload::rsp_gatt_write_characteristic_value_without_response(
                rsp::write_characteristic_value_without_response::from_bytes(buffer),
            ),
        ),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x09,
            message_id: 0x0f,
        } => Ok(MessagePayload::rsp_gatt_write_descriptor_value(
            rsp::write_descriptor_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: _,
            message_class: 0x09,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_gatt_characteristic(
            evt::characteristic::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: _,
            message_class: 0x09,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_gatt_characteristic_value(
            evt::characteristic_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: _,
            message_class: 0x09,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_gatt_descriptor(
            evt::descriptor::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: _,
            message_class: 0x09,
            message_id: 0x05,
        } => Ok(MessagePayload::evt_gatt_descriptor_value(
            evt::descriptor_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x03,
            message_class: 0x09,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_gatt_mtu_exchanged(
            evt::mtu_exchanged::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x03,
            message_class: 0x09,
            message_id: 0x06,
        } => Ok(MessagePayload::evt_gatt_procedure_completed(
            evt::procedure_completed::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
pub enum AttOpcode {
    read_by_type_request = 8,       // Read by type request
    read_by_type_response = 9,      // Read by type response
    read_request = 10,              // Read request
    read_response = 11,             // Read response
    read_blob_request = 12,         // Read blob request
    read_blob_response = 13,        // Read blob response
    read_multiple_request = 14,     // Read multiple request
    read_multiple_response = 15,    // Read multiple response
    write_request = 18,             // Write request
    write_response = 19,            // Write response
    write_command = 82,             // Write command
    prepare_write_request = 22,     // Prepare write request
    prepare_write_response = 23,    // Prepare write response
    execute_write_request = 24,     // Execute write request
    execute_write_response = 25,    // Execute write response
    handle_value_notification = 27, // Notification
    handle_value_indication = 29,   // Indication
}

#[allow(non_camel_case_types)]
pub enum ClientConfigFlag {
    disable = 0,      // Disable notifications and indications
    notification = 1, // Notification
    indication = 2,   // Indication
}

#[allow(non_camel_case_types)]
pub enum execute_write_flag {
    cancel = 0, // Cancel all queued writes
    commit = 1, // Commit all queued writes
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct discover_characteristics {
        connection: u8,
        service: u32,
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
    pub struct discover_characteristics_by_uuid {
        connection: u8,
        service: u32,
        uuid: [u8; 16],
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
    pub struct discover_descriptors {
        connection: u8,
        characteristic: u16,
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
    pub struct discover_primary_services {
        connection: u8,
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
    pub struct discover_primary_services_by_uuid {
        connection: u8,
        uuid: [u8; 16],
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
    pub struct execute_characteristic_value_write {
        connection: u8,
        flags: u8,
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
    pub struct find_included_services {
        connection: u8,
        service: u32,
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
    pub struct prepare_characteristic_value_reliable_write {
        connection: u8,
        characteristic: u16,
        offset: u16,
        value: Vec<u8>,
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
    pub struct prepare_characteristic_value_write {
        connection: u8,
        characteristic: u16,
        offset: u16,
        value: Vec<u8>,
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
    pub struct read_characteristic_value {
        connection: u8,
        characteristic: u16,
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
    pub struct read_characteristic_value_by_uuid {
        connection: u8,
        service: u32,
        uuid: [u8; 16],
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
    pub struct read_characteristic_value_from_offset {
        connection: u8,
        characteristic: u16,
        offset: u16,
        maxlen: u16,
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
    pub struct read_descriptor_value {
        connection: u8,
        descriptor: u16,
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
    pub struct read_multiple_characteristic_values {
        connection: u8,
        characteristic_list: Vec<u16>,
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
    pub struct send_characteristic_confirmation {
        connection: u8,
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
    pub struct set_characteristic_notification {
        connection: u8,
        characteristic: u16,
        flags: u8,
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
    pub struct set_max_mtu {
        max_mtu: u16,
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
    pub struct write_characteristic_value {
        connection: u8,
        characteristic: u16,
        value: Vec<u8>,
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
    pub struct write_characteristic_value_without_response {
        connection: u8,
        characteristic: u16,
        value: Vec<u8>,
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
    pub struct write_descriptor_value {
        connection: u8,
        descriptor: u16,
        value: Vec<u8>,
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
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    pub struct discover_characteristics {
        result: u16,
    }

    impl FromBytes for discover_characteristics {
        fn from_bytes(data: &[u8]) -> discover_characteristics {
            let mut cursor = Cursor::new(data);
            discover_characteristics {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for discover_characteristics {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct discover_characteristics_by_uuid {
        result: u16,
    }

    impl FromBytes for discover_characteristics_by_uuid {
        fn from_bytes(data: &[u8]) -> discover_characteristics_by_uuid {
            let mut cursor = Cursor::new(data);
            discover_characteristics_by_uuid {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for discover_characteristics_by_uuid {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct discover_descriptors {
        result: u16,
    }

    impl FromBytes for discover_descriptors {
        fn from_bytes(data: &[u8]) -> discover_descriptors {
            let mut cursor = Cursor::new(data);
            discover_descriptors {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for discover_descriptors {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct discover_primary_services {
        result: u16,
    }

    impl FromBytes for discover_primary_services {
        fn from_bytes(data: &[u8]) -> discover_primary_services {
            let mut cursor = Cursor::new(data);
            discover_primary_services {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for discover_primary_services {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct discover_primary_services_by_uuid {
        result: u16,
    }

    impl FromBytes for discover_primary_services_by_uuid {
        fn from_bytes(data: &[u8]) -> discover_primary_services_by_uuid {
            let mut cursor = Cursor::new(data);
            discover_primary_services_by_uuid {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for discover_primary_services_by_uuid {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct execute_characteristic_value_write {
        result: u16,
    }

    impl FromBytes for execute_characteristic_value_write {
        fn from_bytes(data: &[u8]) -> execute_characteristic_value_write {
            let mut cursor = Cursor::new(data);
            execute_characteristic_value_write {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for execute_characteristic_value_write {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct find_included_services {
        result: u16,
    }

    impl FromBytes for find_included_services {
        fn from_bytes(data: &[u8]) -> find_included_services {
            let mut cursor = Cursor::new(data);
            find_included_services {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for find_included_services {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct prepare_characteristic_value_reliable_write {
        result: u16,
        sent_len: u16,
    }

    impl FromBytes for prepare_characteristic_value_reliable_write {
        fn from_bytes(data: &[u8]) -> prepare_characteristic_value_reliable_write {
            let mut cursor = Cursor::new(data);
            prepare_characteristic_value_reliable_write {
                result: cursor.get_u16_le(),
                sent_len: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for prepare_characteristic_value_reliable_write {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u16_le(self.sent_len);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct prepare_characteristic_value_write {
        result: u16,
        sent_len: u16,
    }

    impl FromBytes for prepare_characteristic_value_write {
        fn from_bytes(data: &[u8]) -> prepare_characteristic_value_write {
            let mut cursor = Cursor::new(data);
            prepare_characteristic_value_write {
                result: cursor.get_u16_le(),
                sent_len: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for prepare_characteristic_value_write {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u16_le(self.sent_len);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct read_characteristic_value {
        result: u16,
    }

    impl FromBytes for read_characteristic_value {
        fn from_bytes(data: &[u8]) -> read_characteristic_value {
            let mut cursor = Cursor::new(data);
            read_characteristic_value {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for read_characteristic_value {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct read_characteristic_value_by_uuid {
        result: u16,
    }

    impl FromBytes for read_characteristic_value_by_uuid {
        fn from_bytes(data: &[u8]) -> read_characteristic_value_by_uuid {
            let mut cursor = Cursor::new(data);
            read_characteristic_value_by_uuid {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for read_characteristic_value_by_uuid {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct read_characteristic_value_from_offset {
        result: u16,
    }

    impl FromBytes for read_characteristic_value_from_offset {
        fn from_bytes(data: &[u8]) -> read_characteristic_value_from_offset {
            let mut cursor = Cursor::new(data);
            read_characteristic_value_from_offset {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for read_characteristic_value_from_offset {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct read_descriptor_value {
        result: u16,
    }

    impl FromBytes for read_descriptor_value {
        fn from_bytes(data: &[u8]) -> read_descriptor_value {
            let mut cursor = Cursor::new(data);
            read_descriptor_value {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for read_descriptor_value {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct read_multiple_characteristic_values {
        result: u16,
    }

    impl FromBytes for read_multiple_characteristic_values {
        fn from_bytes(data: &[u8]) -> read_multiple_characteristic_values {
            let mut cursor = Cursor::new(data);
            read_multiple_characteristic_values {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for read_multiple_characteristic_values {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct send_characteristic_confirmation {
        result: u16,
    }

    impl FromBytes for send_characteristic_confirmation {
        fn from_bytes(data: &[u8]) -> send_characteristic_confirmation {
            let mut cursor = Cursor::new(data);
            send_characteristic_confirmation {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for send_characteristic_confirmation {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct set_characteristic_notification {
        result: u16,
    }

    impl FromBytes for set_characteristic_notification {
        fn from_bytes(data: &[u8]) -> set_characteristic_notification {
            let mut cursor = Cursor::new(data);
            set_characteristic_notification {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_characteristic_notification {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct set_max_mtu {
        result: u16,
        max_mtu: u16,
    }

    impl FromBytes for set_max_mtu {
        fn from_bytes(data: &[u8]) -> set_max_mtu {
            let mut cursor = Cursor::new(data);
            set_max_mtu {
                result: cursor.get_u16_le(),
                max_mtu: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_max_mtu {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u16_le(self.max_mtu);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct write_characteristic_value {
        result: u16,
    }

    impl FromBytes for write_characteristic_value {
        fn from_bytes(data: &[u8]) -> write_characteristic_value {
            let mut cursor = Cursor::new(data);
            write_characteristic_value {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for write_characteristic_value {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct write_characteristic_value_without_response {
        result: u16,
        sent_len: u16,
    }

    impl FromBytes for write_characteristic_value_without_response {
        fn from_bytes(data: &[u8]) -> write_characteristic_value_without_response {
            let mut cursor = Cursor::new(data);
            write_characteristic_value_without_response {
                result: cursor.get_u16_le(),
                sent_len: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for write_characteristic_value_without_response {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u16_le(self.sent_len);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct write_descriptor_value {
        result: u16,
    }

    impl FromBytes for write_descriptor_value {
        fn from_bytes(data: &[u8]) -> write_descriptor_value {
            let mut cursor = Cursor::new(data);
            write_descriptor_value {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for write_descriptor_value {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

}

pub mod evt {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct characteristic {
        connection: u8,
        characteristic: u16,
        properties: u8,
        uuid: [u8; 16],
    }

    impl FromBytes for characteristic {
        fn from_bytes(data: &[u8]) -> characteristic {
            let mut cursor = Cursor::new(data);
            let connection = cursor.get_u8();
            let characteristic = cursor.get_u16_le();
            let properties = cursor.get_u8();
            let mut uuid: [u8; 16] = Default::default();
            cursor.read_exact(&mut uuid).expect("Failed to read bytes.");
            characteristic {
                connection,
                characteristic,
                properties,
                uuid,
            }
        }
    }

    impl ToBytes for characteristic {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u16_le(self.characteristic);
            bytes.put_u8(self.properties);
            bytes.extend_from_slice(&self.uuid);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct characteristic_value {
        connection: u8,
        characteristic: u16,
        att_opcode: u8,
        offset: u16,
        value: Vec<u8>,
    }

    impl FromBytes for characteristic_value {
        fn from_bytes(data: &[u8]) -> characteristic_value {
            let mut cursor = Cursor::new(data);
            let connection = cursor.get_u8();
            let characteristic = cursor.get_u16_le();
            let att_opcode = cursor.get_u8();
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

    impl ToBytes for characteristic_value {
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

    #[allow(non_camel_case_types)]
    pub struct descriptor {
        connection: u8,
        descriptor: u16,
        uuid: [u8; 16],
    }

    impl FromBytes for descriptor {
        fn from_bytes(data: &[u8]) -> descriptor {
            let mut cursor = Cursor::new(data);
            let connection = cursor.get_u8();
            let descriptor = cursor.get_u16_le();
            let mut uuid: [u8; 16] = Default::default();
            cursor.read_exact(&mut uuid).expect("Failed to read bytes.");
            descriptor {
                connection,
                descriptor,
                uuid,
            }
        }
    }

    impl ToBytes for descriptor {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u16_le(self.descriptor);
            bytes.extend_from_slice(&self.uuid);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct descriptor_value {
        connection: u8,
        descriptor: u16,
        offset: u16,
        value: Vec<u8>,
    }

    impl FromBytes for descriptor_value {
        fn from_bytes(data: &[u8]) -> descriptor_value {
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

    impl ToBytes for descriptor_value {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u16_le(self.descriptor);
            bytes.put_u16_le(self.offset);
            bytes.extend(self.value.iter());
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct mtu_exchanged {
        connection: u8,
        mtu: u16,
    }

    impl FromBytes for mtu_exchanged {
        fn from_bytes(data: &[u8]) -> mtu_exchanged {
            let mut cursor = Cursor::new(data);
            mtu_exchanged {
                connection: cursor.get_u8(),
                mtu: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for mtu_exchanged {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u16_le(self.mtu);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct procedure_completed {
        connection: u8,
        result: u16,
    }

    impl FromBytes for procedure_completed {
        fn from_bytes(data: &[u8]) -> procedure_completed {
            let mut cursor = Cursor::new(data);
            procedure_completed {
                connection: cursor.get_u8(),
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for procedure_completed {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct service {
        connection: u8,
        service: u32,
        uuid: [u8; 16],
    }

    impl FromBytes for service {
        fn from_bytes(data: &[u8]) -> service {
            let mut cursor = Cursor::new(data);
            let connection = cursor.get_u8();
            let service = cursor.get_u32_le();
            let mut uuid: [u8; 16] = Default::default();
            cursor.read_exact(&mut uuid).expect("Failed to read bytes.");
            service {
                connection,
                service,
                uuid,
            }
        }
    }

    impl ToBytes for service {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u32_le(self.service);
            bytes.extend_from_slice(&self.uuid);
            bytes
        }
    }
}
