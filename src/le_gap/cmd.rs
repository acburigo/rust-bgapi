use bytes::{Buf, BufMut};
use le_gap::{AddressType, ConnectableMode, DiscoverMode, DiscoverableMode, PhyType};
use num_traits::FromPrimitive;
use parser::{FromBytes, ToBytes};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct bt5_set_adv_data {
    pub handle: u8,
    pub scan_rsp: u8,
    pub adv_data: Vec<u8>,
}

impl FromBytes for bt5_set_adv_data {
    fn from_bytes(data: &[u8]) -> bt5_set_adv_data {
        let mut cursor = Cursor::new(data);
        let handle = cursor.get_u8();
        let scan_rsp = cursor.get_u8();
        let mut adv_data = Vec::new();
        cursor
            .read_to_end(&mut adv_data)
            .expect("Failed to read bytes.");
        bt5_set_adv_data {
            handle,
            scan_rsp,
            adv_data,
        }
    }
}

impl ToBytes for bt5_set_adv_data {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_u8(self.scan_rsp);
        bytes.extend(self.adv_data.iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct clear_advertise_configuration {
    pub handle: u8,
    pub configurations: u32,
}

impl FromBytes for clear_advertise_configuration {
    fn from_bytes(data: &[u8]) -> clear_advertise_configuration {
        let mut cursor = Cursor::new(data);
        clear_advertise_configuration {
            handle: cursor.get_u8(),
            configurations: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for clear_advertise_configuration {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_u32_le(self.configurations);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct connect {
    pub address: [u8; 6],
    pub address_type: AddressType,
    pub initiating_phy: PhyType,
}

impl FromBytes for connect {
    fn from_bytes(data: &[u8]) -> connect {
        let mut cursor = Cursor::new(data);
        let mut address: [u8; 6] = Default::default();
        cursor
            .read_exact(&mut address)
            .expect("Failed to read bytes.");
        connect {
            address,
            address_type: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
            initiating_phy: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
        }
    }
}

impl ToBytes for connect {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.address);
        bytes.put_u8(self.address_type.clone() as u8);
        bytes.put_u8(self.initiating_phy.clone() as u8);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct end_procedure {}

impl FromBytes for end_procedure {
    fn from_bytes(_: &[u8]) -> end_procedure {
        end_procedure {}
    }
}

impl ToBytes for end_procedure {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_channel_map {
    pub handle: u8,
    pub channel_map: u8,
}

impl FromBytes for set_advertise_channel_map {
    fn from_bytes(data: &[u8]) -> set_advertise_channel_map {
        let mut cursor = Cursor::new(data);
        set_advertise_channel_map {
            handle: cursor.get_u8(),
            channel_map: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_advertise_channel_map {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_u8(self.channel_map);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_configuration {
    pub handle: u8,
    pub configurations: u32,
}

impl FromBytes for set_advertise_configuration {
    fn from_bytes(data: &[u8]) -> set_advertise_configuration {
        let mut cursor = Cursor::new(data);
        set_advertise_configuration {
            handle: cursor.get_u8(),
            configurations: cursor.get_u32_le(),
        }
    }
}

impl ToBytes for set_advertise_configuration {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_u32_le(self.configurations);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_phy {
    pub handle: u8,
    pub primary_phy: PhyType,
    pub secondary_phy: PhyType,
}

impl FromBytes for set_advertise_phy {
    fn from_bytes(data: &[u8]) -> set_advertise_phy {
        let mut cursor = Cursor::new(data);
        set_advertise_phy {
            handle: cursor.get_u8(),
            primary_phy: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
            secondary_phy: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
        }
    }
}

impl ToBytes for set_advertise_phy {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_u8(self.primary_phy.clone() as u8);
        bytes.put_u8(self.secondary_phy.clone() as u8);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_report_scan_request {
    pub handle: u8,
    pub report_scan_req: u8,
}

impl FromBytes for set_advertise_report_scan_request {
    fn from_bytes(data: &[u8]) -> set_advertise_report_scan_request {
        let mut cursor = Cursor::new(data);
        set_advertise_report_scan_request {
            handle: cursor.get_u8(),
            report_scan_req: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_advertise_report_scan_request {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_u8(self.report_scan_req);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_timing {
    pub handle: u8,
    pub interval_min: u32,
    pub interval_max: u32,
    pub duration: u16,
    pub maxevents: u8,
}

impl FromBytes for set_advertise_timing {
    fn from_bytes(data: &[u8]) -> set_advertise_timing {
        let mut cursor = Cursor::new(data);
        set_advertise_timing {
            handle: cursor.get_u8(),
            interval_min: cursor.get_u32_le(),
            interval_max: cursor.get_u32_le(),
            duration: cursor.get_u16_le(),
            maxevents: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_advertise_timing {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_u32_le(self.interval_min);
        bytes.put_u32_le(self.interval_max);
        bytes.put_u16_le(self.duration);
        bytes.put_u8(self.maxevents);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_advertise_tx_power {
    pub handle: u8,
    pub power: i16,
}

impl FromBytes for set_advertise_tx_power {
    fn from_bytes(data: &[u8]) -> set_advertise_tx_power {
        let mut cursor = Cursor::new(data);
        set_advertise_tx_power {
            handle: cursor.get_u8(),
            power: cursor.get_i16_le(),
        }
    }
}

impl ToBytes for set_advertise_tx_power {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_i16_le(self.power);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_conn_parameters {
    pub min_interval: u16,
    pub max_interval: u16,
    pub latency: u16,
    pub timeout: u16,
}

impl FromBytes for set_conn_parameters {
    fn from_bytes(data: &[u8]) -> set_conn_parameters {
        let mut cursor = Cursor::new(data);
        set_conn_parameters {
            min_interval: cursor.get_u16_le(),
            max_interval: cursor.get_u16_le(),
            latency: cursor.get_u16_le(),
            timeout: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_conn_parameters {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u16_le(self.min_interval);
        bytes.put_u16_le(self.max_interval);
        bytes.put_u16_le(self.latency);
        bytes.put_u16_le(self.timeout);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_data_channel_classification {
    pub channel_map: [u8; 5],
}

impl FromBytes for set_data_channel_classification {
    fn from_bytes(data: &[u8]) -> set_data_channel_classification {
        let mut cursor = Cursor::new(data);
        let mut channel_map: [u8; 5] = Default::default();
        cursor
            .read_exact(&mut channel_map)
            .expect("Failed to read bytes.");
        set_data_channel_classification { channel_map }
    }
}

impl ToBytes for set_data_channel_classification {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.channel_map);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_discovery_timing {
    pub phys: u8,
    pub scan_interval: u16,
    pub scan_window: u16,
}

impl FromBytes for set_discovery_timing {
    fn from_bytes(data: &[u8]) -> set_discovery_timing {
        let mut cursor = Cursor::new(data);
        set_discovery_timing {
            phys: cursor.get_u8(),
            scan_interval: cursor.get_u16_le(),
            scan_window: cursor.get_u16_le(),
        }
    }
}

impl ToBytes for set_discovery_timing {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.phys);
        bytes.put_u16_le(self.scan_interval);
        bytes.put_u16_le(self.scan_window);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_discovery_type {
    pub phys: u8,
    pub scan_type: u8,
}

impl FromBytes for set_discovery_type {
    fn from_bytes(data: &[u8]) -> set_discovery_type {
        let mut cursor = Cursor::new(data);
        set_discovery_type {
            phys: cursor.get_u8(),
            scan_type: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_discovery_type {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.phys);
        bytes.put_u8(self.scan_type);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct set_privacy_mode {
    pub privacy: u8,
    pub interval: u8,
}

impl FromBytes for set_privacy_mode {
    fn from_bytes(data: &[u8]) -> set_privacy_mode {
        let mut cursor = Cursor::new(data);
        set_privacy_mode {
            privacy: cursor.get_u8(),
            interval: cursor.get_u8(),
        }
    }
}

impl ToBytes for set_privacy_mode {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.privacy);
        bytes.put_u8(self.interval);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct start_advertising {
    pub handle: u8,
    pub discover: DiscoverableMode,
    pub connect: ConnectableMode,
}

impl FromBytes for start_advertising {
    fn from_bytes(data: &[u8]) -> start_advertising {
        let mut cursor = Cursor::new(data);
        start_advertising {
            handle: cursor.get_u8(),
            discover: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
            connect: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
        }
    }
}

impl ToBytes for start_advertising {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes.put_u8(self.discover.clone() as u8);
        bytes.put_u8(self.connect.clone() as u8);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct start_discovery {
    pub scanning_phy: PhyType,
    pub mode: DiscoverMode,
}

impl FromBytes for start_discovery {
    fn from_bytes(data: &[u8]) -> start_discovery {
        let mut cursor = Cursor::new(data);
        start_discovery {
            scanning_phy: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
            mode: FromPrimitive::from_u8(cursor.get_u8()).unwrap(),
        }
    }
}

impl ToBytes for start_discovery {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.scanning_phy.clone() as u8);
        bytes.put_u8(self.mode.clone() as u8);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct stop_advertising {
    pub handle: u8,
}

impl FromBytes for stop_advertising {
    fn from_bytes(data: &[u8]) -> stop_advertising {
        let mut cursor = Cursor::new(data);
        stop_advertising {
            handle: cursor.get_u8(),
        }
    }
}

impl ToBytes for stop_advertising {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.handle);
        bytes
    }
}
