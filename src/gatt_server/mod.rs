pub mod cmd;
pub mod evt;
pub mod rsp;

use message::{MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt_server,
            message_id: 0x06,
        } => Ok(MessagePayload::rsp_gatt_server_find_attribute(
            rsp::find_attribute::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: _,
            message_class: MessageClass::gatt_server,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_gatt_server_read_attribute_type(
            rsp::read_attribute_type::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: _,
            message_class: MessageClass::gatt_server,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_gatt_server_read_attribute_value(
            rsp::read_attribute_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt_server,
            message_id: 0x05,
        } => Ok(
            MessagePayload::rsp_gatt_server_send_characteristic_notification(
                rsp::send_characteristic_notification::from(buffer),
            ),
        ),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt_server,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_gatt_server_send_user_read_response(
            rsp::send_user_read_response::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt_server,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_gatt_server_send_user_write_response(
            rsp::send_user_write_response::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt_server,
            message_id: 0x08,
        } => Ok(MessagePayload::rsp_gatt_server_set_capabilities(
            rsp::set_capabilities::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt_server,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_gatt_server_write_attribute_value(
            rsp::write_attribute_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x07,
            message_class: MessageClass::gatt_server,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_gatt_server_attribute_value(
            evt::attribute_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x06,
            message_class: MessageClass::gatt_server,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_gatt_server_characteristic_status(
            evt::characteristic_status::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::gatt_server,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_gatt_server_execute_write_completed(
            evt::execute_write_completed::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x06,
            message_class: MessageClass::gatt_server,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_gatt_server_user_read_request(
            evt::user_read_request::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::gatt_server,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_gatt_server_user_write_request(
            evt::user_write_request::from(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CharacteristicStatusFlag {
    client_config = 1, // Characteristic client configuration has been changed.
    confirmation = 2,  // Characteristic confirmation has been received.
}
