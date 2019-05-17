use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x0c,
        } => Ok(MessagePayload::rsp_le_gap_bt5_set_adv_data(
            rsp::bt5_set_adv_data::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x13,
        } => Ok(MessagePayload::rsp_le_gap_clear_advertise_configuration(
            rsp::clear_advertise_configuration::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x03,
            message_class: 0x03,
            message_id: 0x1a,
        } => Ok(MessagePayload::rsp_le_gap_connect(
            rsp::connect::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_le_gap_end_procedure(
            rsp::end_procedure::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x0f,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_channel_map(
            rsp::set_advertise_channel_map::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x12,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_configuration(
            rsp::set_advertise_configuration::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x11,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_phy(
            rsp::set_advertise_phy::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x10,
        } => Ok(
            MessagePayload::rsp_le_gap_set_advertise_report_scan_request(
                rsp::set_advertise_report_scan_request::from_bytes(buffer),
            ),
        ),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x0e,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_timing(
            rsp::set_advertise_timing::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x1b,
        } => Ok(MessagePayload::rsp_le_gap_set_advertise_tx_power(
            rsp::set_advertise_tx_power::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x05,
        } => Ok(MessagePayload::rsp_le_gap_set_conn_parameters(
            rsp::set_conn_parameters::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x19,
        } => Ok(MessagePayload::rsp_le_gap_set_data_channel_classification(
            rsp::set_data_channel_classification::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x16,
        } => Ok(MessagePayload::rsp_le_gap_set_discovery_timing(
            rsp::set_discovery_timing::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x17,
        } => Ok(MessagePayload::rsp_le_gap_set_discovery_type(
            rsp::set_discovery_type::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x0d,
        } => Ok(MessagePayload::rsp_le_gap_set_privacy_mode(
            rsp::set_privacy_mode::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x14,
        } => Ok(MessagePayload::rsp_le_gap_start_advertising(
            rsp::start_advertising::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x18,
        } => Ok(MessagePayload::rsp_le_gap_start_discovery(
            rsp::start_discovery::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x03,
            message_id: 0x15,
        } => Ok(MessagePayload::rsp_le_gap_stop_advertising(
            rsp::stop_advertising::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x01,
            message_class: 0x03,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_le_gap_adv_timeout(
            evt::adv_timeout::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x09,
            message_class: 0x03,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_le_gap_scan_request(
            evt::scan_request::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: _,
            message_class: 0x03,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_le_gap_scan_response(
            evt::scan_response::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
pub enum AddressType {
    public = 0,          // Public address
    random = 1,          // Random address
    public_identity = 2, // Public identity address resolved by stack
    random_identity = 3, // Random identity address resolved by stack
}

#[allow(non_camel_case_types)]
pub enum AdvAddressType {
    identity_address = 0, // Use public or static device address, or identity address if privacy mode is enabled
    non_resolvable = 1, // Use non resolvable address type, advertising mode must also be non-connectable
}

#[allow(non_camel_case_types)]
pub enum ConnectableMode {
    non_connectable = 0,           // Non-connectable non-scannable.
    directed_connectable = 1,      // Directed connectable (RESERVED, DO NOT USE)
    connectable_scannable = 2, // Undirected connectable scannable. This mode can only be used in legacy advertising PDUs.
    scannable_non_connectable = 3, // Undirected scannable (Non-connectable but responds to scan requests)
    connectable_non_scannable = 4, // Undirected connectable non-scannable. This mode can only be used in extended advertising PDUs.
}

#[allow(non_camel_case_types)]
pub enum DiscoverMode {
    limited = 0,     // Discover only limited discoverable devices
    generic = 1,     // Discover limited and generic discoverable devices
    observation = 2, // Discover all devices
}

#[allow(non_camel_case_types)]
pub enum DiscoverableMode {
    non_discoverable = 0,     // Not discoverable
    limited_discoverable = 1, // Discoverable using both limited and general discovery procedures
    general_discoverable = 2, // Discoverable using general discovery procedure
    broadcast = 3, // Device is not discoverable in either limited or generic discovery procedure, but may be discovered by using the Observation procedure
    user_data = 4, // Send advertising and/or scan response data defined by the user using le_gap_bt5_set_adv_data. The limited/general discoverable flags are defined by the user.
}

#[allow(non_camel_case_types)]
pub enum PhyType {
    phy_1m = 1,    // LE 1M PHY
    phy_2m = 2,    // LE 2M PHY
    phy_coded = 4, // LE Coded PHY
}

pub mod cmd {
    use bytes::{Buf, BufMut};
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
        pub address_type: u8,
        pub initiating_phy: u8,
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
                address_type: cursor.get_u8(),
                initiating_phy: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for connect {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&self.address);
            bytes.put_u8(self.address_type);
            bytes.put_u8(self.initiating_phy);
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
        pub primary_phy: u8,
        pub secondary_phy: u8,
    }

    impl FromBytes for set_advertise_phy {
        fn from_bytes(data: &[u8]) -> set_advertise_phy {
            let mut cursor = Cursor::new(data);
            set_advertise_phy {
                handle: cursor.get_u8(),
                primary_phy: cursor.get_u8(),
                secondary_phy: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for set_advertise_phy {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.handle);
            bytes.put_u8(self.primary_phy);
            bytes.put_u8(self.secondary_phy);
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
        pub discover: u8,
        pub connect: u8,
    }

    impl FromBytes for start_advertising {
        fn from_bytes(data: &[u8]) -> start_advertising {
            let mut cursor = Cursor::new(data);
            start_advertising {
                handle: cursor.get_u8(),
                discover: cursor.get_u8(),
                connect: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for start_advertising {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.handle);
            bytes.put_u8(self.discover);
            bytes.put_u8(self.connect);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct start_discovery {
        pub scanning_phy: u8,
        pub mode: u8,
    }

    impl FromBytes for start_discovery {
        fn from_bytes(data: &[u8]) -> start_discovery {
            let mut cursor = Cursor::new(data);
            start_discovery {
                scanning_phy: cursor.get_u8(),
                mode: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for start_discovery {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.scanning_phy);
            bytes.put_u8(self.mode);
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
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct bt5_set_adv_data {
        pub result: u16,
    }

    impl FromBytes for bt5_set_adv_data {
        fn from_bytes(data: &[u8]) -> bt5_set_adv_data {
            let mut cursor = Cursor::new(data);
            bt5_set_adv_data {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for bt5_set_adv_data {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct clear_advertise_configuration {
        pub result: u16,
    }

    impl FromBytes for clear_advertise_configuration {
        fn from_bytes(data: &[u8]) -> clear_advertise_configuration {
            let mut cursor = Cursor::new(data);
            clear_advertise_configuration {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for clear_advertise_configuration {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct connect {
        pub result: u16,
        pub connection: u8,
    }

    impl FromBytes for connect {
        fn from_bytes(data: &[u8]) -> connect {
            let mut cursor = Cursor::new(data);
            connect {
                result: cursor.get_u16_le(),
                connection: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for connect {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u8(self.connection);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct end_procedure {
        pub result: u16,
    }

    impl FromBytes for end_procedure {
        fn from_bytes(data: &[u8]) -> end_procedure {
            let mut cursor = Cursor::new(data);
            end_procedure {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for end_procedure {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_advertise_channel_map {
        pub result: u16,
    }

    impl FromBytes for set_advertise_channel_map {
        fn from_bytes(data: &[u8]) -> set_advertise_channel_map {
            let mut cursor = Cursor::new(data);
            set_advertise_channel_map {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_advertise_channel_map {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_advertise_configuration {
        pub result: u16,
    }

    impl FromBytes for set_advertise_configuration {
        fn from_bytes(data: &[u8]) -> set_advertise_configuration {
            let mut cursor = Cursor::new(data);
            set_advertise_configuration {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_advertise_configuration {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_advertise_phy {
        pub result: u16,
    }

    impl FromBytes for set_advertise_phy {
        fn from_bytes(data: &[u8]) -> set_advertise_phy {
            let mut cursor = Cursor::new(data);
            set_advertise_phy {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_advertise_phy {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_advertise_report_scan_request {
        pub result: u16,
    }

    impl FromBytes for set_advertise_report_scan_request {
        fn from_bytes(data: &[u8]) -> set_advertise_report_scan_request {
            let mut cursor = Cursor::new(data);
            set_advertise_report_scan_request {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_advertise_report_scan_request {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_advertise_timing {
        pub result: u16,
    }

    impl FromBytes for set_advertise_timing {
        fn from_bytes(data: &[u8]) -> set_advertise_timing {
            let mut cursor = Cursor::new(data);
            set_advertise_timing {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_advertise_timing {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_advertise_tx_power {
        pub result: u16,
    }

    impl FromBytes for set_advertise_tx_power {
        fn from_bytes(data: &[u8]) -> set_advertise_tx_power {
            let mut cursor = Cursor::new(data);
            set_advertise_tx_power {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_advertise_tx_power {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_conn_parameters {
        pub result: u16,
    }

    impl FromBytes for set_conn_parameters {
        fn from_bytes(data: &[u8]) -> set_conn_parameters {
            let mut cursor = Cursor::new(data);
            set_conn_parameters {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_conn_parameters {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_data_channel_classification {
        pub result: u16,
    }

    impl FromBytes for set_data_channel_classification {
        fn from_bytes(data: &[u8]) -> set_data_channel_classification {
            let mut cursor = Cursor::new(data);
            set_data_channel_classification {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_data_channel_classification {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_discovery_timing {
        pub result: u16,
    }

    impl FromBytes for set_discovery_timing {
        fn from_bytes(data: &[u8]) -> set_discovery_timing {
            let mut cursor = Cursor::new(data);
            set_discovery_timing {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_discovery_timing {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_discovery_type {
        pub result: u16,
    }

    impl FromBytes for set_discovery_type {
        fn from_bytes(data: &[u8]) -> set_discovery_type {
            let mut cursor = Cursor::new(data);
            set_discovery_type {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_discovery_type {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_privacy_mode {
        pub result: u16,
    }

    impl FromBytes for set_privacy_mode {
        fn from_bytes(data: &[u8]) -> set_privacy_mode {
            let mut cursor = Cursor::new(data);
            set_privacy_mode {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_privacy_mode {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct start_advertising {
        pub result: u16,
    }

    impl FromBytes for start_advertising {
        fn from_bytes(data: &[u8]) -> start_advertising {
            let mut cursor = Cursor::new(data);
            start_advertising {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for start_advertising {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct start_discovery {
        pub result: u16,
    }

    impl FromBytes for start_discovery {
        fn from_bytes(data: &[u8]) -> start_discovery {
            let mut cursor = Cursor::new(data);
            start_discovery {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for start_discovery {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct stop_advertising {
        pub result: u16,
    }

    impl FromBytes for stop_advertising {
        fn from_bytes(data: &[u8]) -> stop_advertising {
            let mut cursor = Cursor::new(data);
            stop_advertising {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for stop_advertising {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }
}

pub mod evt {
    use bytes::{Buf, BufMut};
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
        pub address_type: u8,
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
            let address_type = cursor.get_u8();
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
            bytes.put_u8(self.address_type);
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
        pub address_type: u8,
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
            let address_type = cursor.get_u8();
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
            bytes.put_u8(self.address_type);
            bytes.put_u8(self.bonding);
            bytes.extend(self.data.iter());
            bytes
        }
    }
}
