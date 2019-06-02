pub mod cmd;
pub mod evt;
pub mod rsp;

use message::{MessageClass, MessageHeader, MessagePayload, MessageType};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: _,
            message_class: MessageClass::user,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_user_message_to_target(
            rsp::message_to_target::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::user,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_user_message_to_host(
            evt::message_to_host::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}
