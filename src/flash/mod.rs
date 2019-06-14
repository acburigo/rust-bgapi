pub mod cmd;
pub mod rsp;

use message::{MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::flash,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_flash_ps_erase(rsp::ps_erase::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::flash,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_flash_ps_erase_all(
            rsp::ps_erase_all::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: _,
            message_class: MessageClass::flash,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_flash_ps_load(rsp::ps_load::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::flash,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_flash_ps_save(rsp::ps_save::from(
            buffer,
        ))),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}
