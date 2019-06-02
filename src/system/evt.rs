use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct awake {}

impl FromBytes for awake {
    fn from_bytes(_: &[u8]) -> awake {
        awake {}
    }
}

impl ToBytes for awake {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct boot {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub build: u16,
    pub bootloader: u32,
    pub hw: u16,
    pub hash: u32,
}

impl FromBytes for boot {
    fn from_bytes(data: &[u8]) -> boot {
        let mut cursor = Cursor::new(data);
        boot {
            major: cursor.get_u16_le(),
            minor: cursor.get_u16_le(),
            patch: cursor.get_u16_le(),
            build: cursor.get_u16_le(),
            bootloader: cursor.get_u32_le(),
            hw: cursor.get_u16_le(),
            hash: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for boot {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.major);
        bytes.put_u16_le(self.minor);
        bytes.put_u16_le(self.patch);
        bytes.put_u16_le(self.build);
        bytes.put_u32_le(self.bootloader);
        bytes.put_u16_le(self.hw);
        bytes.put_u32_le(self.hash);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct error {
    pub reason: u16,
    pub data: Vec<u8>,
}

impl FromBytes for error {
    fn from_bytes(data: &[u8]) -> error {
        let mut cursor = Cursor::new(data);
        let reason = cursor.get_u16_le();
        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .expect("Failed to read bytes.");
        error { reason, data }
    }
}

impl ToBytes for error {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.reason);
        bytes.extend(self.data.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct external_signal {
    pub extsignals: u32,
}

impl FromBytes for external_signal {
    fn from_bytes(data: &[u8]) -> external_signal {
        let mut cursor = Cursor::new(data);
        external_signal {
            extsignals: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for external_signal {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u32_le(self.extsignals);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct hardware_error {
    pub status: u16,
}

impl FromBytes for hardware_error {
    fn from_bytes(data: &[u8]) -> hardware_error {
        let mut cursor = Cursor::new(data);
        hardware_error {
            status: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for hardware_error {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.status);
        bytes
    }
}