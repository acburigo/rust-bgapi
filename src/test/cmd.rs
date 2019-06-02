use bytes::{Buf, BufMut};
use num_traits::FromPrimitive;
use parser::{FromBytes, ToBytes};
use std::io::Cursor;
use test::{PacketType, Phy};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_end {}

impl FromBytes for dtm_end {
    fn from_bytes(_: &[u8]) -> dtm_end {
        dtm_end {}
    }
}

impl ToBytes for dtm_end {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_rx {
    pub channel: u8,
    pub phy: Phy,
}

impl FromBytes for dtm_rx {
    fn from_bytes(data: &[u8]) -> dtm_rx {
        let mut cursor = Cursor::new(data);
        dtm_rx {
            channel: cursor.get_u8(),
            phy: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
        }
    }
}

impl ToBytes for dtm_rx {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.channel);
        bytes.put_u8(self.phy.clone() as u8);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct dtm_tx {
    pub packet_type: PacketType,
    pub length: u8,
    pub channel: u8,
    pub phy: Phy,
}

impl FromBytes for dtm_tx {
    fn from_bytes(data: &[u8]) -> dtm_tx {
        let mut cursor = Cursor::new(data);
        dtm_tx {
            packet_type: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
            length: cursor.get_u8(),
            channel: cursor.get_u8(),
            phy: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
        }
    }
}

impl ToBytes for dtm_tx {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.packet_type.clone() as u8);
        bytes.put_u8(self.length);
        bytes.put_u8(self.channel);
        bytes.put_u8(self.phy.clone() as u8);
        bytes
    }
}
