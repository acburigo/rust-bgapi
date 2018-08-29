pub enum Command {
    ps_erase { key: u16 },
    ps_erase_all,
    ps_load { key: u16 },
    ps_save { key: u16, value: Box<[u8]> },
}

pub enum Response {
    ps_erase { result: u16 },
    ps_erase_all { result: u16 },
    ps_load { result: u16, value: Box<[u8]> },
    ps_save { result: u16 },
}
