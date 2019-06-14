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
            message_class: MessageClass::le_gap,
            message_id: 0x0c,
        } => Ok(MessagePayload::rsp_le_gap_bt5_set_adv_data(
            rsp::bt5_set_adv_data::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x13,
        } => Ok(MessagePayload::rsp_le_gap_clear_advertise_configuration(
            rsp::clear_advertise_configuration::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::le_gap,
            message_id: 0x1a,
        } => Ok(MessagePayload::rsp_le_gap_connect(rsp::connect::from(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_le_gap_end_procedure(
            rsp::end_procedure::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x0f,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_channel_map(
            rsp::set_advertise_channel_map::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x12,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_configuration(
            rsp::set_advertise_configuration::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x11,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_phy(
            rsp::set_advertise_phy::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x10,
        } => Ok(
            MessagePayload::rsp_le_gap_set_advertise_report_scan_request(
                rsp::set_advertise_report_scan_request::from(buffer),
            ),
        ),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x0e,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_timing(
            rsp::set_advertise_timing::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x1b,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_tx_power(
            rsp::set_advertise_tx_power::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x05,
        } => Ok(MessagePayload::rsp_le_gap_set_conn_parameters(
            rsp::set_conn_parameters::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x19,
        } => Ok(MessagePayload::rsp_le_gap_set_data_channel_classification(
            rsp::set_data_channel_classification::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x16,
        } => Ok(MessagePayload::rsp_le_gap_set_discovery_timing(
            rsp::set_discovery_timing::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x17,
        } => Ok(MessagePayload::rsp_le_gap_set_discovery_type(
            rsp::set_discovery_type::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x0d,
        } => Ok(MessagePayload::rsp_le_gap_set_privacy_mode(
            rsp::set_privacy_mode::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x14,
        } => Ok(MessagePayload::rsp_le_gap_start_advertising(
            rsp::start_advertising::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x18,
        } => Ok(MessagePayload::rsp_le_gap_start_discovery(
            rsp::start_discovery::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x15,
        } => Ok(MessagePayload::rsp_le_gap_stop_advertising(
            rsp::stop_advertising::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x01,
            message_class: MessageClass::le_gap,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_le_gap_adv_timeout(
            evt::adv_timeout::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x09,
            message_class: MessageClass::le_gap,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_le_gap_scan_request(
            evt::scan_request::from(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::le_gap,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_le_gap_scan_response(
            evt::scan_response::from(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum AddressType {
    public = 0,          // Public address
    random = 1,          // Random address
    public_identity = 2, // Public identity address resolved by stack
    random_identity = 3, // Random identity address resolved by stack
}

#[allow(non_camel_case_types)]
#[derive(Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum AdvAddressType {
    identity_address = 0, // Use public or static device address, or identity address if privacy mode is enabled
    non_resolvable = 1, // Use non resolvable address type, advertising mode must also be non-connectable
}

#[allow(non_camel_case_types)]
#[derive(Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum ConnectableMode {
    non_connectable = 0,           // Non-connectable non-scannable.
    directed_connectable = 1,      // Directed connectable (RESERVED, DO NOT USE)
    connectable_scannable = 2, // Undirected connectable scannable. This mode can only be used in legacy advertising PDUs.
    scannable_non_connectable = 3, // Undirected scannable (Non-connectable but responds to scan requests)
    connectable_non_scannable = 4, // Undirected connectable non-scannable. This mode can only be used in extended advertising PDUs.
}

#[allow(non_camel_case_types)]
#[derive(Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum DiscoverMode {
    limited = 0,     // Discover only limited discoverable devices
    generic = 1,     // Discover limited and generic discoverable devices
    observation = 2, // Discover all devices
}

#[allow(non_camel_case_types)]
#[derive(Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum DiscoverableMode {
    non_discoverable = 0,     // Not discoverable
    limited_discoverable = 1, // Discoverable using both limited and general discovery procedures
    general_discoverable = 2, // Discoverable using general discovery procedure
    broadcast = 3, // Device is not discoverable in either limited or generic discovery procedure, but may be discovered by using the Observation procedure
    user_data = 4, // Send advertising and/or scan response data defined by the user using le_gap_bt5_set_adv_data. The limited/general discoverable flags are defined by the user.
}

#[allow(non_camel_case_types)]
#[derive(Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum PhyType {
    phy_1m = 1,    // LE 1M PHY
    phy_2m = 2,    // LE 2M PHY
    phy_coded = 4, // LE Coded PHY
}
