use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct get_bt_address {}

    #[allow(non_camel_case_types)]
    pub struct get_counters {
        reset: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct get_random_data {
        length: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct halt {
        halt: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct hello {}

    #[allow(non_camel_case_types)]
    pub struct reset {
        dfu: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct set_bt_address {
        address: [u8; 6],
    }

    #[allow(non_camel_case_types)]
    pub struct set_device_name {
        dtype: u8,
        name: Box<[u8]>,
    }

    #[allow(non_camel_case_types)]
    pub struct set_tx_power {
        power: i16,
    }
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct get_bt_address {
        address: [u8; 6],
    }

    #[allow(non_camel_case_types)]
    pub struct get_counters {
        result: u16,
        tx_packets: u16,
        rx_packets: u16,
        crc_errors: u16,
        failures: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct get_random_data {
        result: u16,
        data: Box<[u8]>,
    }

    #[allow(non_camel_case_types)]
    pub struct halt {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct hello {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct set_bt_address {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct set_device_name {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct set_tx_power {
        set_power: i16,
    }
}

pub mod evt {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct awake {}

    #[allow(non_camel_case_types)]
    pub struct boot {
        major: u16,
        minor: u16,
        patch: u16,
        build: u16,
        bootloader: u32,
        hw: u16,
        hash: u32,
    }

    #[allow(non_camel_case_types)]
    pub struct error {
        reason: u16,
        data: Box<[u8]>,
    }

    #[allow(non_camel_case_types)]
    pub struct external_signal {
        extsignals: u32,
    }

    #[allow(non_camel_case_types)]
    pub struct hardware_error {
        status: u16,
    }
}
