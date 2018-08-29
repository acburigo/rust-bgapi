pub enum Command {
    get_counters { reset: u8 },
    set_options { mask: u32, options: u32 },
}

pub enum Response {
    get_counters { result: u16, counters: Box<[u8]> },
    set_options { result: u16 },
}

pub enum Option {
    enable = 256,         // Enable coexistence feature
    tx_abort = 1024,      // Abort transmission if grant is denied
    high_priority = 2048, // Enable priority signal
}

// fn cmd_coex_get_counters(reset: u8) -> [u8] {}