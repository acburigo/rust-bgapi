use message::{MessageHeader, MessagePayload, MessageType, MessageClass};
use parser::FromBytes;
use std::io::{Error, ErrorKind};

pub fn parse(header: &MessageHeader, buffer: &[u8]) -> Result<MessagePayload, Error> {
    match header {
        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x0e,
        } => Ok(MessagePayload::rsp_sm_bonding_confirm(
            rsp::bonding_confirm::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x01,
        } => Ok(MessagePayload::rsp_sm_configure(
            rsp::configure::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x06,
        } => Ok(MessagePayload::rsp_sm_delete_bonding(
            rsp::delete_bonding::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x07,
        } => Ok(MessagePayload::rsp_sm_delete_bondings(
            rsp::delete_bondings::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x08,
        } => Ok(MessagePayload::rsp_sm_enter_passkey(
            rsp::enter_passkey::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x04,
        } => Ok(MessagePayload::rsp_sm_increase_security(
            rsp::increase_security::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x0b,
        } => Ok(MessagePayload::rsp_sm_list_all_bondings(
            rsp::list_all_bondings::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x09,
        } => Ok(MessagePayload::rsp_sm_passkey_confirm(
            rsp::passkey_confirm::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x00,
        } => Ok(MessagePayload::rsp_sm_set_bondable_mode(
            rsp::set_bondable_mode::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x0f,
        } => Ok(MessagePayload::rsp_sm_set_debug_mode(
            rsp::set_debug_mode::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x0a,
        } => Ok(MessagePayload::rsp_sm_set_oob_data(
            rsp::set_oob_data::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x10,
        } => Ok(MessagePayload::rsp_sm_set_passkey(
            rsp::set_passkey::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x12,
        } => Ok(MessagePayload::rsp_sm_set_sc_remote_oob_data(
            rsp::set_sc_remote_oob_data::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x02,
        } => Ok(MessagePayload::rsp_sm_store_bonding_configuration(
            rsp::store_bonding_configuration::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::command_response,
            payload_length: _,
            message_class: MessageClass::sm,
            message_id: 0x11,
        } => Ok(MessagePayload::rsp_sm_use_sc_oob(
            rsp::use_sc_oob::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x03,
        } => Ok(MessagePayload::evt_sm_bonded(evt::bonded::from_bytes(
            buffer,
        ))),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x03,
            message_class: MessageClass::sm,
            message_id: 0x04,
        } => Ok(MessagePayload::evt_sm_bonding_failed(
            evt::bonding_failed::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x09,
        } => Ok(MessagePayload::evt_sm_confirm_bonding(
            evt::confirm_bonding::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x05,
            message_class: MessageClass::sm,
            message_id: 0x02,
        } => Ok(MessagePayload::evt_sm_confirm_passkey(
            evt::confirm_passkey::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x00,
            message_class: MessageClass::sm,
            message_id: 0x06,
        } => Ok(MessagePayload::evt_sm_list_all_bondings_complete(
            evt::list_all_bondings_complete::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x08,
            message_class: MessageClass::sm,
            message_id: 0x05,
        } => Ok(MessagePayload::evt_sm_list_bonding_entry(
            evt::list_bonding_entry::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x05,
            message_class: MessageClass::sm,
            message_id: 0x00,
        } => Ok(MessagePayload::evt_sm_passkey_display(
            evt::passkey_display::from_bytes(buffer),
        )),

        MessageHeader {
            message_type: MessageType::event,
            payload_length: 0x01,
            message_class: MessageClass::sm,
            message_id: 0x01,
        } => Ok(MessagePayload::evt_sm_passkey_request(
            evt::passkey_request::from_bytes(buffer),
        )),

        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

#[allow(non_camel_case_types)]
pub enum BondingKey {
    ltk = 1,         // LTK saved in master
    addr_public = 2, // Public Address
    addr_static = 4, // Static Address
    irk = 8,         // Identity resolving key for resolvable private addresses
    edivrand = 16,   // EDIV+RAND received from slave
    csrk = 32,       // Connection signature resolving key
    masterid = 64,   // EDIV+RAND sent to master
}

#[allow(non_camel_case_types)]
pub enum io_capability {
    displayonly = 0,     // Display Only
    displayyesno = 1,    // Display with Yes/No-buttons
    keyboardonly = 2,    // Keyboard Only
    noinputnooutput = 3, // No Input and No Output
    keyboarddisplay = 4, // Display with Keyboard
}

pub mod cmd {
    use bytes::{Buf, BufMut};
    use parser::{FromBytes, ToBytes};
    use std::io::{Cursor, Read};

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct bonding_confirm {
        pub connection: u8,
        pub confirm: u8,
    }

    impl FromBytes for bonding_confirm {
        fn from_bytes(data: &[u8]) -> bonding_confirm {
            let mut cursor = Cursor::new(data);
            bonding_confirm {
                connection: cursor.get_u8(),
                confirm: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for bonding_confirm {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u8(self.confirm);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct configure {
        pub flags: u8,
        pub io_capabilities: u8,
    }

    impl FromBytes for configure {
        fn from_bytes(data: &[u8]) -> configure {
            let mut cursor = Cursor::new(data);
            configure {
                flags: cursor.get_u8(),
                io_capabilities: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for configure {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.flags);
            bytes.put_u8(self.io_capabilities);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct delete_bonding {
        pub bonding: u8,
    }

    impl FromBytes for delete_bonding {
        fn from_bytes(data: &[u8]) -> delete_bonding {
            let mut cursor = Cursor::new(data);
            delete_bonding {
                bonding: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for delete_bonding {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.bonding);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct delete_bondings {}

    impl FromBytes for delete_bondings {
        fn from_bytes(_: &[u8]) -> delete_bondings {
            delete_bondings {}
        }
    }

    impl ToBytes for delete_bondings {
        fn to_bytes(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct enter_passkey {
        pub connection: u8,
        pub passkey: i32,
    }

    impl FromBytes for enter_passkey {
        fn from_bytes(data: &[u8]) -> enter_passkey {
            let mut cursor = Cursor::new(data);
            enter_passkey {
                connection: cursor.get_u8(),
                passkey: cursor.get_i32_le(),
            }
        }
    }

    impl ToBytes for enter_passkey {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_i32_le(self.passkey);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct increase_security {
        pub connection: u8,
    }

    impl FromBytes for increase_security {
        fn from_bytes(data: &[u8]) -> increase_security {
            let mut cursor = Cursor::new(data);
            increase_security {
                connection: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for increase_security {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct list_all_bondings {}

    impl FromBytes for list_all_bondings {
        fn from_bytes(_: &[u8]) -> list_all_bondings {
            list_all_bondings {}
        }
    }

    impl ToBytes for list_all_bondings {
        fn to_bytes(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct passkey_confirm {
        pub connection: u8,
        pub confirm: u8,
    }

    impl FromBytes for passkey_confirm {
        fn from_bytes(data: &[u8]) -> passkey_confirm {
            let mut cursor = Cursor::new(data);
            passkey_confirm {
                connection: cursor.get_u8(),
                confirm: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for passkey_confirm {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u8(self.confirm);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_bondable_mode {
        pub bondable: u8,
    }

    impl FromBytes for set_bondable_mode {
        fn from_bytes(data: &[u8]) -> set_bondable_mode {
            let mut cursor = Cursor::new(data);
            set_bondable_mode {
                bondable: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for set_bondable_mode {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.bondable);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_debug_mode {}

    impl FromBytes for set_debug_mode {
        fn from_bytes(_: &[u8]) -> set_debug_mode {
            set_debug_mode {}
        }
    }

    impl ToBytes for set_debug_mode {
        fn to_bytes(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_oob_data {
        pub oob_data: Vec<u8>,
    }

    impl FromBytes for set_oob_data {
        fn from_bytes(data: &[u8]) -> set_oob_data {
            let mut cursor = Cursor::new(data);
            let mut oob_data = Vec::new();
            cursor
                .read_to_end(&mut oob_data)
                .expect("Failed to read bytes.");
            set_oob_data { oob_data }
        }
    }

    impl ToBytes for set_oob_data {
        fn to_bytes(&self) -> Vec<u8> {
            self.oob_data.clone()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_passkey {
        pub passkey: i32,
    }

    impl FromBytes for set_passkey {
        fn from_bytes(data: &[u8]) -> set_passkey {
            let mut cursor = Cursor::new(data);
            set_passkey {
                passkey: cursor.get_i32_le(),
            }
        }
    }

    impl ToBytes for set_passkey {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_i32_le(self.passkey);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_sc_remote_oob_data {
        pub oob_data: Vec<u8>,
    }

    impl FromBytes for set_sc_remote_oob_data {
        fn from_bytes(data: &[u8]) -> set_sc_remote_oob_data {
            let mut cursor = Cursor::new(data);
            let mut oob_data = Vec::new();
            cursor
                .read_to_end(&mut oob_data)
                .expect("Failed to read bytes.");
            set_sc_remote_oob_data { oob_data }
        }
    }

    impl ToBytes for set_sc_remote_oob_data {
        fn to_bytes(&self) -> Vec<u8> {
            self.oob_data.clone()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct store_bonding_configuration {
        pub max_bonding_count: u8,
        pub policy_flags: u8,
    }

    impl FromBytes for store_bonding_configuration {
        fn from_bytes(data: &[u8]) -> store_bonding_configuration {
            let mut cursor = Cursor::new(data);
            store_bonding_configuration {
                max_bonding_count: cursor.get_u8(),
                policy_flags: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for store_bonding_configuration {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.max_bonding_count);
            bytes.put_u8(self.policy_flags);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct use_sc_oob {
        pub enable: u8,
    }

    impl FromBytes for use_sc_oob {
        fn from_bytes(data: &[u8]) -> use_sc_oob {
            let mut cursor = Cursor::new(data);
            use_sc_oob {
                enable: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for use_sc_oob {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.enable);
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
    pub struct bonding_confirm {
        pub result: u16,
    }

    impl FromBytes for bonding_confirm {
        fn from_bytes(data: &[u8]) -> bonding_confirm {
            let mut cursor = Cursor::new(data);
            bonding_confirm {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for bonding_confirm {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct configure {
        pub result: u16,
    }

    impl FromBytes for configure {
        fn from_bytes(data: &[u8]) -> configure {
            let mut cursor = Cursor::new(data);
            configure {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for configure {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct delete_bonding {
        pub result: u16,
    }

    impl FromBytes for delete_bonding {
        fn from_bytes(data: &[u8]) -> delete_bonding {
            let mut cursor = Cursor::new(data);
            delete_bonding {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for delete_bonding {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct delete_bondings {
        pub result: u16,
    }

    impl FromBytes for delete_bondings {
        fn from_bytes(data: &[u8]) -> delete_bondings {
            let mut cursor = Cursor::new(data);
            delete_bondings {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for delete_bondings {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct enter_passkey {
        pub result: u16,
    }

    impl FromBytes for enter_passkey {
        fn from_bytes(data: &[u8]) -> enter_passkey {
            let mut cursor = Cursor::new(data);
            enter_passkey {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for enter_passkey {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct increase_security {
        pub result: u16,
    }

    impl FromBytes for increase_security {
        fn from_bytes(data: &[u8]) -> increase_security {
            let mut cursor = Cursor::new(data);
            increase_security {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for increase_security {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct list_all_bondings {
        pub result: u16,
    }

    impl FromBytes for list_all_bondings {
        fn from_bytes(data: &[u8]) -> list_all_bondings {
            let mut cursor = Cursor::new(data);
            list_all_bondings {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for list_all_bondings {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct passkey_confirm {
        pub result: u16,
    }

    impl FromBytes for passkey_confirm {
        fn from_bytes(data: &[u8]) -> passkey_confirm {
            let mut cursor = Cursor::new(data);
            passkey_confirm {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for passkey_confirm {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_bondable_mode {
        pub result: u16,
    }

    impl FromBytes for set_bondable_mode {
        fn from_bytes(data: &[u8]) -> set_bondable_mode {
            let mut cursor = Cursor::new(data);
            set_bondable_mode {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_bondable_mode {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_debug_mode {
        pub result: u16,
    }

    impl FromBytes for set_debug_mode {
        fn from_bytes(data: &[u8]) -> set_debug_mode {
            let mut cursor = Cursor::new(data);
            set_debug_mode {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_debug_mode {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_oob_data {
        pub result: u16,
    }

    impl FromBytes for set_oob_data {
        fn from_bytes(data: &[u8]) -> set_oob_data {
            let mut cursor = Cursor::new(data);
            set_oob_data {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_oob_data {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_passkey {
        pub result: u16,
    }

    impl FromBytes for set_passkey {
        fn from_bytes(data: &[u8]) -> set_passkey {
            let mut cursor = Cursor::new(data);
            set_passkey {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_passkey {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct set_sc_remote_oob_data {
        pub result: u16,
    }

    impl FromBytes for set_sc_remote_oob_data {
        fn from_bytes(data: &[u8]) -> set_sc_remote_oob_data {
            let mut cursor = Cursor::new(data);
            set_sc_remote_oob_data {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for set_sc_remote_oob_data {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct store_bonding_configuration {
        pub result: u16,
    }

    impl FromBytes for store_bonding_configuration {
        fn from_bytes(data: &[u8]) -> store_bonding_configuration {
            let mut cursor = Cursor::new(data);
            store_bonding_configuration {
                result: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for store_bonding_configuration {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct use_sc_oob {
        pub result: u16,
        pub oob_data: [u8; 32],
    }

    impl FromBytes for use_sc_oob {
        fn from_bytes(data: &[u8]) -> use_sc_oob {
            let mut cursor = Cursor::new(data);
            let result = cursor.get_u16_le();
            let mut oob_data: [u8; 32] = Default::default();
            cursor
                .read_exact(&mut oob_data)
                .expect("Failed to read bytes.");
            use_sc_oob { result, oob_data }
        }
    }

    impl ToBytes for use_sc_oob {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u16_le(self.result);
            bytes.extend_from_slice(&self.oob_data);
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
    pub struct bonded {
        pub connection: u8,
        pub bonding: u8,
    }

    impl FromBytes for bonded {
        fn from_bytes(data: &[u8]) -> bonded {
            let mut cursor = Cursor::new(data);
            bonded {
                connection: cursor.get_u8(),
                bonding: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for bonded {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u8(self.bonding);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct bonding_failed {
        pub connection: u8,
        pub reason: u16,
    }

    impl FromBytes for bonding_failed {
        fn from_bytes(data: &[u8]) -> bonding_failed {
            let mut cursor = Cursor::new(data);
            bonding_failed {
                connection: cursor.get_u8(),
                reason: cursor.get_u16_le(),
            }
        }
    }

    impl ToBytes for bonding_failed {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u16_le(self.reason);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct confirm_bonding {
        pub connection: u8,
        pub bonding_handle: i8,
    }

    impl FromBytes for confirm_bonding {
        fn from_bytes(data: &[u8]) -> confirm_bonding {
            let mut cursor = Cursor::new(data);
            confirm_bonding {
                connection: cursor.get_u8(),
                bonding_handle: cursor.get_i8(),
            }
        }
    }

    impl ToBytes for confirm_bonding {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_i8(self.bonding_handle);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct confirm_passkey {
        pub connection: u8,
        pub passkey: u32,
    }

    impl FromBytes for confirm_passkey {
        fn from_bytes(data: &[u8]) -> confirm_passkey {
            let mut cursor = Cursor::new(data);
            confirm_passkey {
                connection: cursor.get_u8(),
                passkey: cursor.get_u32_le(),
            }
        }
    }

    impl ToBytes for confirm_passkey {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u32_le(self.passkey);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct list_all_bondings_complete {}

    impl FromBytes for list_all_bondings_complete {
        fn from_bytes(_: &[u8]) -> list_all_bondings_complete {
            list_all_bondings_complete {}
        }
    }

    impl ToBytes for list_all_bondings_complete {
        fn to_bytes(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct list_bonding_entry {
        pub bonding: u8,
        pub address: [u8; 6],
        pub address_type: u8,
    }

    impl FromBytes for list_bonding_entry {
        fn from_bytes(data: &[u8]) -> list_bonding_entry {
            let mut cursor = Cursor::new(data);
            let bonding = cursor.get_u8();
            let mut address: [u8; 6] = Default::default();
            cursor
                .read_exact(&mut address)
                .expect("Failed to read bytes.");
            let address_type = cursor.get_u8();
            list_bonding_entry {
                bonding,
                address,
                address_type,
            }
        }
    }

    impl ToBytes for list_bonding_entry {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.bonding);
            bytes.extend_from_slice(&self.address);
            bytes.put_u8(self.address_type);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct passkey_display {
        pub connection: u8,
        pub passkey: u32,
    }

    impl FromBytes for passkey_display {
        fn from_bytes(data: &[u8]) -> passkey_display {
            let mut cursor = Cursor::new(data);
            passkey_display {
                connection: cursor.get_u8(),
                passkey: cursor.get_u32_le(),
            }
        }
    }

    impl ToBytes for passkey_display {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes.put_u32_le(self.passkey);
            bytes
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(PartialEq, PartialOrd)]
    pub struct passkey_request {
        pub connection: u8,
    }

    impl FromBytes for passkey_request {
        fn from_bytes(data: &[u8]) -> passkey_request {
            let mut cursor = Cursor::new(data);
            passkey_request {
                connection: cursor.get_u8(),
            }
        }
    }

    impl ToBytes for passkey_request {
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.put_u8(self.connection);
            bytes
        }
    }
}
