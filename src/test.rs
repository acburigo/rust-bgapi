use std::io::Cursor;
use std::io::{Error, ErrorKind};

use MessageHeader;

pub enum Command {
    dtm_end,
    dtm_rx {
        channel: u8,
        phy: u8,
    },
    dtm_tx {
        packet_type: u8,
        length: u8,
        channel: u8,
        phy: u8,
    },
}

pub enum Response {
    dtm_end { result: u16 },
    dtm_rx { result: u16 },
    dtm_tx { result: u16 },
}

impl Response {
    pub fn from_binary(
        cursor: &mut Cursor<&[u8]>,
        header: &MessageHeader,
    ) -> Result<Response, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}

pub enum Event {
    dtm_completed { result: u16, number_of_packets: u16 },
}

impl Event {
    pub fn from_binary(cursor: &mut Cursor<&[u8]>, header: &MessageHeader) -> Result<Event, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}

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

pub enum Phy {
    phy_1m = 1,   // 1M PHY
    phy_2m = 2,   // 2M PHY
    phy_125k = 3, // 125k Coded PHY
    phy_500k = 4, // 500k Coded PHY Bluetooth Software API Reference Manual API Reference silabs.
}
