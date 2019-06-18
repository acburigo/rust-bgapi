use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct message_to_target {
    pub data: Vec<u8>,
}

impl message_to_target {
    pub fn new(data: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: data.len() as u8,
            message_class: MessageClass::user,
            message_id: 0x00,
        };
        let payload = message_to_target { data };
        let payload = MessagePayload::cmd_user_message_to_target(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for message_to_target {
    fn from(data: &[u8]) -> message_to_target {
        let mut cursor = Cursor::new(data);
        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .expect("Failed to read bytes.");
        message_to_target { data }
    }
}

impl Into<Vec<u8>> for message_to_target {
    fn into(self) -> Vec<u8> {
        self.data.clone()
    }
}
