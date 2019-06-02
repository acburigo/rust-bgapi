pub mod cmd;
pub mod evt;
pub mod rsp;

use message::{MessageClass, MessageHeader, MessagePayload, MessageType};
use num_derive::FromPrimitive;
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::test,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_test_dtm_end(rsp::dtm_end::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::test,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_test_dtm_rx(rsp::dtm_rx::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::test,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_test_dtm_tx(rsp::dtm_tx::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x04,
            message_class: MessageClass::test,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_test_dtm_completed(
            evt::dtm_completed::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum PacketType {
    pkt_prbs9 = 0,              // PRBS9 packet payload
    pkt_11110000 = 1,           // 11110000 packet payload
    pkt_10101010 = 2,           // 10101010 packet payload
    pkt_carrier_deprecated = 3, // Unmodulated carrier - deprecated
    pkt_11111111 = 4,           // 11111111 packet payload
    pkt_00000000 = 5,           // 00000000 packet payload
    pkt_00001111 = 6,           // 00001111 packet payload
    pkt_01010101 = 7,           // 01010101 packet payload
    pkt_pn9 = 253,              // PN9 continuously modulated output
    pkt_carrier = 254,          // Unmodulated carrier
}

#[allow(non_camel_case_types)]
#[derive(Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum Phy {
    phy_1m = 1,   // 1M PHY
    phy_2m = 2,   // 2M PHY
    phy_125k = 3, // 125k Coded PHY
    phy_500k = 4, // 500k Coded PHY Bluetooth Software API Reference Manual API Reference silabs.
}
