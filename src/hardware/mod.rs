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
            payload_length: 0x02,
            message_class: MessageClass::hardware,
            message_id: 0x0c,
        } => Ok(MessagePayload::rsp_hardware_set_lazy_soft_timer(
            rsp::set_lazy_soft_timer::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::hardware,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_hardware_set_soft_timer(
            rsp::set_soft_timer::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x01,
            message_class: MessageClass::hardware,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_hardware_soft_timer(
            evt::soft_timer::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}
