use message::{MessageHeader, MessagePayload, MessageType, MessageClass};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x06,
            message_class: MessageClass::system,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_system_get_bt_address(
            rsp::get_bt_address::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x0a,
            message_class: MessageClass::system,
            message_id: 0x0f,
        } => Ok(MessagePayload::rsp_system_get_counters(
            rsp::get_counters::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: _,
            message_class: MessageClass::system,
            message_id: 0x0b,
        } => Ok(MessagePayload::rsp_system_get_random_data(
            rsp::get_random_data::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x0c,
        } => Ok(MessagePayload::rsp_system_halt(rsp::halt::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_system_hello(rsp::hello::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_system_set_bt_address(
            rsp::set_bt_address::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x0d,
        } => Ok(MessagePayload::rsp_system_set_device_name(
            rsp::set_device_name::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x0a,
        } => Ok(MessagePayload::rsp_system_set_tx_power(
            rsp::set_tx_power::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x00,
            message_class: MessageClass::system,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_system_awake(evt::awake::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x12,
            message_class: MessageClass::system,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_system_boot(evt::boot::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: _,
            message_class: MessageClass::system,
            message_id: 0x06,
        } => Ok(MessagePayload::evt_system_error(evt::error::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x04,
            message_class: MessageClass::system,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_system_external_signal(
            evt::external_signal::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x05,
        } => Ok(MessagePayload::evt_system_hardware_error(
            evt::hardware_error::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct get_bt_address {}

    impl FromBytes for get_bt_address {
        fn from_bytes(_: &[u8]) -> get_bt_address {
            get_bt_address {}
        }
    }

    impl ToBytes for get_bt_address {
        fn to_bytes(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct get_counters {
        pub reset: u8,
    }

    impl FromBytes for get_counters {
        fn from_bytes(data: &[u8]) -> get_counters {
            let mut cursor = Cursor::new(data);
            get_counters {
                reset: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for get_counters {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.reset);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct get_random_data {
        pub length: u8,
    }

    impl FromBytes for get_random_data {
        fn from_bytes(data: &[u8]) -> get_random_data {
            let mut cursor = Cursor::new(data);
            get_random_data {
                length: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for get_random_data {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.length);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct halt {
        pub halt: u8,
    }

    impl FromBytes for halt {
        fn from_bytes(data: &[u8]) -> halt {
            let mut cursor = Cursor::new(data);
            halt {
                halt: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for halt {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.halt);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct hello {}

    impl FromBytes for hello {
        fn from_bytes(_: &[u8]) -> hello {
            hello {}
        }
    }

    impl ToBytes for hello {
        fn to_bytes(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct reset {
        pub dfu: u8,
    }

    impl FromBytes for reset {
        fn from_bytes(data: &[u8]) -> reset {
            let mut cursor = Cursor::new(data);
            reset {
                dfu: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for reset {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.dfu);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_bt_address {
        pub address: [u8; 6],
    }

    impl FromBytes for set_bt_address {
        fn from_bytes(data: &[u8]) -> set_bt_address {
            let mut cursor = Cursor::new(data);
            let mut address: [u8; 6] = Default::default();
            cursor
                .read_exact(&mut address)
                .expect("Failed to read bytes.");
            set_bt_address { address }
        }
    }

    impl ToBytes for set_bt_address {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&self.address);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_device_name {
        pub dtype: u8,
        pub name: Vec<u8>,
    }

    impl FromBytes for set_device_name {
        fn from_bytes(data: &[u8]) -> set_device_name {
            let mut cursor = Cursor::new(data);
            let dtype = cursor.get_u8();
            let mut name = Vec::new();
            cursor
                .read_to_end(&mut name)
                .expect("Failed to read bytes.");
            set_device_name { dtype, name }
        }
    }

    impl ToBytes for set_device_name {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.dtype);
            bytes.extend(self.name.iter());
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_tx_power {
        pub power: i16,
    }

    impl FromBytes for set_tx_power {
        fn from_bytes(data: &[u8]) -> set_tx_power {
            let mut cursor = Cursor::new(data);
            set_tx_power {
                power: cursor.get_i16_le(),
            }
        }
    }

    impl ToBytes for set_tx_power {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_i16_le(self.power);
            bytes
        }
    }
}

pub mod rsp {
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
}

pub mod evt {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct awake {}

    impl FromBytes for awake {
        fn from_bytes(_: &[u8]) -> awake {
            awake {}
        }
    }

    impl ToBytes for awake {
        fn to_bytes(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct boot {
        pub major: u16,
        pub minor: u16,
        pub patch: u16,
        pub build: u16,
        pub bootloader: u32,
        pub hw: u16,
        pub hash: u32,
    }

    impl FromBytes for boot {
        fn from_bytes(data: &[u8]) -> boot {
            let mut cursor = Cursor::new(data);
            boot {
                major: cursor.get_u16_le(),
                minor: cursor.get_u16_le(),
                patch: cursor.get_u16_le(),
                build: cursor.get_u16_le(),
                bootloader: cursor.get_u32_le(),
                hw: cursor.get_u16_le(),
                hash: cursor.get_u32_le(),
            }
        }
    }

    impl ToBytes for boot {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.major);
            bytes.put_u16_le(self.minor);
            bytes.put_u16_le(self.patch);
            bytes.put_u16_le(self.build);
            bytes.put_u32_le(self.bootloader);
            bytes.put_u16_le(self.hw);
            bytes.put_u32_le(self.hash);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct error {
        pub reason: u16,
        pub data: Vec<u8>,
    }

    impl FromBytes for error {
        fn from_bytes(data: &[u8]) -> error {
            let mut cursor = Cursor::new(data);
            let reason = cursor.get_u16_le();
            let mut data = Vec::new();
            cursor
                .read_to_end(&mut data)
                .expect("Failed to read bytes.");
            error { reason, data }
        }
    }

    impl ToBytes for error {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.reason);
            bytes.extend(self.data.iter());
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct external_signal {
        pub extsignals: u32,
    }

    impl FromBytes for external_signal {
        fn from_bytes(data: &[u8]) -> external_signal {
            let mut cursor = Cursor::new(data);
            external_signal {
                extsignals: cursor.get_u32_le(),
            }
        }
    }

    impl ToBytes for external_signal {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u32_le(self.extsignals);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct hardware_error {
        pub status: u16,
    }

    impl FromBytes for hardware_error {
        fn from_bytes(data: &[u8]) -> hardware_error {
            let mut cursor = Cursor::new(data);
            hardware_error {
                status: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for hardware_error {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.status);
            bytes
        }
    }
}
