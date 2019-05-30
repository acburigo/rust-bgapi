use message::{MessageHeader, MessagePayload, MessageType, MessageClass};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_le_connection_close(
            rsp::close::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_le_connection_disable_slave_latency(
            rsp::disable_slave_latency::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_le_connection_get_rssi(
            rsp::get_rssi::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_le_connection_set_parameters(
            rsp::set_parameters::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_le_connection_set_phy(
            rsp::set_phy::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::le_connection,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_le_connection_closed(
            evt::closed::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x0b,
            message_class: MessageClass::le_connection,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_le_connection_opened(
            evt::opened::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x0a,
            message_class: MessageClass::le_connection,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_le_connection_parameters(
            evt::parameters::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::le_connection,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_le_connection_phy_status(
            evt::phy_status::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::le_connection,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_le_connection_rssi(
            evt::rssi::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
pub enum Security {
    mode1_level1 = 0, // No security
    mode1_level2 = 1, // Unauthenticated pairing with encryption
    mode1_level3 = 2, // Authenticated pairing with encryption
    mode1_level4 = 3, // Authenticated Secure Connections pairing with encryption using a 128-bit strength encryption key
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct close {
        pub connection: u8,
    }

    impl FromBytes for close {
        fn from_bytes(data: &[u8]) -> close {
            let mut cursor = Cursor::new(data);
            close {
                connection: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for close {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct disable_slave_latency {
        pub connection: u8,
        pub disable: u8,
    }

    impl FromBytes for disable_slave_latency {
        fn from_bytes(data: &[u8]) -> disable_slave_latency {
            let mut cursor = Cursor::new(data);
            disable_slave_latency {
                connection: cursor.get_u8(),
                disable: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for disable_slave_latency {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u8(self.disable);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct get_rssi {
        pub connection: u8,
    }

    impl FromBytes for get_rssi {
        fn from_bytes(data: &[u8]) -> get_rssi {
            let mut cursor = Cursor::new(data);
            get_rssi {
                connection: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for get_rssi {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_parameters {
        pub connection: u8,
        pub min_interval: u16,
        pub max_interval: u16,
        pub latency: u16,
        pub timeout: u16,
    }

    impl FromBytes for set_parameters {
        fn from_bytes(data: &[u8]) -> set_parameters {
            let mut cursor = Cursor::new(data);
            set_parameters {
                connection: cursor.get_u8(),
                min_interval: cursor.get_u16_le(),
                max_interval: cursor.get_u16_le(),
                latency: cursor.get_u16_le(),
                timeout: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_parameters {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u16_le(self.min_interval);
            bytes.put_u16_le(self.max_interval);
            bytes.put_u16_le(self.latency);
            bytes.put_u16_le(self.timeout);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_phy {
        pub connection: u8,
        pub phy: u8,
    }

    impl FromBytes for set_phy {
        fn from_bytes(data: &[u8]) -> set_phy {
            let mut cursor = Cursor::new(data);
            set_phy {
                connection: cursor.get_u8(),
                phy: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for set_phy {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u8(self.phy);
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
    pub struct close {
        pub result: u16,
    }

    impl FromBytes for close {
        fn from_bytes(data: &[u8]) -> close {
            let mut cursor = Cursor::new(data);
            close {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for close {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct disable_slave_latency {
        pub result: u16,
    }

    impl FromBytes for disable_slave_latency {
        fn from_bytes(data: &[u8]) -> disable_slave_latency {
            let mut cursor = Cursor::new(data);
            disable_slave_latency {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for disable_slave_latency {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct get_rssi {
        pub result: u16,
    }

    impl FromBytes for get_rssi {
        fn from_bytes(data: &[u8]) -> get_rssi {
            let mut cursor = Cursor::new(data);
            get_rssi {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for get_rssi {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_parameters {
        pub result: u16,
    }

    impl FromBytes for set_parameters {
        fn from_bytes(data: &[u8]) -> set_parameters {
            let mut cursor = Cursor::new(data);
            set_parameters {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_parameters {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_phy {
        pub result: u16,
    }

    impl FromBytes for set_phy {
        fn from_bytes(data: &[u8]) -> set_phy {
            let mut cursor = Cursor::new(data);
            set_phy {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_phy {
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
    pub struct closed {
        pub reason: u16,
        pub connection: u8,
    }

    impl FromBytes for closed {
        fn from_bytes(data: &[u8]) -> closed {
            let mut cursor = Cursor::new(data);
            closed {
                reason: cursor.get_u16_le(),
                connection: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for closed {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.reason);
            bytes.put_u8(self.connection);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct opened {
        pub address: [u8; 6],
        pub address_type: u8,
        pub master: u8,
        pub connection: u8,
        pub bonding: u8,
        pub advertiser: u8,
    }

    impl FromBytes for opened {
        fn from_bytes(data: &[u8]) -> opened {
            let mut cursor = Cursor::new(data);
            let mut address: [u8; 6] = Default::default();
            cursor
                .read_exact(&mut address)
                .expect("Failed to read bytes.");
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

    impl ToBytes for opened {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&self.address);
            bytes.put_u8(self.address_type);
            bytes.put_u8(self.master);
            bytes.put_u8(self.connection);
            bytes.put_u8(self.bonding);
            bytes.put_u8(self.advertiser);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct parameters {
        pub connection: u8,
        pub interval: u16,
        pub latency: u16,
        pub timeout: u16,
        pub security_mode: u8,
        pub txsize: u16,
    }

    impl FromBytes for parameters {
        fn from_bytes(data: &[u8]) -> parameters {
            let mut cursor = Cursor::new(data);
            parameters {
                connection: cursor.get_u8(),
                interval: cursor.get_u16_le(),
                latency: cursor.get_u16_le(),
                timeout: cursor.get_u16_le(),
                security_mode: cursor.get_u8(),
                txsize: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for parameters {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u16_le(self.interval);
            bytes.put_u16_le(self.latency);
            bytes.put_u16_le(self.timeout);
            bytes.put_u8(self.security_mode);
            bytes.put_u16_le(self.txsize);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct phy_status {
        pub connection: u8,
        pub phy: u8,
    }

    impl FromBytes for phy_status {
        fn from_bytes(data: &[u8]) -> phy_status {
            let mut cursor = Cursor::new(data);
            phy_status {
                connection: cursor.get_u8(),
                phy: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for phy_status {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u8(self.phy);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct rssi {
        pub connection: u8,
        pub status: u8,
        pub rssi: i8,
    }

    impl FromBytes for rssi {
        fn from_bytes(data: &[u8]) -> rssi {
            let mut cursor = Cursor::new(data);
            rssi {
                connection: cursor.get_u8(),
                status: cursor.get_u8(),
                rssi: cursor.get_i8(),
            }
        }
    }

    impl ToBytes for rssi {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u8(self.status);
            bytes.put_i8(self.rssi);
            bytes
        }
    }
}
