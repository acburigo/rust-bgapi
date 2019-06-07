extern crate bytes;
extern crate num_derive;
extern crate num_traits;
extern crate spmc;

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
    use parser;
    use spmc::Receiver;
    use std::io::{Error, ErrorKind};

    impl parser::Stream for Receiver<u8> {
        fn next(&self) -> Result<u8, Error> {
            match self.recv() {
                Ok(x) => Ok(x),
                Err(_) => Err(Error::from(ErrorKind::InvalidInput)),
            }
        }
    }

    #[test]
    fn message_header_from_bytes() {
        use message::{MessageClass, MessageHeader, MessageType};
        use parser::FromBytes;

        let data = [0x20, 0x02, 0x01, 0x00];
        let header = MessageHeader::from_bytes(&data);
        let expected_header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x00,
        };

        assert_eq!(header == expected_header, true);
    }

    #[test]
    fn message_header_to_bytes() {
        use message::{MessageClass, MessageHeader, MessageType};
        use parser::ToBytes;

        let header = MessageHeader {
            message_type: MessageType::command_response,
            payload_length: 0x02,
            message_class: MessageClass::system,
            message_id: 0x00,
        };

        assert_eq!(header.to_bytes(), vec![0x20, 0x02, 0x01, 0x00]);
    }

    #[test]
    fn rsp_system_hello_from_bytes() {
        use error;
        use parser::FromBytes;
        use system::rsp::hello;

        let data = [0x00, 0x00];
        let actual = hello::from_bytes(&data);
        let expected = hello {
            result: error::Error::success,
        };
        assert_eq!(actual == expected, true);
    }

    #[test]
    fn payload_cmd_system_hello_to_bytes() {
        use message::MessagePayload::cmd_system_hello;
        use parser::ToBytes;
        use system;

        let payload = cmd_system_hello(system::cmd::hello {});
        assert_eq!(payload.to_bytes(), vec![]);
    }

    #[test]
    fn message_rsp_system_hello_from_bytes() {
        use error;
        use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
        use parser::parse_next_message;
        use system;

        let (tx, rx) = spmc::channel();
        let bytes = [0x20, 0x02, 0x01, 0x00, 0x00, 0x00];
        for x in &bytes {
            tx.send(*x).unwrap();
        }
        let actual = parse_next_message(&rx).expect("Failed parsing message.");
        let expected = Message {
            header: MessageHeader {
                message_type: MessageType::command_response,
                payload_length: 0x02,
                message_class: MessageClass::system,
                message_id: 0x00,
            },
            payload: MessagePayload::rsp_system_hello(system::rsp::hello {
                result: error::Error::success,
            }),
        };

        assert_eq!(actual == expected, true);
    }

    #[test]
    fn message_cmd_system_hello_to_bytes() {
        use message::{Message, MessageClass, MessageHeader, MessagePayload, MessageType};
        use parser::ToBytes;
        use system;

        let msg = Message {
            header: MessageHeader {
                message_type: MessageType::command_response,
                payload_length: 0x00,
                message_class: MessageClass::system,
                message_id: 0x00,
            },
            payload: MessagePayload::cmd_system_hello(system::cmd::hello {}),
        };

        assert_eq!(msg.to_bytes(), vec![0x20, 0x00, 0x01, 0x00]);
    }

    #[test]
    fn new_message_cmd_system_hello_to_bytes() {
        use parser::ToBytes;
        use system::cmd::hello;
        assert_eq!(hello::new().to_bytes(), vec![0x20, 0x00, 0x01, 0x00]);
    }
}
