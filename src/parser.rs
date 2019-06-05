use coex;
use dfu;
use flash;
use gatt;
use gatt_server;
use hardware;
use le_connection;
use le_gap;
use message::{Message, MessageClass, MessageHeader};
use sm;
use std::io::{Error, ErrorKind};
use system;
use test;
use user;

pub trait Stream {
    fn next(&self) -> Result<u8, Error>;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait FromBytes {
    fn from_bytes(&[u8]) -> Self;
}

pub fn parse_next_message(stream: &Stream) -> Result<Message, Error> {
    let mut buffer = Vec::new();

    while buffer.len() < MessageHeader::size() {
        buffer.push(stream.next()?);
    }

    let header = MessageHeader::from_bytes(buffer.as_slice());

    buffer.clear();

    while buffer.len() < header.payload_length as usize {
        buffer.push(stream.next()?);
    }

    let payload = match header.message_class {
        MessageClass::coex => coex::parse(&header, buffer.as_slice())?,
        MessageClass::dfu => dfu::parse(&header, buffer.as_slice())?,
        MessageClass::flash => flash::parse(&header, buffer.as_slice())?,
        MessageClass::gatt => gatt::parse(&header, buffer.as_slice())?,
        MessageClass::gatt_server => gatt_server::parse(&header, buffer.as_slice())?,
        MessageClass::hardware => hardware::parse(&header, buffer.as_slice())?,
        MessageClass::le_connection => le_connection::parse(&header, buffer.as_slice())?,
        MessageClass::le_gap => le_gap::parse(&header, buffer.as_slice())?,
        MessageClass::sm => sm::parse(&header, buffer.as_slice())?,
        MessageClass::system => system::parse(&header, buffer.as_slice())?,
        MessageClass::test => test::parse(&header, buffer.as_slice())?,
        MessageClass::user => user::parse(&header, buffer.as_slice())?,
        _ => return Err(Error::from(ErrorKind::NotFound)),
    };

    Ok(Message { header, payload })
}
