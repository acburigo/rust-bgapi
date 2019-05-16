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

// pub enum Packet {
//     Command(Command),
//     Response(Response),
//     Event(Event),
// }

// impl Packet {
//     pub fn from_binary(cursor: &mut Cursor<&[u8]>) -> Result<Packet, Error> {
//         let header = MessageHeader::from_binary(cursor)?;

//         if cursor.remaining() < header.payload_length as usize {
//             return Err(Error::from(ErrorKind::UnexpectedEof));
//         }

//         match header.message_type {
//             t if t == MessageType::CommandResponse as u8 => {
//                 let response = Response::from_binary(cursor, &header)?;
//                 let packet = Packet::Response(response);
//                 Ok(packet)
//             }
//             t if t == MessageType::Event as u8 => {
//                 let event = Event::from_binary(cursor, &header)?;
//                 let packet = Packet::Event(event);
//                 Ok(packet)
//             }
//             _ => Err(Error::from(ErrorKind::InvalidInput)),
//         }
//     }
// }

// pub enum Command {
//     coex(coex::Command),
//     dfu(dfu::Command),
//     flash(flash::Command),
//     gatt(gatt::Command),
//     gatt_server(gatt_server::Command),
//     hardware(hardware::Command),
//     le_connection(le_connection::Command),
//     le_gap(le_gap::Command),
//     sm(sm::Command),
//     system(system::Command),
//     test(test::Command),
//     user(user::Command),
// }

// pub enum Response {
//     coex(coex::Response),
//     dfu(dfu::Response),
//     flash(flash::Response),
//     gatt(gatt::Response),
//     gatt_server(gatt_server::Response),
//     hardware(hardware::Response),
//     le_connection(le_connection::Response),
//     le_gap(le_gap::Response),
//     sm(sm::Response),
//     system(system::Response),
//     test(test::Response),
//     user(user::Response),
// }

// impl Response {
//     pub fn from_binary(
//         cursor: &mut Cursor<&[u8]>,
//         header: &MessageHeader,
//     ) -> Result<Response, Error> {
//         match header.message_class {
//             c if c == MessageClass::coex as u8 => {
//                 let response = coex::Response::from_binary(cursor, &header)?;
//                 let packet = Response::coex(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::dfu as u8 => {
//                 let response = dfu::Response::from_binary(cursor, &header)?;
//                 let packet = Response::dfu(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::flash as u8 => {
//                 let response = flash::Response::from_binary(cursor, &header)?;
//                 let packet = Response::flash(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::gatt as u8 => {
//                 let response = gatt::Response::from_binary(cursor, &header)?;
//                 let packet = Response::gatt(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::gatt_server as u8 => {
//                 let response = gatt_server::Response::from_binary(cursor, &header)?;
//                 let packet = Response::gatt_server(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::hardware as u8 => {
//                 let response = hardware::Response::from_binary(cursor, &header)?;
//                 let packet = Response::hardware(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::le_connection as u8 => {
//                 let response = le_connection::Response::from_binary(cursor, &header)?;
//                 let packet = Response::le_connection(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::le_gap as u8 => {
//                 let response = le_gap::Response::from_binary(cursor, &header)?;
//                 let packet = Response::le_gap(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::sm as u8 => {
//                 let response = sm::Response::from_binary(cursor, &header)?;
//                 let packet = Response::sm(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::system as u8 => {
//                 let response = system::Response::from_binary(cursor, &header)?;
//                 let packet = Response::system(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::test as u8 => {
//                 let response = test::Response::from_binary(cursor, &header)?;
//                 let packet = Response::test(response);
//                 Ok(packet)
//             }
//             c if c == MessageClass::user as u8 => {
//                 let response = user::Response::from_binary(cursor, &header)?;
//                 let packet = Response::user(response);
//                 Ok(packet)
//             }
//             _ => Err(Error::from(ErrorKind::InvalidInput)),
//         }
//     }
// }

// pub enum Event {
//     dfu(dfu::Event),
//     gatt(gatt::Event),
//     gatt_server(gatt_server::Event),
//     hardware(hardware::Event),
//     le_connection(le_connection::Event),
//     le_gap(le_gap::Event),
//     sm(sm::Event),
//     system(system::Event),
//     test(test::Event),
//     user(user::Event),
// }

// impl Event {
//     pub fn from_binary(cursor: &mut Cursor<&[u8]>, header: &MessageHeader) -> Result<Event, Error> {
//         match header.message_class {
//             c if c == MessageClass::dfu as u8 => {
//                 let event = dfu::Event::from_binary(cursor, &header)?;
//                 let packet = Event::dfu(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::gatt as u8 => {
//                 let event = gatt::Event::from_binary(cursor, &header)?;
//                 let packet = Event::gatt(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::gatt_server as u8 => {
//                 let event = gatt_server::Event::from_binary(cursor, &header)?;
//                 let packet = Event::gatt_server(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::hardware as u8 => {
//                 let event = hardware::Event::from_binary(cursor, &header)?;
//                 let packet = Event::hardware(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::le_connection as u8 => {
//                 let event = le_connection::Event::from_binary(cursor, &header)?;
//                 let packet = Event::le_connection(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::le_gap as u8 => {
//                 let event = le_gap::Event::from_binary(cursor, &header)?;
//                 let packet = Event::le_gap(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::sm as u8 => {
//                 let event = sm::Event::from_binary(cursor, &header)?;
//                 let packet = Event::sm(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::system as u8 => {
//                 let event = system::Event::from_binary(cursor, &header)?;
//                 let packet = Event::system(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::test as u8 => {
//                 let event = test::Event::from_binary(cursor, &header)?;
//                 let packet = Event::test(event);
//                 Ok(packet)
//             }
//             c if c == MessageClass::user as u8 => {
//                 let event = user::Event::from_binary(cursor, &header)?;
//                 let packet = Event::user(event);
//                 Ok(packet)
//             }
//             _ => Err(Error::from(ErrorKind::InvalidInput)),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
