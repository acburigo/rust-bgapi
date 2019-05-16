use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: 0x20,
            payload_length: _,
            message_class: 0xff,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_user_message_to_target(
            rsp::message_to_target::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: _,
            message_class: 0xff,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_user_message_to_host(
            evt::message_to_host::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub mod cmd {
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct message_to_target {
        data: Vec<u8>,
    }

    impl FromBytes for message_to_target {
        fn from_bytes(data: &[u8]) -> message_to_target {
            let mut cursor = Cursor::new(data);
            let mut data = Vec::new();
            cursor
                .read_to_end(&mut data)
                .expect("Failed to read bytes.");
            message_to_target { data }
        }
    }

    impl ToBytes for message_to_target {
        fn to_bytes(&self) -> Vec<u8> {
            self.data.clone()
        }
    }
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct message_to_target {
        result: u16,
        data: Vec<u8>,
    }

    impl FromBytes for message_to_target {
        fn from_bytes(data: &[u8]) -> message_to_target {
            let mut cursor = Cursor::new(data);
            let result = cursor.get_u16_le();
            let mut data = Vec::new();
            cursor
                .read_to_end(&mut data)
                .expect("Failed to read bytes.");
            message_to_target { result, data }
        }
    }

    impl ToBytes for message_to_target {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.extend(self.data.iter());
            bytes
        }
    }
}

pub mod evt {
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    pub struct message_to_host {
        data: Vec<u8>,
    }

    impl FromBytes for message_to_host {
        fn from_bytes(data: &[u8]) -> message_to_host {
            let mut cursor = Cursor::new(data);
            let mut data = Vec::new();
            cursor
                .read_to_end(&mut data)
                .expect("Failed to read bytes.");
            message_to_host { data }
        }
    }

    impl ToBytes for message_to_host {
        fn to_bytes(&self) -> Vec<u8> {
            self.data.clone()
        }
    }
}
