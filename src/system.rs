use std::io::Cursor;
use std::io::{Error, ErrorKind};

use MessageHeader;

pub enum Command {
    get_bt_address,
    get_counters { reset: u8 },
    get_random_data { length: u8 },
    halt { halt: u8 },
    hello,
    reset { dfu: u8 },
    set_bt_address { address: [u8; 6] },
    set_device_name { dtype: u8, name: Box<[u8]> },
    set_tx_power { power: i16 },
}

pub enum Response {
    get_bt_address {
        address: [u8; 6],
    },
    get_counters {
        result: u16,
        tx_packets: u16,
        rx_packets: u16,
        crc_errors: u16,
        failures: u16,
    },
    get_random_data {
        result: u16,
        data: Box<[u8]>,
    },
    halt {
        result: u16,
    },
    hello {
        result: u16,
    },
    set_bt_address {
        result: u16,
    },
    set_device_name {
        result: u16,
    },
    set_tx_power {
        set_power: i16,
    },
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
    awake,
    boot {
        major: u16,
        minor: u16,
        patch: u16,
        build: u16,
        bootloader: u32,
        hw: u16,
        hash: u32,
    },
    error {
        reason: u16,
        data: Box<[u8]>,
    },
    external_signal {
        extsignals: u32,
    },
    hardware_error {
        status: u16,
    },
}

impl Event {
    pub fn from_binary(cursor: &mut Cursor<&[u8]>, header: &MessageHeader) -> Result<Event, Error> {
        Err(Error::from(ErrorKind::Other))
    }
}
