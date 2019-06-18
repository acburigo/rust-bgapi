use bytes::{Buf, BufMut};
use error::Error;
use le_connection::Security;
use num_traits::FromPrimitive;
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct closed {
    pub reason: Error,
    pub connection: u8,
}

impl From<&[u8]> for closed {
    fn from(data: &[u8]) -> closed {
        let mut cursor = Cursor::new(data);
        closed {
            reason: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            connection: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for closed {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.reason.clone() as u16);
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct opened {
    pub address: [u8; 6],
    pub address_type: u8,
    pub master: u8,
    pub connection: u8,
    pub bonding: u8,
    pub advertiser: u8,
}

impl From<&[u8]> for opened {
    fn from(data: &[u8]) -> opened {
        let mut cursor = Cursor::new(data);
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        address.reverse();
        opened {
            address,
            address_type: cursor.get_u8(),
            master: cursor.get_u8(),
            connection: cursor.get_u8(),
            bonding: cursor.get_u8(),
            advertiser: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for opened {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.address.iter().rev());
        bytes.put_u8(self.address_type);
        bytes.put_u8(self.master);
        bytes.put_u8(self.connection);
        bytes.put_u8(self.bonding);
        bytes.put_u8(self.advertiser);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct parameters {
    pub connection: u8,
    pub interval: u16,
    pub latency: u16,
    pub timeout: u16,
    pub security_mode: Security,
    pub txsize: u16,
}

impl From<&[u8]> for parameters {
    fn from(data: &[u8]) -> parameters {
        let mut cursor = Cursor::new(data);
        parameters {
            connection: cursor.get_u8(),
            interval: cursor.get_u16_le(),
            latency: cursor.get_u16_le(),
            timeout: cursor.get_u16_le(),
            security_mode: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
            txsize: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for parameters {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u16_le(self.interval);
        bytes.put_u16_le(self.latency);
        bytes.put_u16_le(self.timeout);
        bytes.put_u8(self.security_mode.clone() as u8);
        bytes.put_u16_le(self.txsize);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct phy_status {
    pub connection: u8,
    pub phy: u8,
}

impl From<&[u8]> for phy_status {
    fn from(data: &[u8]) -> phy_status {
        let mut cursor = Cursor::new(data);
        phy_status {
            connection: cursor.get_u8(),
            phy: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for phy_status {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.phy);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct rssi {
    pub connection: u8,
    pub status: u8,
    pub rssi: i8,
}

impl From<&[u8]> for rssi {
    fn from(data: &[u8]) -> rssi {
        let mut cursor = Cursor::new(data);
        rssi {
            connection: cursor.get_u8(),
            status: cursor.get_u8(),
            rssi: cursor.get_i8(),
        }
    }
}

impl Into<Vec<u8>> for rssi {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.status);
        bytes.put_i8(self.rssi);
        bytes
    }
}
