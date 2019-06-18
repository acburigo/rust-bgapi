use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct bonding_confirm {
    pub result: Error,
}

impl From<&[u8]> for bonding_confirm {
    fn from(data: &[u8]) -> bonding_confirm {
        let mut cursor = Cursor::new(data);
        bonding_confirm {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for bonding_confirm {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct configure {
    pub result: Error,
}

impl From<&[u8]> for configure {
    fn from(data: &[u8]) -> configure {
        let mut cursor = Cursor::new(data);
        configure {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for configure {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct delete_bonding {
    pub result: Error,
}

impl From<&[u8]> for delete_bonding {
    fn from(data: &[u8]) -> delete_bonding {
        let mut cursor = Cursor::new(data);
        delete_bonding {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for delete_bonding {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct delete_bondings {
    pub result: Error,
}

impl From<&[u8]> for delete_bondings {
    fn from(data: &[u8]) -> delete_bondings {
        let mut cursor = Cursor::new(data);
        delete_bondings {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for delete_bondings {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct enter_passkey {
    pub result: Error,
}

impl From<&[u8]> for enter_passkey {
    fn from(data: &[u8]) -> enter_passkey {
        let mut cursor = Cursor::new(data);
        enter_passkey {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for enter_passkey {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct increase_security {
    pub result: Error,
}

impl From<&[u8]> for increase_security {
    fn from(data: &[u8]) -> increase_security {
        let mut cursor = Cursor::new(data);
        increase_security {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for increase_security {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct list_all_bondings {
    pub result: Error,
}

impl From<&[u8]> for list_all_bondings {
    fn from(data: &[u8]) -> list_all_bondings {
        let mut cursor = Cursor::new(data);
        list_all_bondings {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for list_all_bondings {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct passkey_confirm {
    pub result: Error,
}

impl From<&[u8]> for passkey_confirm {
    fn from(data: &[u8]) -> passkey_confirm {
        let mut cursor = Cursor::new(data);
        passkey_confirm {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for passkey_confirm {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_bondable_mode {
    pub result: Error,
}

impl From<&[u8]> for set_bondable_mode {
    fn from(data: &[u8]) -> set_bondable_mode {
        let mut cursor = Cursor::new(data);
        set_bondable_mode {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_bondable_mode {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_debug_mode {
    pub result: Error,
}

impl From<&[u8]> for set_debug_mode {
    fn from(data: &[u8]) -> set_debug_mode {
        let mut cursor = Cursor::new(data);
        set_debug_mode {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_debug_mode {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_oob_data {
    pub result: Error,
}

impl From<&[u8]> for set_oob_data {
    fn from(data: &[u8]) -> set_oob_data {
        let mut cursor = Cursor::new(data);
        set_oob_data {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_oob_data {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_passkey {
    pub result: Error,
}

impl From<&[u8]> for set_passkey {
    fn from(data: &[u8]) -> set_passkey {
        let mut cursor = Cursor::new(data);
        set_passkey {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_passkey {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct set_sc_remote_oob_data {
    pub result: Error,
}

impl From<&[u8]> for set_sc_remote_oob_data {
    fn from(data: &[u8]) -> set_sc_remote_oob_data {
        let mut cursor = Cursor::new(data);
        set_sc_remote_oob_data {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_sc_remote_oob_data {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct store_bonding_configuration {
    pub result: Error,
}

impl From<&[u8]> for store_bonding_configuration {
    fn from(data: &[u8]) -> store_bonding_configuration {
        let mut cursor = Cursor::new(data);
        store_bonding_configuration {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for store_bonding_configuration {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct use_sc_oob {
    pub result: Error,
    pub oob_data: [u8; 32],
}

impl From<&[u8]> for use_sc_oob {
    fn from(data: &[u8]) -> use_sc_oob {
        let mut cursor = Cursor::new(data);
        let result = FromPrimitive::from_u16(cursor.get_u16_le()).unwrap();
        let mut oob_data: [u8; 32] = Default::default();
        cursor
            .read_exact(&mut oob_data)
            .expect("Failed to read bytes.");
        use_sc_oob { result, oob_data }
    }
}

impl Into<Vec<u8>> for use_sc_oob {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.extend_from_slice(&self.oob_data);
        bytes
    }
}
