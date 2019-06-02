use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct bonding_confirm {
    pub connection: u8,
    pub confirm: u8,
}

impl FromBytes for bonding_confirm {
    fn from_bytes(data: &[u8]) -> bonding_confirm {
        let mut cursor = Cursor::new(data);
        bonding_confirm {
            connection: cursor.get_u8(),
            confirm: cursor.get_u8(),
        }
    }
}

impl ToBytes for bonding_confirm {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.confirm);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct configure {
    pub flags: u8,
    pub io_capabilities: u8,
}

impl FromBytes for configure {
    fn from_bytes(data: &[u8]) -> configure {
        let mut cursor = Cursor::new(data);
        configure {
            flags: cursor.get_u8(),
            io_capabilities: cursor.get_u8(),
        }
    }
}

impl ToBytes for configure {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.flags);
        bytes.put_u8(self.io_capabilities);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct delete_bonding {
    pub bonding: u8,
}

impl FromBytes for delete_bonding {
    fn from_bytes(data: &[u8]) -> delete_bonding {
        let mut cursor = Cursor::new(data);
        delete_bonding {
            bonding: cursor.get_u8(),
        }
    }
}

impl ToBytes for delete_bonding {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.bonding);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct delete_bondings {}

impl FromBytes for delete_bondings {
    fn from_bytes(_: &[u8]) -> delete_bondings {
        delete_bondings {}
    }
}

impl ToBytes for delete_bondings {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct enter_passkey {
    pub connection: u8,
    pub passkey: i32,
}

impl FromBytes for enter_passkey {
    fn from_bytes(data: &[u8]) -> enter_passkey {
        let mut cursor = Cursor::new(data);
        enter_passkey {
            connection: cursor.get_u8(),
            passkey: cursor.get_i32_le(),
        }
    }
}

impl ToBytes for enter_passkey {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_i32_le(self.passkey);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct increase_security {
    pub connection: u8,
}

impl FromBytes for increase_security {
    fn from_bytes(data: &[u8]) -> increase_security {
        let mut cursor = Cursor::new(data);
        increase_security {
            connection: cursor.get_u8(),
        }
    }
}

impl ToBytes for increase_security {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct list_all_bondings {}

impl FromBytes for list_all_bondings {
    fn from_bytes(_: &[u8]) -> list_all_bondings {
        list_all_bondings {}
    }
}

impl ToBytes for list_all_bondings {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct passkey_confirm {
    pub connection: u8,
    pub confirm: u8,
}

impl FromBytes for passkey_confirm {
    fn from_bytes(data: &[u8]) -> passkey_confirm {
        let mut cursor = Cursor::new(data);
        passkey_confirm {
            connection: cursor.get_u8(),
            confirm: cursor.get_u8(),
        }
    }
}

impl ToBytes for passkey_confirm {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.confirm);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_bondable_mode {
    pub bondable: u8,
}

impl FromBytes for set_bondable_mode {
    fn from_bytes(data: &[u8]) -> set_bondable_mode {
        let mut cursor = Cursor::new(data);
        set_bondable_mode {
            bondable: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_bondable_mode {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.bondable);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_debug_mode {}

impl FromBytes for set_debug_mode {
    fn from_bytes(_: &[u8]) -> set_debug_mode {
        set_debug_mode {}
    }
}

impl ToBytes for set_debug_mode {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_oob_data {
    pub oob_data: Vec<u8>,
}

impl FromBytes for set_oob_data {
    fn from_bytes(data: &[u8]) -> set_oob_data {
        let mut cursor = Cursor::new(data);
        let mut oob_data = Vec::new();
        cursor
            .read_to_end(&mut oob_data)
            .expect("Failed to read bytes.");
        set_oob_data { oob_data }
    }
}

impl ToBytes for set_oob_data {
    fn to_bytes(&self) -> Vec<u8> {
        self.oob_data.clone()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_passkey {
    pub passkey: i32,
}

impl FromBytes for set_passkey {
    fn from_bytes(data: &[u8]) -> set_passkey {
        let mut cursor = Cursor::new(data);
        set_passkey {
            passkey: cursor.get_i32_le(),
        }
    }
}

impl ToBytes for set_passkey {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_i32_le(self.passkey);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_sc_remote_oob_data {
    pub oob_data: Vec<u8>,
}

impl FromBytes for set_sc_remote_oob_data {
    fn from_bytes(data: &[u8]) -> set_sc_remote_oob_data {
        let mut cursor = Cursor::new(data);
        let mut oob_data = Vec::new();
        cursor
            .read_to_end(&mut oob_data)
            .expect("Failed to read bytes.");
        set_sc_remote_oob_data { oob_data }
    }
}

impl ToBytes for set_sc_remote_oob_data {
    fn to_bytes(&self) -> Vec<u8> {
        self.oob_data.clone()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct store_bonding_configuration {
    pub max_bonding_count: u8,
    pub policy_flags: u8,
}

impl FromBytes for store_bonding_configuration {
    fn from_bytes(data: &[u8]) -> store_bonding_configuration {
        let mut cursor = Cursor::new(data);
        store_bonding_configuration {
            max_bonding_count: cursor.get_u8(),
            policy_flags: cursor.get_u8(),
        }
    }
}

impl ToBytes for store_bonding_configuration {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.max_bonding_count);
        bytes.put_u8(self.policy_flags);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct use_sc_oob {
    pub enable: u8,
}

impl FromBytes for use_sc_oob {
    fn from_bytes(data: &[u8]) -> use_sc_oob {
        let mut cursor = Cursor::new(data);
        use_sc_oob {
            enable: cursor.get_u8(),
        }
    }
}

impl ToBytes for use_sc_oob {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.enable);
        bytes
    }
}
