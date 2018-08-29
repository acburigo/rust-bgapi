pub mod coex;
pub mod dfu;
pub mod error;
pub mod flash;
pub mod gatt;
pub mod gatt_server;
pub mod hardware;
pub mod le_connection;
pub mod le_gap;
pub mod sm;
pub mod system;
pub mod test;
pub mod user;

pub enum MessageType {
    CommandResponse = 0x20,
    Event = 0xa0,
}

pub enum MessageClass {
    coex = 0x20,
    dfu = 0x00,
    Endpoint = 0x0b,
    flash = 0x0d,
    gatt = 0x09,
    gatt_server = 0x0a,
    hardware = 0x0c,
    le_connection = 0x08,
    le_gap = 0x03,
    sm = 0x0f,
    system = 0x01,
    test = 0x0e,
    user = 0xff,
}

pub struct MessageHeader {
    message_type: MessageType,
    payload_length: u8,
    message_class: MessageClass,
    message_id: u8,
}

// impl MessageHeader {
//     fn parse_next(buffer: &[u8]) -> MessageHeader {
//         MessageHeader {
//             message_type: MessageType::from_u8(buffer[0]),
//             payload_length: buffer[1],
//             message_class = buffer[2],
//             message_id = buffer[3],
//         }
//     }
// }

enum Packet {
    Command(Command),
    Response(Response),
    Event(Event),
}

enum Command {
    coex(coex::Command),
    dfu(dfu::Command),
    flash(flash::Command),
    gatt(gatt::Command),
    gatt_server(gatt_server::Command),
    hardware(hardware::Command),
    le_connection(le_connection::Command),
    le_gap(le_gap::Command),
    sm(sm::Command),
    system(system::Command),
    test(test::Command),
    user(user::Command),
}

enum Response {
    coex(coex::Response),
    dfu(dfu::Response),
    flash(flash::Response),
    gatt(gatt::Response),
    gatt_server(gatt_server::Response),
    hardware(hardware::Response),
    le_connection(le_connection::Response),
    le_gap(le_gap::Response),
    sm(sm::Response),
    system(system::Response),
    test(test::Response),
    user(user::Response),
}

enum Event {
    Dfu(dfu::Event),
    gatt(gatt::Event),
    gatt_server(gatt_server::Event),
    hardware(hardware::Event),
    le_connection(le_connection::Event),
    le_gap(le_gap::Event),
    sm(sm::Event),
    system(system::Event),
    test(test::Event),
    user(user::Event),
}

// pub struct Message {
//     header: MessageHeader,
//     payload: MessagePayload,
// }

// impl Message {
//     pub fn new(header: MessageHeader, payload: MessagePayload) -> Message {
//         Message { header, payload }
//     }
// }

// fn from_binary(buffer: &Vec<u8>) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
