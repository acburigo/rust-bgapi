use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x0c,
            message_id: 0x0c,
        } => Ok(MessagePayload::rsp_hardware_set_lazy_soft_timer(
            rsp::set_lazy_soft_timer::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x0c,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_hardware_set_soft_timer(
            rsp::set_soft_timer::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x01,
            message_class: 0x0c,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_hardware_soft_timer(
            evt::soft_timer::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    pub struct set_lazy_soft_timer {
        time: u32,
        slack: u32,
        handle: u8,
        single_shot: u8,
    }

    impl FromBytes for set_lazy_soft_timer {
        fn from_bytes(data: &[u8]) -> set_lazy_soft_timer {
            let mut cursor = Cursor::new(data);
            set_lazy_soft_timer {
                time: cursor.get_u32_le(),
                slack: cursor.get_u32_le(),
                handle: cursor.get_u8(),
                single_shot: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for set_lazy_soft_timer {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u32_le(self.time);
            bytes.put_u32_le(self.slack);
            bytes.put_u8(self.handle);
            bytes.put_u8(self.single_shot);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct set_soft_timer {
        time: u32,
        handle: u8,
        single_shot: u8,
    }

    impl FromBytes for set_soft_timer {
        fn from_bytes(data: &[u8]) -> set_soft_timer {
            let mut cursor = Cursor::new(data);
            set_soft_timer {
                time: cursor.get_u32_le(),
                handle: cursor.get_u8(),
                single_shot: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for set_soft_timer {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u32_le(self.time);
            bytes.put_u8(self.handle);
            bytes.put_u8(self.single_shot);
            bytes
        }
    }
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    pub struct set_lazy_soft_timer {
        result: u16,
    }

    impl FromBytes for set_lazy_soft_timer {
        fn from_bytes(data: &[u8]) -> set_lazy_soft_timer {
            let mut cursor = Cursor::new(data);
            set_lazy_soft_timer {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_lazy_soft_timer {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct set_soft_timer {
        result: u16,
    }

    impl FromBytes for set_soft_timer {
        fn from_bytes(data: &[u8]) -> set_soft_timer {
            let mut cursor = Cursor::new(data);
            set_soft_timer {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_soft_timer {
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
    pub struct soft_timer {
        handle: u8,
    }

    impl FromBytes for soft_timer {
        fn from_bytes(data: &[u8]) -> soft_timer {
            let mut cursor = Cursor::new(data);
            soft_timer {
                handle: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for soft_timer {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.handle);
            bytes
        }
    }
}
