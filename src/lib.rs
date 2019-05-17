extern crate bytes;

pub mod coex;
pub mod dfu;
pub mod error;
pub mod flash;
pub mod gatt;
pub mod gatt_server;
pub mod hardware;
pub mod le_connection;
pub mod le_gap;
pub mod message;
pub mod parser;
pub mod sm;
pub mod system;
pub mod test;
pub mod user;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn message_header_from_bytes() {
        use message::MessageHeader;
        use parser::FromBytes;
        let data = [0x20, 0x02, 0x01, 0x00];
        let header = MessageHeader::from_bytes(&data);
        let expected_header = MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x01,
            message_id: 0x00,
        };

        assert_eq!(header == expected_header, true);
    }

    #[test]
    fn message_header_to_bytes() {
        use message::MessageHeader;
        use parser::ToBytes;

        let header = MessageHeader {
            message_type: 0x20,
            payload_length: 0x02,
            message_class: 0x01,
            message_id: 0x00,
        };

        assert_eq!(header.to_bytes(), vec![0x20, 0x02, 0x01, 0x00]);
    }

    #[test]
    fn message_payload_to_bytes() {
        use message::MessagePayload;
        use parser::ToBytes;
        use system;

        let payload = MessagePayload::cmd_system_hello(system::cmd::hello {});

        assert_eq!(payload.to_bytes(), vec![]);
    }

    #[test]
    fn message_to_bytes() {
        use message::{Message, MessageHeader, MessagePayload};
        use parser::ToBytes;
        use system;

        let msg = Message {
            header: MessageHeader {
                message_type: 0x20,
                payload_length: 0x02,
                message_class: 0x01,
                message_id: 0x00,
            },
            payload: MessagePayload::cmd_system_hello(system::cmd::hello {}),
        };

        assert_eq!(msg.to_bytes(), vec![0x20, 0x02, 0x01, 0x00]);
    }
}
