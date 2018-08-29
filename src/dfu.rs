pub enum Command {
    flash_set_address { address: u32 },
    flash_upload { data: Box<[u8]> },
    flash_upload_finish,
    reset { dfu: u8 },
}

pub enum Response {
    flash_set_address { result: u16 },
    flash_upload { result: u16 },
    flash_upload_finish { result: u16 },
}

pub enum Event {
    boot { version: u32 },
    boot_failure { reason: u16 },
}
