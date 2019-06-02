use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use parser::{FromBytes, ToBytes};
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct bt5_set_adv_data {
    pub result: Error,
}

impl FromBytes for bt5_set_adv_data {
    fn from_bytes(data: &[u8]) -> bt5_set_adv_data {
        let mut cursor = Cursor::new(data);
        bt5_set_adv_data {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for bt5_set_adv_data {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct clear_advertise_configuration {
    pub result: Error,
}

impl FromBytes for clear_advertise_configuration {
    fn from_bytes(data: &[u8]) -> clear_advertise_configuration {
        let mut cursor = Cursor::new(data);
        clear_advertise_configuration {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for clear_advertise_configuration {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct connect {
    pub result: Error,
    pub connection: u8,
}

impl FromBytes for connect {
    fn from_bytes(data: &[u8]) -> connect {
        let mut cursor = Cursor::new(data);
        connect {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            connection: cursor.get_u8(),
        }
    }
}

impl ToBytes for connect {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct end_procedure {
    pub result: Error,
}

impl FromBytes for end_procedure {
    fn from_bytes(data: &[u8]) -> end_procedure {
        let mut cursor = Cursor::new(data);
        end_procedure {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for end_procedure {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_channel_map {
    pub result: Error,
}

impl FromBytes for set_advertise_channel_map {
    fn from_bytes(data: &[u8]) -> set_advertise_channel_map {
        let mut cursor = Cursor::new(data);
        set_advertise_channel_map {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_advertise_channel_map {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_configuration {
    pub result: Error,
}

impl FromBytes for set_advertise_configuration {
    fn from_bytes(data: &[u8]) -> set_advertise_configuration {
        let mut cursor = Cursor::new(data);
        set_advertise_configuration {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_advertise_configuration {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_phy {
    pub result: Error,
}

impl FromBytes for set_advertise_phy {
    fn from_bytes(data: &[u8]) -> set_advertise_phy {
        let mut cursor = Cursor::new(data);
        set_advertise_phy {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_advertise_phy {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_report_scan_request {
    pub result: Error,
}

impl FromBytes for set_advertise_report_scan_request {
    fn from_bytes(data: &[u8]) -> set_advertise_report_scan_request {
        let mut cursor = Cursor::new(data);
        set_advertise_report_scan_request {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_advertise_report_scan_request {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_timing {
    pub result: Error,
}

impl FromBytes for set_advertise_timing {
    fn from_bytes(data: &[u8]) -> set_advertise_timing {
        let mut cursor = Cursor::new(data);
        set_advertise_timing {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_advertise_timing {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_tx_power {
    pub result: Error,
}

impl FromBytes for set_advertise_tx_power {
    fn from_bytes(data: &[u8]) -> set_advertise_tx_power {
        let mut cursor = Cursor::new(data);
        set_advertise_tx_power {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_advertise_tx_power {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_conn_parameters {
    pub result: Error,
}

impl FromBytes for set_conn_parameters {
    fn from_bytes(data: &[u8]) -> set_conn_parameters {
        let mut cursor = Cursor::new(data);
        set_conn_parameters {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_conn_parameters {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_data_channel_classification {
    pub result: Error,
}

impl FromBytes for set_data_channel_classification {
    fn from_bytes(data: &[u8]) -> set_data_channel_classification {
        let mut cursor = Cursor::new(data);
        set_data_channel_classification {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_data_channel_classification {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_discovery_timing {
    pub result: Error,
}

impl FromBytes for set_discovery_timing {
    fn from_bytes(data: &[u8]) -> set_discovery_timing {
        let mut cursor = Cursor::new(data);
        set_discovery_timing {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_discovery_timing {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_discovery_type {
    pub result: Error,
}

impl FromBytes for set_discovery_type {
    fn from_bytes(data: &[u8]) -> set_discovery_type {
        let mut cursor = Cursor::new(data);
        set_discovery_type {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_discovery_type {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_privacy_mode {
    pub result: Error,
}

impl FromBytes for set_privacy_mode {
    fn from_bytes(data: &[u8]) -> set_privacy_mode {
        let mut cursor = Cursor::new(data);
        set_privacy_mode {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for set_privacy_mode {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct start_advertising {
    pub result: Error,
}

impl FromBytes for start_advertising {
    fn from_bytes(data: &[u8]) -> start_advertising {
        let mut cursor = Cursor::new(data);
        start_advertising {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for start_advertising {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct start_discovery {
    pub result: Error,
}

impl FromBytes for start_discovery {
    fn from_bytes(data: &[u8]) -> start_discovery {
        let mut cursor = Cursor::new(data);
        start_discovery {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for start_discovery {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct stop_advertising {
    pub result: Error,
}

impl FromBytes for stop_advertising {
    fn from_bytes(data: &[u8]) -> stop_advertising {
        let mut cursor = Cursor::new(data);
        stop_advertising {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl ToBytes for stop_advertising {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}
