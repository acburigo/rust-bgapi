pub enum Command {
    bonding_confirm {
        connection: u8,
        confirm: u8,
    },
    configure {
        flags: u8,
        io_capabilities: u8,
    },
    delete_bonding {
        bonding: u8,
    },
    delete_bondings,
    enter_passkey {
        connection: u8,
        passkey: i32,
    },
    increase_security {
        connection: u8,
    },
    list_all_bondings,
    passkey_confirm {
        connection: u8,
        confirm: u8,
    },
    set_bondable_mode {
        bondable: u8,
    },
    set_debug_mode,
    set_oob_data {
        oob_data: Box<[u8]>,
    },
    set_passkey {
        passkey: i32,
    },
    set_sc_remote_oob_data {
        oob_data: Box<[u8]>,
    },
    store_bonding_configuration {
        max_bonding_count: u8,
        policy_flags: u8,
    },
    use_sc_oob {
        enable: u8,
    },
}

pub enum Response {
    bonding_confirm { result: u16 },
    configure { result: u16 },
    delete_bonding { result: u16 },
    delete_bondings { result: u16 },
    enter_passkey { result: u16 },
    increase_security { result: u16 },
    list_all_bondings { result: u16 },
    passkey_confirm { result: u16 },
    set_bondable_mode { result: u16 },
    set_debug_mode { result: u16 },
    set_oob_data { result: u16 },
    set_passkey { result: u16 },
    set_sc_remote_oob_data { result: u16 },
    store_bonding_configuration { result: u16 },
    use_sc_oob { result: u16, oob_data: [u8; 32] },
}

pub enum Event {
    bonded {
        connection: u8,
        bonding: u8,
    },
    bonding_failed {
        connection: u8,
        reason: u16,
    },
    confirm_bonding {
        connection: u8,
        bonding_handle: i8,
    },
    confirm_passkey {
        connection: u8,
        passkey: u32,
    },
    list_all_bondings_complete,
    list_bonding_entry {
        bonding: u8,
        address: [u8; 6],
        address_type: u8,
    },
    passkey_display {
        connection: u8,
        passkey: u32,
    },
    passkey_request {
        connection: u8,
    },
}

pub enum BondingKey {
    ltk = 1,         // LTK saved in master
    addr_public = 2, // Public Address
    addr_static = 4, // Static Address
    irk = 8,         // Identity resolving key for resolvable private addresses
    edivrand = 16,   // EDIV+RAND received from slave
    csrk = 32,       // Connection signature resolving key
    masterid = 64,   // EDIV+RAND sent to master
}

pub enum io_capability {
    displayonly = 0,     // Display Only
    displayyesno = 1,    // Display with Yes/No-buttons
    keyboardonly = 2,    // Keyboard Only
    noinputnooutput = 3, // No Input and No Output
    keyboarddisplay = 4, // Display with Keyboard
}
