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
            message_class: MessageClass::gatt,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_gatt_discover_characteristics(
            rsp::discover_characteristics::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_gatt_discover_characteristics_by_uuid(
            rsp::discover_characteristics_by_uuid::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x06,
        } => Ok(MessagePayload::rsp_gatt_discover_descriptors(
            rsp::discover_descriptors::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_gatt_discover_primary_services(
            rsp::discover_primary_services::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_gatt_discover_primary_services_by_uuid(
            rsp::discover_primary_services_by_uuid::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x0c,
        } => Ok(MessagePayload::rsp_gatt_execute_characteristic_value_write(
            rsp::execute_characteristic_value_write::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x10,
        } => Ok(MessagePayload::rsp_gatt_find_included_services(
            rsp::find_included_services::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt,
            message_id: 0x13,
        } => Ok(
            MessagePayload::rsp_gatt_prepare_characteristic_value_reliable_write(
                rsp::prepare_characteristic_value_reliable_write::from(buffer),
            ),
        ),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt,
            message_id: 0x0b,
        } => Ok(MessagePayload::rsp_gatt_prepare_characteristic_value_write(
            rsp::prepare_characteristic_value_write::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x07,
        } => Ok(MessagePayload::rsp_gatt_read_characteristic_value(
            rsp::read_characteristic_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x08,
        } => Ok(MessagePayload::rsp_gatt_read_characteristic_value_by_uuid(
            rsp::read_characteristic_value_by_uuid::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x12,
        } => Ok(
            MessagePayload::rsp_gatt_read_characteristic_value_from_offset(
                rsp::read_characteristic_value_from_offset::from(buffer),
            ),
        ),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x0e,
        } => Ok(MessagePayload::rsp_gatt_read_descriptor_value(
            rsp::read_descriptor_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x11,
        } => Ok(
            MessagePayload::rsp_gatt_read_multiple_characteristic_values(
                rsp::read_multiple_characteristic_values::from(buffer),
            ),
        ),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x0d,
        } => Ok(MessagePayload::rsp_gatt_send_characteristic_confirmation(
            rsp::send_characteristic_confirmation::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x05,
        } => Ok(MessagePayload::rsp_gatt_set_characteristic_notification(
            rsp::set_characteristic_notification::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_gatt_set_max_mtu(
            rsp::set_max_mtu::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x09,
        } => Ok(MessagePayload::rsp_gatt_write_characteristic_value(
            rsp::write_characteristic_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::gatt,
            message_id: 0x0a,
        } => Ok(
            MessagePayload::rsp_gatt_write_characteristic_value_without_response(
                rsp::write_characteristic_value_without_response::from(buffer),
            ),
        ),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::gatt,
            message_id: 0x0f,
        } => Ok(MessagePayload::rsp_gatt_write_descriptor_value(
            rsp::write_descriptor_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::gatt,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_gatt_characteristic(
            evt::characteristic::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::gatt,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_gatt_characteristic_value(
            evt::characteristic_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::gatt,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_gatt_descriptor(evt::descriptor::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::gatt,
            message_id: 0x05,
        } => Ok(MessagePayload::evt_gatt_descriptor_value(
            evt::descriptor_value::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::gatt,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_gatt_mtu_exchanged(
            evt::mtu_exchanged::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::gatt,
            message_id: 0x06,
        } => Ok(MessagePayload::evt_gatt_procedure_completed(
            evt::procedure_completed::from(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd, Clone, FromPrimitive)]
pub enum AttOpcode {
    read_by_type_request = 8,       // Read by type request
    read_by_type_response = 9,      // Read by type response
    read_request = 10,              // Read request
    read_response = 11,             // Read response
    read_blob_request = 12,         // Read blob request
    read_blob_response = 13,        // Read blob response
    read_multiple_request = 14,     // Read multiple request
    read_multiple_response = 15,    // Read multiple response
    write_request = 18,             // Write request
    write_response = 19,            // Write response
    write_command = 82,             // Write command
    prepare_write_request = 22,     // Prepare write request
    prepare_write_response = 23,    // Prepare write response
    execute_write_request = 24,     // Execute write request
    execute_write_response = 25,    // Execute write response
    handle_value_notification = 27, // Notification
    handle_value_indication = 29,   // Indication
}

#[allow(non_camel_case_types)]
pub enum ClientConfigFlag {
    disable = 0,      // Disable notifications and indications
    notification = 1, // Notification
    indication = 2,   // Indication
}

#[allow(non_camel_case_types)]
pub enum execute_write_flag {
    cancel = 0, // Cancel all queued writes
    commit = 1, // Commit all queued writes
}
