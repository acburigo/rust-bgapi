use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct find_attribute {
    pub start: u16,
    pub atype: Vec<u8>,
}

impl find_attribute {
    pub fn new(start: u16, atype: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02 + (atype.len() as u8),
            message_class: MessageClass::gatt_server,
            message_id: 0x06,
        };
        let payload = find_attribute { start, atype };
        let payload = MessagePayload::cmd_gatt_server_find_attribute(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for find_attribute {
    fn from(data: &[u8]) -> find_attribute {
        let mut cursor = Cursor::new(data);
        let start = cursor.get_u16_le();
        let mut atype = Vec::new();
        cursor
            .read_to_end(&mut atype)
            .expect("Failed to read bytes.");
        find_attribute { start, atype }
    }
}

impl Into<Vec<u8>> for find_attribute {
    fn into(self) -> Vec<u8> {
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

impl read_attribute_type {
    pub fn new(attribute: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt_server,
            message_id: 0x01,
        };
        let payload = read_attribute_type { attribute };
        let payload = MessagePayload::cmd_gatt_server_read_attribute_type(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for read_attribute_type {
    fn from(data: &[u8]) -> read_attribute_type {
        let mut cursor = Cursor::new(data);
        read_attribute_type {
            attribute: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for read_attribute_type {
    fn into(self) -> Vec<u8> {
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

impl read_attribute_value {
    pub fn new(attribute: u16, offset: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt_server,
            message_id: 0x00,
        };
        let payload = read_attribute_value { attribute, offset };
        let payload = MessagePayload::cmd_gatt_server_read_attribute_value(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for read_attribute_value {
    fn from(data: &[u8]) -> read_attribute_value {
        let mut cursor = Cursor::new(data);
        read_attribute_value {
            attribute: cursor.get_u16_le(),
            offset: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for read_attribute_value {
    fn into(self) -> Vec<u8> {
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

impl send_characteristic_notification {
    pub fn new(connection: u8, characteristic: u16, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03 + (value.len() as u8),
            message_class: MessageClass::gatt_server,
            message_id: 0x05,
        };
        let payload = send_characteristic_notification {
            connection,
            characteristic,
            value,
        };
        let payload = MessagePayload::cmd_gatt_server_send_characteristic_notification(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for send_characteristic_notification {
    fn from(data: &[u8]) -> send_characteristic_notification {
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

impl Into<Vec<u8>> for send_characteristic_notification {
    fn into(self) -> Vec<u8> {
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

impl send_user_read_response {
    pub fn new(connection: u8, characteristic: u16, att_errorcode: u8, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04 + (value.len() as u8),
            message_class: MessageClass::gatt_server,
            message_id: 0x03,
        };
        let payload = send_user_read_response {
            connection,
            characteristic,
            att_errorcode,
            value,
        };
        let payload = MessagePayload::cmd_gatt_server_send_user_read_response(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for send_user_read_response {
    fn from(data: &[u8]) -> send_user_read_response {
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

impl Into<Vec<u8>> for send_user_read_response {
    fn into(self) -> Vec<u8> {
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

impl send_user_write_response {
    pub fn new(connection: u8, characteristic: u16, att_errorcode: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt_server,
            message_id: 0x04,
        };
        let payload = send_user_write_response {
            connection,
            characteristic,
            att_errorcode,
        };
        let payload = MessagePayload::cmd_gatt_server_send_user_write_response(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for send_user_write_response {
    fn from(data: &[u8]) -> send_user_write_response {
        let mut cursor = Cursor::new(data);
        send_user_write_response {
            connection: cursor.get_u8(),
            characteristic: cursor.get_u16_le(),
            att_errorcode: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for send_user_write_response {
    fn into(self) -> Vec<u8> {
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

impl set_capabilities {
    pub fn new(caps: u32, reserved: u32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x08,
            message_class: MessageClass::gatt_server,
            message_id: 0x08,
        };
        let payload = set_capabilities { caps, reserved };
        let payload = MessagePayload::cmd_gatt_server_set_capabilities(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_capabilities {
    fn from(data: &[u8]) -> set_capabilities {
        let mut cursor = Cursor::new(data);
        set_capabilities {
            caps: cursor.get_u32_le(),
            reserved: cursor.get_u32_le(),
        }
    }
}

impl Into<Vec<u8>> for set_capabilities {
    fn into(self) -> Vec<u8> {
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

impl write_attribute_value {
    pub fn new(attribute: u16, offset: u16, value: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04 + (value.len() as u8),
            message_class: MessageClass::gatt_server,
            message_id: 0x02,
        };
        let payload = write_attribute_value {
            attribute,
            offset,
            value,
        };
        let payload = MessagePayload::cmd_gatt_server_write_attribute_value(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for write_attribute_value {
    fn from(data: &[u8]) -> write_attribute_value {
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

impl Into<Vec<u8>> for write_attribute_value {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.attribute);
        bytes.put_u16_le(self.offset);
        bytes.extend(self.value.iter());
        bytes
    }
}
