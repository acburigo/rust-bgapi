use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
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
pub enum io_capability {
    displayonly = 0,     // Display Only
    displayyesno = 1,    // Display with Yes/No-buttons
    keyboardonly = 2,    // Keyboard Only
    noinputnooutput = 3, // No Input and No Output
    keyboarddisplay = 4, // Display with Keyboard
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct bonding_confirm {
        connection: u8,
        confirm: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct configure {
        flags: u8,
        io_capabilities: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct delete_bonding {
        bonding: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct delete_bondings {}

    #[allow(non_camel_case_types)]
    pub struct enter_passkey {
        connection: u8,
        passkey: i32,
    }

    #[allow(non_camel_case_types)]
    pub struct increase_security {
        connection: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct list_all_bondings {}

    #[allow(non_camel_case_types)]
    pub struct passkey_confirm {
        connection: u8,
        confirm: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct set_bondable_mode {
        bondable: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct set_debug_mode {}

    #[allow(non_camel_case_types)]
    pub struct set_oob_data {
        oob_data: Box<[u8]>,
    }

    #[allow(non_camel_case_types)]
    pub struct set_passkey {
        passkey: i32,
    }

    #[allow(non_camel_case_types)]
    pub struct set_sc_remote_oob_data {
        oob_data: Box<[u8]>,
    }

    #[allow(non_camel_case_types)]
    pub struct store_bonding_configuration {
        max_bonding_count: u8,
        policy_flags: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct use_sc_oob {
        enable: u8,
    }
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct bonding_confirm {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct configure {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct delete_bonding {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct delete_bondings {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct enter_passkey {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct increase_security {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct list_all_bondings {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct passkey_confirm {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct set_bondable_mode {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct set_debug_mode {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct set_oob_data {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct set_passkey {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct set_sc_remote_oob_data {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct store_bonding_configuration {
        result: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct use_sc_oob {
        result: u16,
        oob_data: [u8; 32],
    }
}

pub mod evt {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct bonded {
        connection: u8,
        bonding: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct bonding_failed {
        connection: u8,
        reason: u16,
    }

    #[allow(non_camel_case_types)]
    pub struct confirm_bonding {
        connection: u8,
        bonding_handle: i8,
    }

    #[allow(non_camel_case_types)]
    pub struct confirm_passkey {
        connection: u8,
        passkey: u32,
    }

    pub struct list_all_bondings_complete {}

    #[allow(non_camel_case_types)]
    pub struct list_bonding_entry {
        bonding: u8,
        address: [u8; 6],
        address_type: u8,
    }

    #[allow(non_camel_case_types)]
    pub struct passkey_display {
        connection: u8,
        passkey: u32,
    }

    #[allow(non_camel_case_types)]
    pub struct passkey_request {
        connection: u8,
    }
}
