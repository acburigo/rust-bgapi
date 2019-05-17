use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: 0x20,
            payload_length: 0x04,
            message_class: 0x0a,
            message_id: 0x06,
        } => Ok(MessagePayload::rsp_gatt_server_find_attribute(
            rsp::find_attribute::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: _,
            message_class: 0x0a,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_gatt_server_read_attribute_type(
            rsp::read_attribute_type::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: _,
            message_class: 0x0a,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_gatt_server_read_attribute_value(
            rsp::read_attribute_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x04,
            message_class: 0x0a,
            message_id: 0x05,
        } => Ok(
            MessagePayload::rsp_gatt_server_send_characteristic_notification(
                rsp::send_characteristic_notification::from_bytes(buffer),
            ),
        ),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x04,
            message_class: 0x0a,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_gatt_server_send_user_read_response(
            rsp::send_user_read_response::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x0a,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_gatt_server_send_user_write_response(
            rsp::send_user_write_response::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x0a,
            message_id: 0x08,
        } => Ok(MessagePayload::rsp_gatt_server_set_capabilities(
            rsp::set_capabilities::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x0a,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_gatt_server_write_attribute_value(
            rsp::write_attribute_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x07,
            message_class: 0x0a,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_gatt_server_attribute_value(
            evt::attribute_value::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x06,
            message_class: 0x0a,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_gatt_server_characteristic_status(
            evt::characteristic_status::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x03,
            message_class: 0x0a,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_gatt_server_execute_write_completed(
            evt::execute_write_completed::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x06,
            message_class: 0x0a,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_gatt_server_user_read_request(
            evt::user_read_request::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: _,
            message_class: 0x0a,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_gatt_server_user_write_request(
            evt::user_write_request::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
pub enum CharacteristicStatusFlag {
    client_config = 1, // Characteristic client configuration has been changed.
    confirmation = 2,  // Characteristic confirmation has been received.
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct find_attribute {
        start: u16,
        atype: Vec<u8>,
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
        attribute: u16,
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
        attribute: u16,
        offset: u16,
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
        connection: u8,
        characteristic: u16,
        value: Vec<u8>,
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
        connection: u8,
        characteristic: u16,
        att_errorcode: u8,
        value: Vec<u8>,
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
        connection: u8,
        characteristic: u16,
        att_errorcode: u8,
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
        caps: u32,
        reserved: u32,
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
        attribute: u16,
        offset: u16,
        value: Vec<u8>,
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
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct find_attribute {
        result: u16,
        attribute: u16,
    }

    impl FromBytes for find_attribute {
        fn from_bytes(data: &[u8]) -> find_attribute {
            let mut cursor = Cursor::new(data);
            find_attribute {
                result: cursor.get_u16_le(),
                attribute: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for find_attribute {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u16_le(self.attribute);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct read_attribute_type {
        result: u16,
        atype: Vec<u8>,
    }

    impl FromBytes for read_attribute_type {
        fn from_bytes(data: &[u8]) -> read_attribute_type {
            let mut cursor = Cursor::new(data);
            let result = cursor.get_u16_le();
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
            bytes.put_u16_le(self.result);
            bytes.extend(self.atype.iter());
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct read_attribute_value {
        result: u16,
        value: Vec<u8>,
    }

    impl FromBytes for read_attribute_value {
        fn from_bytes(data: &[u8]) -> read_attribute_value {
            let mut cursor = Cursor::new(data);
            let result = cursor.get_u16_le();
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
            bytes.put_u16_le(self.result);
            bytes.extend(self.value.iter());
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct send_characteristic_notification {
        result: u16,
        sent_len: u16,
    }

    impl FromBytes for send_characteristic_notification {
        fn from_bytes(data: &[u8]) -> send_characteristic_notification {
            let mut cursor = Cursor::new(data);
            send_characteristic_notification {
                result: cursor.get_u16_le(),
                sent_len: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for send_characteristic_notification {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u16_le(self.sent_len);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct send_user_read_response {
        result: u16,
        sent_len: u16,
    }

    impl FromBytes for send_user_read_response {
        fn from_bytes(data: &[u8]) -> send_user_read_response {
            let mut cursor = Cursor::new(data);
            send_user_read_response {
                result: cursor.get_u16_le(),
                sent_len: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for send_user_read_response {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u16_le(self.sent_len);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct send_user_write_response {
        result: u16,
    }

    impl FromBytes for send_user_write_response {
        fn from_bytes(data: &[u8]) -> send_user_write_response {
            let mut cursor = Cursor::new(data);
            send_user_write_response {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for send_user_write_response {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_capabilities {
        result: u16,
    }

    impl FromBytes for set_capabilities {
        fn from_bytes(data: &[u8]) -> set_capabilities {
            let mut cursor = Cursor::new(data);
            set_capabilities {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_capabilities {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct write_attribute_value {
        result: u16,
    }

    impl FromBytes for write_attribute_value {
        fn from_bytes(data: &[u8]) -> write_attribute_value {
            let mut cursor = Cursor::new(data);
            write_attribute_value {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for write_attribute_value {
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
    #[derive(PartialEq, PartialOrd)]
    pub struct attribute_value {
        connection: u8,
        attribute: u16,
        att_opcode: u8,
        offset: u16,
        value: Vec<u8>,
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
        connection: u8,
        characteristic: u16,
        status_flags: u8,
        client_config_flags: u16,
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
        connection: u8,
        result: u16,
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
        connection: u8,
        characteristic: u16,
        att_opcode: u8,
        offset: u16,
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
        connection: u8,
        characteristic: u16,
        att_opcode: u8,
        offset: u16,
        value: Vec<u8>,
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
}
