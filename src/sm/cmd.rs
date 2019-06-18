use bytes::{Buf, BufMut};
use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
use std::io::{Cursor, Read};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct bonding_confirm {
    pub connection: u8,
    pub confirm: u8,
}

impl bonding_confirm {
    pub fn new(connection: u8, confirm: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x0e,
        };
        let payload = bonding_confirm {
            connection,
            confirm,
        };
        let payload = MessagePayload::cmd_sm_bonding_confirm(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for bonding_confirm {
    fn from(data: &[u8]) -> bonding_confirm {
        let mut cursor = Cursor::new(data);
        bonding_confirm {
            connection: cursor.get_u8(),
            confirm: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for bonding_confirm {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.confirm);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct configure {
    pub flags: u8,
    pub io_capabilities: u8,
}

impl configure {
    pub fn new(flags: u8, io_capabilities: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x01,
        };
        let payload = configure {
            flags,
            io_capabilities,
        };
        let payload = MessagePayload::cmd_sm_configure(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for configure {
    fn from(data: &[u8]) -> configure {
        let mut cursor = Cursor::new(data);
        configure {
            flags: cursor.get_u8(),
            io_capabilities: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for configure {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.flags);
        bytes.put_u8(self.io_capabilities);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct delete_bonding {
    pub bonding: u8,
}

impl delete_bonding {
    pub fn new(bonding: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::sm,
            message_id: 0x06,
        };
        let payload = delete_bonding { bonding };
        let payload = MessagePayload::cmd_sm_delete_bonding(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for delete_bonding {
    fn from(data: &[u8]) -> delete_bonding {
        let mut cursor = Cursor::new(data);
        delete_bonding {
            bonding: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for delete_bonding {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.bonding);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct delete_bondings {}

impl delete_bondings {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::sm,
            message_id: 0x07,
        };
        let payload = delete_bondings {};
        let payload = MessagePayload::cmd_sm_delete_bondings(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for delete_bondings {
    fn from(_: &[u8]) -> delete_bondings {
        delete_bondings {}
    }
}

impl Into<Vec<u8>> for delete_bondings {
    fn into(self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct enter_passkey {
    pub connection: u8,
    pub passkey: i32,
}

impl enter_passkey {
    pub fn new(connection: u8, passkey: i32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x05,
            message_class: MessageClass::sm,
            message_id: 0x08,
        };
        let payload = enter_passkey {
            connection,
            passkey,
        };
        let payload = MessagePayload::cmd_sm_enter_passkey(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for enter_passkey {
    fn from(data: &[u8]) -> enter_passkey {
        let mut cursor = Cursor::new(data);
        enter_passkey {
            connection: cursor.get_u8(),
            passkey: cursor.get_i32_le(),
        }
    }
}

impl Into<Vec<u8>> for enter_passkey {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_i32_le(self.passkey);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct increase_security {
    pub connection: u8,
}

impl increase_security {
    pub fn new(connection: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::sm,
            message_id: 0x04,
        };
        let payload = increase_security { connection };
        let payload = MessagePayload::cmd_sm_increase_security(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for increase_security {
    fn from(data: &[u8]) -> increase_security {
        let mut cursor = Cursor::new(data);
        increase_security {
            connection: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for increase_security {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct list_all_bondings {}

impl list_all_bondings {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::sm,
            message_id: 0x0b,
        };
        let payload = list_all_bondings {};
        let payload = MessagePayload::cmd_sm_list_all_bondings(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for list_all_bondings {
    fn from(_: &[u8]) -> list_all_bondings {
        list_all_bondings {}
    }
}

impl Into<Vec<u8>> for list_all_bondings {
    fn into(self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct passkey_confirm {
    pub connection: u8,
    pub confirm: u8,
}

impl passkey_confirm {
    pub fn new(connection: u8, confirm: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x09,
        };
        let payload = passkey_confirm {
            connection,
            confirm,
        };
        let payload = MessagePayload::cmd_sm_passkey_confirm(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for passkey_confirm {
    fn from(data: &[u8]) -> passkey_confirm {
        let mut cursor = Cursor::new(data);
        passkey_confirm {
            connection: cursor.get_u8(),
            confirm: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for passkey_confirm {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.connection);
        bytes.put_u8(self.confirm);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_bondable_mode {
    pub bondable: u8,
}

impl set_bondable_mode {
    pub fn new(bondable: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::sm,
            message_id: 0x00,
        };
        let payload = set_bondable_mode { bondable };
        let payload = MessagePayload::cmd_sm_set_bondable_mode(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_bondable_mode {
    fn from(data: &[u8]) -> set_bondable_mode {
        let mut cursor = Cursor::new(data);
        set_bondable_mode {
            bondable: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for set_bondable_mode {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.bondable);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_debug_mode {}

impl set_debug_mode {
    pub fn new() -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x00,
            message_class: MessageClass::sm,
            message_id: 0x0f,
        };
        let payload = set_debug_mode {};
        let payload = MessagePayload::cmd_sm_set_debug_mode(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_debug_mode {
    fn from(_: &[u8]) -> set_debug_mode {
        set_debug_mode {}
    }
}

impl Into<Vec<u8>> for set_debug_mode {
    fn into(self) -> Vec<u8> {
        Vec::new()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_oob_data {
    pub oob_data: Vec<u8>,
}

impl set_oob_data {
    pub fn new(oob_data: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: oob_data.len() as u8,
            message_class: MessageClass::sm,
            message_id: 0x0a,
        };
        let payload = set_oob_data { oob_data };
        let payload = MessagePayload::cmd_sm_set_oob_data(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_oob_data {
    fn from(data: &[u8]) -> set_oob_data {
        let mut cursor = Cursor::new(data);
        let mut oob_data = Vec::new();
        cursor
            .read_to_end(&mut oob_data)
            .expect("Failed to read bytes.");
        set_oob_data { oob_data }
    }
}

impl Into<Vec<u8>> for set_oob_data {
    fn into(self) -> Vec<u8> {
        self.oob_data.clone()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_passkey {
    pub passkey: i32,
}

impl set_passkey {
    pub fn new(passkey: i32) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x04,
            message_class: MessageClass::sm,
            message_id: 0x10,
        };
        let payload = set_passkey { passkey };
        let payload = MessagePayload::cmd_sm_set_passkey(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_passkey {
    fn from(data: &[u8]) -> set_passkey {
        let mut cursor = Cursor::new(data);
        set_passkey {
            passkey: cursor.get_i32_le(),
        }
    }
}

impl Into<Vec<u8>> for set_passkey {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_i32_le(self.passkey);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct set_sc_remote_oob_data {
    pub oob_data: Vec<u8>,
}

impl set_sc_remote_oob_data {
    pub fn new(oob_data: Vec<u8>) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: oob_data.len() as u8,
            message_class: MessageClass::sm,
            message_id: 0x12,
        };
        let payload = set_sc_remote_oob_data { oob_data };
        let payload = MessagePayload::cmd_sm_set_sc_remote_oob_data(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for set_sc_remote_oob_data {
    fn from(data: &[u8]) -> set_sc_remote_oob_data {
        let mut cursor = Cursor::new(data);
        let mut oob_data = Vec::new();
        cursor
            .read_to_end(&mut oob_data)
            .expect("Failed to read bytes.");
        set_sc_remote_oob_data { oob_data }
    }
}

impl Into<Vec<u8>> for set_sc_remote_oob_data {
    fn into(self) -> Vec<u8> {
        self.oob_data.clone()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct store_bonding_configuration {
    pub max_bonding_count: u8,
    pub policy_flags: u8,
}

impl store_bonding_configuration {
    pub fn new(max_bonding_count: u8, policy_flags: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::sm,
            message_id: 0x02,
        };
        let payload = store_bonding_configuration {
            max_bonding_count,
            policy_flags,
        };
        let payload = MessagePayload::cmd_sm_store_bonding_configuration(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for store_bonding_configuration {
    fn from(data: &[u8]) -> store_bonding_configuration {
        let mut cursor = Cursor::new(data);
        store_bonding_configuration {
            max_bonding_count: cursor.get_u8(),
            policy_flags: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for store_bonding_configuration {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.max_bonding_count);
        bytes.put_u8(self.policy_flags);
        bytes
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct use_sc_oob {
    pub enable: u8,
}

impl use_sc_oob {
    pub fn new(enable: u8) -> Message {
        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x01,
            message_class: MessageClass::sm,
            message_id: 0x11,
        };
        let payload = use_sc_oob { enable };
        let payload = MessagePayload::cmd_sm_use_sc_oob(payload);
        Message { header, payload }
    }
}

impl From<&[u8]> for use_sc_oob {
    fn from(data: &[u8]) -> use_sc_oob {
        let mut cursor = Cursor::new(data);
        use_sc_oob {
            enable: cursor.get_u8(),
        }
    }
}

impl Into<Vec<u8>> for use_sc_oob {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.enable);
        bytes
    }
}
