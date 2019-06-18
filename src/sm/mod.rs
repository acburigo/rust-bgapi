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
            message_class: MessageClass::sm,
            message_id: 0x0e,
        } => Ok(MessagePayload::rsp_sm_bonding_confirm(
            rsp::bonding_confirm::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_sm_configure(rsp::configure::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x06,
        } => Ok(MessagePayload::rsp_sm_delete_bonding(
            rsp::delete_bonding::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x07,
        } => Ok(MessagePayload::rsp_sm_delete_bondings(
            rsp::delete_bondings::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x08,
        } => Ok(MessagePayload::rsp_sm_enter_passkey(
            rsp::enter_passkey::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_sm_increase_security(
            rsp::increase_security::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x0b,
        } => Ok(MessagePayload::rsp_sm_list_all_bondings(
            rsp::list_all_bondings::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x09,
        } => Ok(MessagePayload::rsp_sm_passkey_confirm(
            rsp::passkey_confirm::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_sm_set_bondable_mode(
            rsp::set_bondable_mode::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x0f,
        } => Ok(MessagePayload::rsp_sm_set_debug_mode(
            rsp::set_debug_mode::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x0a,
        } => Ok(MessagePayload::rsp_sm_set_oob_data(
            rsp::set_oob_data::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x10,
        } => Ok(MessagePayload::rsp_sm_set_passkey(rsp::set_passkey::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x12,
        } => Ok(MessagePayload::rsp_sm_set_sc_remote_oob_data(
            rsp::set_sc_remote_oob_data::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_sm_store_bonding_configuration(
            rsp::store_bonding_configuration::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: _,
            message_class: MessageClass::sm,
            message_id: 0x11,
        } => Ok(MessagePayload::rsp_sm_use_sc_oob(rsp::use_sc_oob::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_sm_bonded(evt::bonded::from(buffer))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::sm,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_sm_bonding_failed(
            evt::bonding_failed::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x09,
        } => Ok(MessagePayload::evt_sm_confirm_bonding(
            evt::confirm_bonding::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x05,
            message_class: MessageClass::sm,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_sm_confirm_passkey(
            evt::confirm_passkey::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x00,
            message_class: MessageClass::sm,
            message_id: 0x06,
        } => Ok(MessagePayload::evt_sm_list_all_bondings_complete(
            evt::list_all_bondings_complete::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x08,
            message_class: MessageClass::sm,
            message_id: 0x05,
        } => Ok(MessagePayload::evt_sm_list_bonding_entry(
            evt::list_bonding_entry::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x05,
            message_class: MessageClass::sm,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_sm_passkey_display(
            evt::passkey_display::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x01,
            message_class: MessageClass::sm,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_sm_passkey_request(
            evt::passkey_request::from(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum BondingKey {
    ltk = 1,         // LTK saved in master
    addr_public = 2, // Public Address
    addr_static = 4, // Static Address
    irk = 8,         // Identity resolving key for resolvable private addresses
    edivrand = 16,   // EDIV+RAND received from slave
    csrk = 32,       // Connection signature resolving key
    masterid = 64,   // EDIV+RAND sent to master
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum io_capability {
    displayonly = 0,     // Display Only
    displayyesno = 1,    // Display with Yes/No-buttons
    keyboardonly = 2,    // Keyboard Only
    noinputnooutput = 3, // No Input and No Output
    keyboarddisplay = 4, // Display with Keyboard
}
