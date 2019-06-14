use bytes::{Buf, BufMut};
use le_gap::{AddressType, ConnectableMode, DiscoverMode, DiscoverableMode, PhyType};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use num_traits::FromPrimitive;
use parser::ToBytes;
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub struct bt5_set_adv_data {
    pub handle: u8,
    pub scan_rsp: u8,
    pub adv_data: Vec<u8>,
}

impl bt5_set_adv_data {
    pub fn new(handle: u8, scan_rsp: u8, adv_data: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::le_gap,
            message_id: 0x0c,
        };
        let payload = bt5_set_adv_data {
            handle,
            scan_rsp,
            adv_data,
        };
        let payload = MessagePayload::cmd_le_gap_bt5_set_adv_data(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for bt5_set_adv_data {
    fn from(data: &[u8]) -> bt5_set_adv_data {
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

impl clear_advertise_configuration {
    pub fn new(handle: u8, configurations: u32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05,
            message_class: MessageClass::le_gap,
            message_id: 0x13,
        };
        let payload = clear_advertise_configuration {
            handle,
            configurations,
        };
        let payload = MessagePayload::cmd_le_gap_clear_advertise_configuration(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for clear_advertise_configuration {
    fn from(data: &[u8]) -> clear_advertise_configuration {
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

impl connect {
    pub fn new(address: [u8; 6], address_type: AddressType, initiating_phy: PhyType) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x08,
            message_class: MessageClass::le_gap,
            message_id: 0x1a,
        };
        let payload = connect {
            address,
            address_type,
            initiating_phy,
        };
        let payload = MessagePayload::cmd_le_gap_connect(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for connect {
    fn from(data: &[u8]) -> connect {
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

impl end_procedure {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::le_gap,
            message_id: 0x03,
        };
        let payload = end_procedure {};
        let payload = MessagePayload::cmd_le_gap_end_procedure(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for end_procedure {
    fn from(_: &[u8]) -> end_procedure {
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

impl set_advertise_channel_map {
    pub fn new(handle: u8, channel_map: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x0f,
        };
        let payload = set_advertise_channel_map {
            handle,
            channel_map,
        };
        let payload = MessagePayload::cmd_le_gap_set_advertise_channel_map(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_advertise_channel_map {
    fn from(data: &[u8]) -> set_advertise_channel_map {
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

impl set_advertise_configuration {
    pub fn new(handle: u8, configurations: u32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05,
            message_class: MessageClass::le_gap,
            message_id: 0x12,
        };
        let payload = set_advertise_configuration {
            handle,
            configurations,
        };
        let payload = MessagePayload::cmd_le_gap_set_advertise_configuration(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_advertise_configuration {
    fn from(data: &[u8]) -> set_advertise_configuration {
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

impl set_advertise_phy {
    pub fn new(handle: u8, primary_phy: PhyType, secondary_phy: PhyType) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::le_gap,
            message_id: 0x11,
        };
        let payload = set_advertise_phy {
            handle,
            primary_phy,
            secondary_phy,
        };
        let payload = MessagePayload::cmd_le_gap_set_advertise_phy(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_advertise_phy {
    fn from(data: &[u8]) -> set_advertise_phy {
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

impl set_advertise_report_scan_request {
    pub fn new(handle: u8, report_scan_req: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x10,
        };
        let payload = set_advertise_report_scan_request {
            handle,
            report_scan_req,
        };
        let payload = MessagePayload::cmd_le_gap_set_advertise_report_scan_request(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_advertise_report_scan_request {
    fn from(data: &[u8]) -> set_advertise_report_scan_request {
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

impl set_advertise_timing {
    pub fn new(
        handle: u8,
        interval_min: u32,
        interval_max: u32,
        duration: u16,
        maxevents: u8,
    ) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x0c,
            message_class: MessageClass::le_gap,
            message_id: 0x0e,
        };
        let payload = set_advertise_timing {
            handle,
            interval_min,
            interval_max,
            duration,
            maxevents,
        };
        let payload = MessagePayload::cmd_le_gap_set_advertise_timing(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_advertise_timing {
    fn from(data: &[u8]) -> set_advertise_timing {
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

impl set_advertise_tx_power {
    pub fn new(handle: u8, power: i16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::le_gap,
            message_id: 0x1b,
        };
        let payload = set_advertise_tx_power { handle, power };
        let payload = MessagePayload::cmd_le_gap_set_advertise_tx_power(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_advertise_tx_power {
    fn from(data: &[u8]) -> set_advertise_tx_power {
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

impl set_conn_parameters {
    pub fn new(min_interval: u16, max_interval: u16, latency: u16, timeout: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x08,
            message_class: MessageClass::le_gap,
            message_id: 0x05,
        };
        let payload = set_conn_parameters {
            min_interval,
            max_interval,
            latency,
            timeout,
        };
        let payload = MessagePayload::cmd_le_gap_set_conn_parameters(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_conn_parameters {
    fn from(data: &[u8]) -> set_conn_parameters {
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

impl set_data_channel_classification {
    pub fn new(channel_map: [u8; 5]) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: channel_map.len() as u8,
            message_class: MessageClass::le_gap,
            message_id: 0x19,
        };
        let payload = set_data_channel_classification { channel_map };
        let payload = MessagePayload::cmd_le_gap_set_data_channel_classification(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_data_channel_classification {
    fn from(data: &[u8]) -> set_data_channel_classification {
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

impl set_discovery_timing {
    pub fn new(phys: u8, scan_interval: u16, scan_window: u16) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05,
            message_class: MessageClass::le_gap,
            message_id: 0x16,
        };
        let payload = set_discovery_timing {
            phys,
            scan_interval,
            scan_window,
        };
        let payload = MessagePayload::cmd_le_gap_set_discovery_timing(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_discovery_timing {
    fn from(data: &[u8]) -> set_discovery_timing {
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

impl set_discovery_type {
    pub fn new(phys: u8, scan_type: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x17,
        };
        let payload = set_discovery_type { phys, scan_type };
        let payload = MessagePayload::cmd_le_gap_set_discovery_type(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_discovery_type {
    fn from(data: &[u8]) -> set_discovery_type {
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

impl set_privacy_mode {
    pub fn new(privacy: u8, interval: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x0d,
        };
        let payload = set_privacy_mode { privacy, interval };
        let payload = MessagePayload::cmd_le_gap_set_privacy_mode(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_privacy_mode {
    fn from(data: &[u8]) -> set_privacy_mode {
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

impl start_advertising {
    pub fn new(handle: u8, discover: DiscoverableMode, connect: ConnectableMode) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x03,
            message_class: MessageClass::le_gap,
            message_id: 0x14,
        };
        let payload = start_advertising {
            handle,
            discover,
            connect,
        };
        let payload = MessagePayload::cmd_le_gap_start_advertising(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for start_advertising {
    fn from(data: &[u8]) -> start_advertising {
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

impl start_discovery {
    pub fn new(scanning_phy: PhyType, mode: DiscoverMode) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_gap,
            message_id: 0x18,
        };
        let payload = start_discovery { scanning_phy, mode };
        let payload = MessagePayload::cmd_le_gap_start_discovery(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for start_discovery {
    fn from(data: &[u8]) -> start_discovery {
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

impl stop_advertising {
    pub fn new(handle: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::le_gap,
            message_id: 0x15,
        };
        let payload = stop_advertising { handle };
        let payload = MessagePayload::cmd_le_gap_stop_advertising(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for stop_advertising {
    fn from(data: &[u8]) -> stop_advertising {
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
