use bytes::{Buf, BufMut};
use le_gap::AddressType;
use num_traits::FromPrimitive;
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct adv_timeout {
    pub handle: u8,
}

impl From<&[u8]> for adv_timeout {
    fn from(data: &[u8]) -> adv_timeout {
        let mut cursor = Cursor::new(data);
        adv_timeout {
            handle: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for adv_timeout {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct scan_request {
    pub handle: u8,
    pub address: [u8; 6],
    pub address_type: AddressType,
    pub bonding: u8,
}

impl From<&[u8]> for scan_request {
    fn from(data: &[u8]) -> scan_request {
        let mut cursor = Cursor::new(data);
        let handle = cursor.get_u8();
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        address.reverse();
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

impl Into<Vec<u8>> for scan_request {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.extend(self.address.iter().rev());
        bytes.put_u8(self.address_type.clone() as u8);
        bytes.put_u8(self.bonding);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct scan_response {
    pub rssi: i8,
    pub packet_type: u8,
    pub address: [u8; 6],
    pub address_type: AddressType,
    pub bonding: u8,
    pub data: Vec<u8>,
}

impl From<&[u8]> for scan_response {
    fn from(data: &[u8]) -> scan_response {
        let mut cursor = Cursor::new(data);
        let rssi = cursor.get_i8();
        let packet_type = cursor.get_u8();
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        address.reverse();
        let address_type = FromPrimitive::from_u8(cursor.get_u8()).unwrap();
        let bonding = cursor.get_u8();
        let mut data = Vec::new();
        cursor.get_u8();
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

impl Into<Vec<u8>> for scan_response {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_i8(self.rssi);
        bytes.put_u8(self.packet_type);
        bytes.extend(self.address.iter().rev());
        bytes.put_u8(self.address_type.clone() as u8);
        bytes.put_u8(self.bonding);
        bytes.put_u8(self.data.len() as u8);
        bytes.extend(self.data.iter());
        bytes
    }
}
