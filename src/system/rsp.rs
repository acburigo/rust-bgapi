use bytes::{Buf, BufMut};
use error::Error;
use num_traits::FromPrimitive;
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct get_bt_address {
    pub address: [u8; 6],
}

impl From<&[u8]> for get_bt_address {
    fn from(data: &[u8]) -> get_bt_address {
        let mut cursor = Cursor::new(data);
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        address.reverse();
        get_bt_address { address }
    }
}

impl Into<Vec<u8>> for get_bt_address {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.address.iter().rev());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct get_counters {
    pub result: Error,
    pub tx_packets: u16,
    pub rx_packets: u16,
    pub crc_errors: u16,
    pub failures: u16,
}

impl From<&[u8]> for get_counters {
    fn from(data: &[u8]) -> get_counters {
        let mut cursor = Cursor::new(data);
        get_counters {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
            tx_packets: cursor.get_u16_le(),
            rx_packets: cursor.get_u16_le(),
            crc_errors: cursor.get_u16_le(),
            failures: cursor.get_u16_le(),
        }
    }
}

impl Into<Vec<u8>> for get_counters {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.put_u16_le(self.tx_packets);
        bytes.put_u16_le(self.rx_packets);
        bytes.put_u16_le(self.crc_errors);
        bytes.put_u16_le(self.failures);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct get_random_data {
    pub result: Error,
    pub data: Vec<u8>,
}

impl From<&[u8]> for get_random_data {
    fn from(data: &[u8]) -> get_random_data {
        let mut cursor = Cursor::new(data);
        let result = FromPrimitive::from_u16(cursor.get_u16_le()).unwrap();
        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .expect("Failed to read bytes.");
        get_random_data { result, data }
    }
}

impl Into<Vec<u8>> for get_random_data {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes.extend(self.data.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct halt {
    pub result: Error,
}

impl From<&[u8]> for halt {
    fn from(data: &[u8]) -> halt {
        let mut cursor = Cursor::new(data);
        halt {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for halt {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct hello {
    pub result: Error,
}

impl From<&[u8]> for hello {
    fn from(data: &[u8]) -> hello {
        let mut cursor = Cursor::new(data);
        hello {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for hello {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_bt_address {
    pub result: Error,
}

impl From<&[u8]> for set_bt_address {
    fn from(data: &[u8]) -> set_bt_address {
        let mut cursor = Cursor::new(data);
        set_bt_address {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_bt_address {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_device_name {
    pub result: Error,
}

impl From<&[u8]> for set_device_name {
    fn from(data: &[u8]) -> set_device_name {
        let mut cursor = Cursor::new(data);
        set_device_name {
            result: FromPrimitive::from_u16(cursor.get_u16_le()).unwrap(),
        }
    }
}

impl Into<Vec<u8>> for set_device_name {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result.clone() as u16);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_tx_power {
    pub set_power: i16,
}

impl From<&[u8]> for set_tx_power {
    fn from(data: &[u8]) -> set_tx_power {
        let mut cursor = Cursor::new(data);
        set_tx_power {
            set_power: cursor.get_i16_le(),
        }
    }
}

impl Into<Vec<u8>> for set_tx_power {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_i16_le(self.set_power);
        bytes
    }
}
