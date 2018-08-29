pub enum Command {
    message_to_target { data: Box<[u8]> },
}

pub enum Response {
    message_to_target { result: u16, data: Box<[u8]> },
}

pub enum Event {
    message_to_host { data: Box<[u8]> },
}
