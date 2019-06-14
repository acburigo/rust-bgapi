pub mod cmd;
pub mod evt;
pub mod rsp;

use message::{MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::dfu,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_dfu_flash_set_address(
            rsp::flash_set_address::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::dfu,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_dfu_flash_upload(
            rsp::flash_upload::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::dfu,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_dfu_flash_upload_finish(
            rsp::flash_upload_finish::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x04,
            message_class: MessageClass::dfu,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_dfu_boot(evt::boot::from(buffer))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::dfu,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_dfu_boot_failure(
            evt::boot_failure::from(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}
