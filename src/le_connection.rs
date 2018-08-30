use std::io::Cursor;
use std::io::{Error, ErrorKind};

use MessageHeader;

pub enum Command {
    close {
        connection: u8,
    },
    disable_slave_latency {
        connection: u8,
        disable: u8,
    },
    get_rssi {
        connection: u8,
    },
    set_parameters {
        connection: u8,
        min_interval: u16,
        max_interval: u16,
        latency: u16,
        timeout: u16,
    },
    set_phy {
        connection: u8,
        phy: u8,
    },
}

pub enum Response {
    close { result: u16 },
    disable_slave_latency { result: u16 },
    get_rssi { result: u16 },
    set_parameters { result: u16 },
    set_phy { result: u16 },
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
    closed {
        reason: u16,
        connection: u8,
    },
    opened {
        address: [u8; 6],
        address_type: u8,
        master: u8,
        connection: u8,
        bonding: u8,
        advertiser: u8,
    },
    parameters {
        connection: u8,
        interval: u16,
        latency: u16,
        timeout: u16,
        security_mode: u8,
        txsize: u16,
    },
    phy_status {
        connection: u8,
        phy: u8,
    },
    rssi {
        connection: u8,
        status: u8,
        rssi: i8,
    },
}

impl Event {
    pub fn from_binary(cursor: &mut Cursor<&[u8]>, header: &MessageHeader) -> Result<Event, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}

pub enum Security {
    mode1_level1 = 0, // No security
    mode1_level2 = 1, // Unauthenticated pairing with encryption
    mode1_level3 = 2, // Authenticated pairing with encryption
    mode1_level4 = 3, // Authenticated Secure Connections pairing with encryption using a 128-bit strength encryption key
}
