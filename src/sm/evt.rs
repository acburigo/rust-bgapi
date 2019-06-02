use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct bonded {
    pub connection: u8,
    pub bonding: u8,
}

impl FromBytes for bonded {
    fn from_bytes(data: &[u8]) -> bonded {
        let mut cursor = Cursor::new(data);
        bonded {
            connection: cursor.get_u8(),
            bonding: cursor.get_u8(),
        }
    }
}

impl ToBytes for bonded {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.bonding);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct bonding_failed {
    pub connection: u8,
    pub reason: u16,
}

impl FromBytes for bonding_failed {
    fn from_bytes(data: &[u8]) -> bonding_failed {
        let mut cursor = Cursor::new(data);
        bonding_failed {
            connection: cursor.get_u8(),
            reason: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for bonding_failed {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.reason);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct confirm_bonding {
    pub connection: u8,
    pub bonding_handle: i8,
}

impl FromBytes for confirm_bonding {
    fn from_bytes(data: &[u8]) -> confirm_bonding {
        let mut cursor = Cursor::new(data);
        confirm_bonding {
            connection: cursor.get_u8(),
            bonding_handle: cursor.get_i8(),
        }
    }
}

impl ToBytes for confirm_bonding {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_i8(self.bonding_handle);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct confirm_passkey {
    pub connection: u8,
    pub passkey: u32,
}

impl FromBytes for confirm_passkey {
    fn from_bytes(data: &[u8]) -> confirm_passkey {
        let mut cursor = Cursor::new(data);
        confirm_passkey {
            connection: cursor.get_u8(),
            passkey: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for confirm_passkey {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.passkey);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct list_all_bondings_complete {}

impl FromBytes for list_all_bondings_complete {
    fn from_bytes(_: &[u8]) -> list_all_bondings_complete {
        list_all_bondings_complete {}
    }
}

impl ToBytes for list_all_bondings_complete {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct list_bonding_entry {
    pub bonding: u8,
    pub address: [u8; 6],
    pub address_type: u8,
}

impl FromBytes for list_bonding_entry {
    fn from_bytes(data: &[u8]) -> list_bonding_entry {
        let mut cursor = Cursor::new(data);
        let bonding = cursor.get_u8();
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        let address_type = cursor.get_u8();
        list_bonding_entry {
            bonding,
            address,
            address_type,
        }
    }
}

impl ToBytes for list_bonding_entry {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.bonding);
        bytes.extend_from_slice(&self.address);
        bytes.put_u8(self.address_type);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct passkey_display {
    pub connection: u8,
    pub passkey: u32,
}

impl FromBytes for passkey_display {
    fn from_bytes(data: &[u8]) -> passkey_display {
        let mut cursor = Cursor::new(data);
        passkey_display {
            connection: cursor.get_u8(),
            passkey: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for passkey_display {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u32_le(self.passkey);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct passkey_request {
    pub connection: u8,
}

impl FromBytes for passkey_request {
    fn from_bytes(data: &[u8]) -> passkey_request {
        let mut cursor = Cursor::new(data);
        passkey_request {
            connection: cursor.get_u8(),
        }
    }
}

impl ToBytes for passkey_request {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}
