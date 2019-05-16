use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x08,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_test_dtm_end(rsp::dtm_end::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x0e,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_test_dtm_rx(rsp::dtm_rx::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x0e,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_test_dtm_tx(rsp::dtm_tx::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x04,
            message_class: 0x0e,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_test_dtm_completed(
            evt::dtm_completed::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
pub enum PacketType {
    pkt_prbs9 = 0,              // PRBS9 packet payload
    pkt_11110000 = 1,           // 11110000 packet payload
    pkt_10101010 = 2,           // 10101010 packet payload
    pkt_carrier_deprecated = 3, // Unmodulated carrier - deprecated
    pkt_11111111 = 4,           // 11111111 packet payload
    pkt_00000000 = 5,           // 00000000 packet payload
    pkt_00001111 = 6,           // 00001111 packet payload
    pkt_01010101 = 7,           // 01010101 packet payload
    pkt_pn9 = 253,              // PN9 continuously modulated output
    pkt_carrier = 254,          // Unmodulated carrier
}

#[allow(non_camel_case_types)]
pub enum Phy {
    phy_1m = 1,   // 1M PHY
    phy_2m = 2,   // 2M PHY
    phy_125k = 3, // 125k Coded PHY
    phy_500k = 4, // 500k Coded PHY Bluetooth Software API Reference Manual API Reference silabs.
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
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
    pub struct dtm_rx {
        channel: u8,
        phy: u8,
    }

    impl FromBytes for dtm_rx {
        fn from_bytes(data: &[u8]) -> dtm_rx {
            let mut cursor = Cursor::new(data);
            dtm_rx {
                channel: cursor.get_u8(),
                phy: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for dtm_rx {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.channel);
            bytes.put_u8(self.phy);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct dtm_tx {
        packet_type: u8,
        length: u8,
        channel: u8,
        phy: u8,
    }

    impl FromBytes for dtm_tx {
        fn from_bytes(data: &[u8]) -> dtm_tx {
            let mut cursor = Cursor::new(data);
            dtm_tx {
                packet_type: cursor.get_u8(),
                length: cursor.get_u8(),
                channel: cursor.get_u8(),
                phy: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for dtm_tx {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.packet_type);
            bytes.put_u8(self.length);
            bytes.put_u8(self.channel);
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
    pub struct dtm_end {
        result: u16,
    }

    impl FromBytes for dtm_end {
        fn from_bytes(data: &[u8]) -> dtm_end {
            let mut cursor = Cursor::new(data);
            dtm_end {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for dtm_end {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct dtm_rx {
        result: u16,
    }

    impl FromBytes for dtm_rx {
        fn from_bytes(data: &[u8]) -> dtm_rx {
            let mut cursor = Cursor::new(data);
            dtm_rx {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for dtm_rx {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct dtm_tx {
        result: u16,
    }

    impl FromBytes for dtm_tx {
        fn from_bytes(data: &[u8]) -> dtm_tx {
            let mut cursor = Cursor::new(data);
            dtm_tx {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for dtm_tx {
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
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    pub struct dtm_completed {
        result: u16,
        number_of_packets: u16,
    }

    impl FromBytes for dtm_completed {
        fn from_bytes(data: &[u8]) -> dtm_completed {
            let mut cursor = Cursor::new(data);
            dtm_completed {
                result: cursor.get_u16_le(),
                number_of_packets: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for dtm_completed {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.put_u16_le(self.number_of_packets);
            bytes
        }
    }
}
