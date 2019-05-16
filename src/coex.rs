use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: 0x20,
            payload_length: 0x03,
            message_class: 0x20,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_coex_get_counters(
            rsp::get_counters::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x20,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_coex_set_options(
            rsp::set_options::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
pub enum Option {
    enable = 256,         // Enable coexistence feature
    tx_abort = 1024,      // Abort transmission if grant is denied
    high_priority = 2048, // Enable priority signal
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    pub struct get_counters {
        reset: u8,
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
    pub struct set_options {
        mask: u32,
        options: u32,
    }

    impl FromBytes for set_options {
        fn from_bytes(data: &[u8]) -> set_options {
            let mut cursor = Cursor::new(data);
            set_options {
                mask: cursor.get_u32_le(),
                options: cursor.get_u32_le(),
            }
        }
    }

    impl ToBytes for set_options {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u32_le(self.mask);
            bytes.put_u32_le(self.options);
            bytes
        }
    }
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use std::io::Cursor;

    use parser::{FromBytes, ToBytes};

    #[allow(non_camel_case_types)]
    pub struct get_counters {
        result: u16,
        counters: Box<[u8]>,
    }

    impl FromBytes for get_counters {
        fn from_bytes(data: &[u8]) -> get_counters {
            let mut cursor = Cursor::new(data);
            get_counters {
                result: cursor.get_u16_le(),
                counters: cursor.bytes().to_vec().into_boxed_slice(),
            }
        }
    }

    impl ToBytes for get_counters {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes: Vec<u8> = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.extend_from_slice(&self.counters);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct set_options {
        result: u16,
    }

    impl FromBytes for set_options {
        fn from_bytes(data: &[u8]) -> set_options {
            let mut cursor = Cursor::new(data);
            set_options {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_options {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes: Vec<u8> = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }
}
