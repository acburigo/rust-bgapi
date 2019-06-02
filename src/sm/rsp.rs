use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct bonding_confirm {
    pub result: u16,
}

impl FromBytes for bonding_confirm {
    fn from_bytes(data: &[u8]) -> bonding_confirm {
        let mut cursor = Cursor::new(data);
        bonding_confirm {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for bonding_confirm {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct configure {
    pub result: u16,
}

impl FromBytes for configure {
    fn from_bytes(data: &[u8]) -> configure {
        let mut cursor = Cursor::new(data);
        configure {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for configure {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct delete_bonding {
    pub result: u16,
}

impl FromBytes for delete_bonding {
    fn from_bytes(data: &[u8]) -> delete_bonding {
        let mut cursor = Cursor::new(data);
        delete_bonding {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for delete_bonding {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct delete_bondings {
    pub result: u16,
}

impl FromBytes for delete_bondings {
    fn from_bytes(data: &[u8]) -> delete_bondings {
        let mut cursor = Cursor::new(data);
        delete_bondings {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for delete_bondings {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct enter_passkey {
    pub result: u16,
}

impl FromBytes for enter_passkey {
    fn from_bytes(data: &[u8]) -> enter_passkey {
        let mut cursor = Cursor::new(data);
        enter_passkey {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for enter_passkey {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct increase_security {
    pub result: u16,
}

impl FromBytes for increase_security {
    fn from_bytes(data: &[u8]) -> increase_security {
        let mut cursor = Cursor::new(data);
        increase_security {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for increase_security {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct list_all_bondings {
    pub result: u16,
}

impl FromBytes for list_all_bondings {
    fn from_bytes(data: &[u8]) -> list_all_bondings {
        let mut cursor = Cursor::new(data);
        list_all_bondings {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for list_all_bondings {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct passkey_confirm {
    pub result: u16,
}

impl FromBytes for passkey_confirm {
    fn from_bytes(data: &[u8]) -> passkey_confirm {
        let mut cursor = Cursor::new(data);
        passkey_confirm {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for passkey_confirm {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_bondable_mode {
    pub result: u16,
}

impl FromBytes for set_bondable_mode {
    fn from_bytes(data: &[u8]) -> set_bondable_mode {
        let mut cursor = Cursor::new(data);
        set_bondable_mode {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_bondable_mode {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_debug_mode {
    pub result: u16,
}

impl FromBytes for set_debug_mode {
    fn from_bytes(data: &[u8]) -> set_debug_mode {
        let mut cursor = Cursor::new(data);
        set_debug_mode {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_debug_mode {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_oob_data {
    pub result: u16,
}

impl FromBytes for set_oob_data {
    fn from_bytes(data: &[u8]) -> set_oob_data {
        let mut cursor = Cursor::new(data);
        set_oob_data {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_oob_data {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_passkey {
    pub result: u16,
}

impl FromBytes for set_passkey {
    fn from_bytes(data: &[u8]) -> set_passkey {
        let mut cursor = Cursor::new(data);
        set_passkey {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_passkey {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_sc_remote_oob_data {
    pub result: u16,
}

impl FromBytes for set_sc_remote_oob_data {
    fn from_bytes(data: &[u8]) -> set_sc_remote_oob_data {
        let mut cursor = Cursor::new(data);
        set_sc_remote_oob_data {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_sc_remote_oob_data {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct store_bonding_configuration {
    pub result: u16,
}

impl FromBytes for store_bonding_configuration {
    fn from_bytes(data: &[u8]) -> store_bonding_configuration {
        let mut cursor = Cursor::new(data);
        store_bonding_configuration {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for store_bonding_configuration {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct use_sc_oob {
    pub result: u16,
    pub oob_data: [u8; 32],
}

impl FromBytes for use_sc_oob {
    fn from_bytes(data: &[u8]) -> use_sc_oob {
        let mut cursor = Cursor::new(data);
        let result = cursor.get_u16_le();
        let mut oob_data: [u8; 32] = Default::default();
        cursor
            .read_exact(&mut oob_data)
            .expect("Failed to read bytes.");
        use_sc_oob { result, oob_data }
    }
}

impl ToBytes for use_sc_oob {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes.extend_from_slice(&self.oob_data);
        bytes
    }
}
