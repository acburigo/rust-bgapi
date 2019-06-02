use bytes::{Buf, BufMut};
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_bt_address {
    pub address: [u8; 6],
}

impl FromBytes for get_bt_address {
    fn from_bytes(data: &[u8]) -> get_bt_address {
        let mut cursor = Cursor::new(data);
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        get_bt_address { address }
    }
}

impl ToBytes for get_bt_address {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.address);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_counters {
    pub result: u16,
    pub tx_packets: u16,
    pub rx_packets: u16,
    pub crc_errors: u16,
    pub failures: u16,
}

impl FromBytes for get_counters {
    fn from_bytes(data: &[u8]) -> get_counters {
        let mut cursor = Cursor::new(data);
        get_counters {
            result: cursor.get_u16_le(),
            tx_packets: cursor.get_u16_le(),
            rx_packets: cursor.get_u16_le(),
            crc_errors: cursor.get_u16_le(),
            failures: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for get_counters {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes.put_u16_le(self.tx_packets);
        bytes.put_u16_le(self.rx_packets);
        bytes.put_u16_le(self.crc_errors);
        bytes.put_u16_le(self.failures);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct get_random_data {
    pub result: u16,
    pub data: Vec<u8>,
}

impl FromBytes for get_random_data {
    fn from_bytes(data: &[u8]) -> get_random_data {
        let mut cursor = Cursor::new(data);
        let result = cursor.get_u16_le();
        let mut data = Vec::new();
        cursor
            .read_to_end(&mut data)
            .expect("Failed to read bytes.");
        get_random_data { result, data }
    }
}

impl ToBytes for get_random_data {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes.extend(self.data.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct halt {
    pub result: u16,
}

impl FromBytes for halt {
    fn from_bytes(data: &[u8]) -> halt {
        let mut cursor = Cursor::new(data);
        halt {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for halt {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct hello {
    pub result: u16,
}

impl FromBytes for hello {
    fn from_bytes(data: &[u8]) -> hello {
        let mut cursor = Cursor::new(data);
        hello {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for hello {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_bt_address {
    pub result: u16,
}

impl FromBytes for set_bt_address {
    fn from_bytes(data: &[u8]) -> set_bt_address {
        let mut cursor = Cursor::new(data);
        set_bt_address {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_bt_address {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_device_name {
    pub result: u16,
}

impl FromBytes for set_device_name {
    fn from_bytes(data: &[u8]) -> set_device_name {
        let mut cursor = Cursor::new(data);
        set_device_name {
            result: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_device_name {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.result);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_tx_power {
    pub set_power: i16,
}

impl FromBytes for set_tx_power {
    fn from_bytes(data: &[u8]) -> set_tx_power {
        let mut cursor = Cursor::new(data);
        set_tx_power {
            set_power: cursor.get_i16_le(),
        }
    }
}

impl ToBytes for set_tx_power {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_i16_le(self.set_power);
        bytes
    }
}
