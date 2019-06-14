pub mod cmd;
pub mod evt;
pub mod rsp;

use message::{MessageClass, MessageHeader, MessagePayload, MessageType};
use num_derive::FromPrimitive;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_le_connection_close(rsp::close::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_le_connection_disable_slave_latency(
            rsp::disable_slave_latency::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_le_connection_get_rssi(
            rsp::get_rssi::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_le_connection_set_parameters(
            rsp::set_parameters::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_le_connection_set_phy(
            rsp::set_phy::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::le_connection,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_le_connection_closed(evt::closed::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x0b,
            message_class: MessageClass::le_connection,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_le_connection_opened(evt::opened::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x0a,
            message_class: MessageClass::le_connection,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_le_connection_parameters(
            evt::parameters::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_le_connection_phy_status(
            evt::phy_status::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::le_connection,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_le_connection_rssi(evt::rssi::from(
            buffer,
        ))),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd, Clone, FromPrimitive)]
pub enum Security {
    mode1_level1 = 0, // No security
    mode1_level2 = 1, // Unauthenticated pairing with encryption
    mode1_level3 = 2, // Authenticated pairing with encryption
    mode1_level4 = 3, // Authenticated Secure Connections pairing with encryption using a 128-bit strength encryption key
}
