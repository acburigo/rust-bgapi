pub mod cmd;
pub mod rsp;

use message::{MessageClass, MessageHeader, MessagePayload, MessageType};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::coex,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_coex_get_counters(
            rsp::get_counters::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::coex,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_coex_set_options(
            rsp::set_options::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
pub enum Option {
    enable = 256,         // Enable coexistence feature
    tx_abort = 1024,      // Abort transmission if grant is denied
    high_priority = 2048, // Enable priority signal
}
