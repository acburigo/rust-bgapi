pub enum Command {
    set_lazy_soft_timer {
        time: u32,
        slack: u32,
        handle: u8,
        single_shot: u8,
    },
    set_soft_timer {
        time: u32,
        handle: u8,
        single_shot: u8,
    },
}

pub enum Response {
    set_lazy_soft_timer { result: u16 },
    set_soft_timer { result: u16 },
}

pub enum Event {
    soft_timer { handle: u8 },
}
