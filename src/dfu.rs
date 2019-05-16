use message::{MessageHeader, MessagePayload};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x00,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_dfu_flash_set_address(
            rsp::flash_set_address::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x00,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_dfu_flash_upload(
            rsp::flash_upload::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x00,
            message_id: 0x03,
        } => Ok(MessagePayload::rsp_dfu_flash_upload_finish(
            rsp::flash_upload_finish::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x04,
            message_class: 0x00,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_dfu_boot(evt::boot::from_bytes(buffer))),

        MessageHeader {
            message_type: 0xa0,
            payload_length: 0x02,
            message_class: 0x00,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_dfu_boot_failure(
            evt::boot_failure::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    pub struct flash_set_address {
        address: u32,
    }

    impl FromBytes for flash_set_address {
        fn from_bytes(data: &[u8]) -> flash_set_address {
            let mut cursor = Cursor::new(data);
            flash_set_address {
                address: cursor.get_u32_le(),
            }
        }
    }

    impl ToBytes for flash_set_address {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u32_le(self.address);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct flash_upload {
        data: Vec<u8>,
    }

    impl FromBytes for flash_upload {
        fn from_bytes(data: &[u8]) -> flash_upload {
            flash_upload {
                data: data.to_vec(),
            }
        }
    }

    impl ToBytes for flash_upload {
        fn to_bytes(&self) -> Vec<u8> {
            self.data.clone()
        }
    }

    #[allow(non_camel_case_types)]
    pub struct flash_upload_finish {}

    impl FromBytes for flash_upload_finish {
        fn from_bytes(_: &[u8]) -> flash_upload_finish {
            flash_upload_finish {}
        }
    }

    impl ToBytes for flash_upload_finish {
        fn to_bytes(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    pub struct reset {
        dfu: u8,
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
}

pub mod rsp {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::Cursor;

    #[allow(non_camel_case_types)]
    pub struct flash_set_address {
        result: u16,
    }

    impl FromBytes for flash_set_address {
        fn from_bytes(data: &[u8]) -> flash_set_address {
            let mut cursor = Cursor::new(data);
            flash_set_address {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for flash_set_address {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct flash_upload {
        result: u16,
    }

    impl FromBytes for flash_upload {
        fn from_bytes(data: &[u8]) -> flash_upload {
            let mut cursor = Cursor::new(data);
            flash_upload {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for flash_upload {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct flash_upload_finish {
        result: u16,
    }

    impl FromBytes for flash_upload_finish {
        fn from_bytes(data: &[u8]) -> flash_upload_finish {
            let mut cursor = Cursor::new(data);
            flash_upload_finish {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for flash_upload_finish {
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
    pub struct boot {
        version: u32,
    }

    impl FromBytes for boot {
        fn from_bytes(data: &[u8]) -> boot {
            let mut cursor = Cursor::new(data);
            boot {
                version: cursor.get_u32_le(),
            }
        }
    }

    impl ToBytes for boot {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u32_le(self.version);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    pub struct boot_failure {
        reason: u16,
    }

    impl FromBytes for boot_failure {
        fn from_bytes(data: &[u8]) -> boot_failure {
            let mut cursor = Cursor::new(data);
            boot_failure {
                reason: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for boot_failure {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.reason);
            bytes
        }
    }
}
