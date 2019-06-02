use bytes::{Buf, BufMut};
use le_gap::AddressType;
use num_traits::FromPrimitive;
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct adv_timeout {
    pub handle: u8,
}

impl FromBytes for adv_timeout {
    fn from_bytes(data: &[u8]) -> adv_timeout {
        let mut cursor = Cursor::new(data);
        adv_timeout {
            handle: cursor.get_u8(),
        }
    }
}

impl ToBytes for adv_timeout {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct scan_request {
    pub handle: u8,
    pub address: [u8; 6],
    pub address_type: AddressType,
    pub bonding: u8,
}

impl FromBytes for scan_request {
    fn from_bytes(data: &[u8]) -> scan_request {
        let mut cursor = Cursor::new(data);
        let handle = cursor.get_u8();
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        let address_type = FromPrimitive::from_u8(cursor.get_u8()).unwrap();
        let bonding = cursor.get_u8();
        scan_request {
            handle,
            address,
            address_type,
            bonding,
        }
    }
}

impl ToBytes for scan_request {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.extend_from_slice(&self.address);
        bytes.put_u8(self.address_type.clone() as u8);
        bytes.put_u8(self.bonding);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct scan_response {
    pub rssi: i8,
    pub packet_type: u8,
    pub address: [u8; 6],
    pub address_type: AddressType,
    pub bonding: u8,
    pub data: Vec<u8>,
}

impl FromBytes for scan_response {
    fn from_bytes(data: &[u8]) -> scan_response {
        let mut cursor = Cursor::new(data);
        let rssi = cursor.get_i8();
        let packet_type = cursor.get_u8();
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        let address_type = FromPrimitive::from_u8(cursor.get_u8()).unwrap();
        let bonding = cursor.get_u8();
        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .expect("Failed to read bytes.");
        scan_response {
            rssi,
            packet_type,
            address,
            address_type,
            bonding,
            data,
        }
    }
}

impl ToBytes for scan_response {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_i8(self.rssi);
        bytes.put_u8(self.packet_type);
        bytes.extend_from_slice(&self.address);
        bytes.put_u8(self.address_type.clone() as u8);
        bytes.put_u8(self.bonding);
        bytes.extend(self.data.iter());
        bytes
    }
}
