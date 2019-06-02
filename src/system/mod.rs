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
            payload_length: 0x06,
            message_class: MessageClass::system,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_system_get_bt_address(
            rsp::get_bt_address::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x0a,
            message_class: MessageClass::system,
            message_id: 0x0f,
        } => Ok(MessagePayload::rsp_system_get_counters(
            rsp::get_counters::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: _,
            message_class: MessageClass::system,
            message_id: 0x0b,
        } => Ok(MessagePayload::rsp_system_get_random_data(
            rsp::get_random_data::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x0c,
        } => Ok(MessagePayload::rsp_system_halt(rsp::halt::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_system_hello(rsp::hello::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_system_set_bt_address(
            rsp::set_bt_address::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x0d,
        } => Ok(MessagePayload::rsp_system_set_device_name(
            rsp::set_device_name::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x0a,
        } => Ok(MessagePayload::rsp_system_set_tx_power(
            rsp::set_tx_power::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x00,
            message_class: MessageClass::system,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_system_awake(evt::awake::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x12,
            message_class: MessageClass::system,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_system_boot(evt::boot::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::system,
            message_id: 0x06,
        } => Ok(MessagePayload::evt_system_error(evt::error::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x04,
            message_class: MessageClass::system,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_system_external_signal(
            evt::external_signal::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x05,
        } => Ok(MessagePayload::evt_system_hardware_error(
            evt::hardware_error::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}
