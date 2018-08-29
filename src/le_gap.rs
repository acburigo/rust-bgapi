pub enum Command {
    bt5_set_adv_data {
        handle: u8,
        scan_rsp: u8,
        adv_data: Box<[u8]>,
    },
    clear_advertise_configuration {
        handle: u8,
        configurations: u32,
    },
    connect {
        address: [u8; 6],
        address_type: u8,
        initiating_phy: u8,
    },
    end_procedure,
    set_advertise_channel_map {
        handle: u8,
        channel_map: u8,
    },
    set_advertise_configuration {
        handle: u8,
        configurations: u32,
    },
    set_advertise_phy {
        handle: u8,
        primary_phy: u8,
        secondary_phy: u8,
    },
    set_advertise_report_scan_request {
        handle: u8,
        report_scan_req: u8,
    },
    set_advertise_timing {
        handle: u8,
        interval_min: u32,
        interval_max: u32,
        duration: u16,
        maxevents: u8,
    },
    set_advertise_tx_power {
        handle: u8,
        power: i16,
    },
    set_conn_parameters {
        min_interval: u16,
        max_interval: u16,
        latency: u16,
        timeout: u16,
    },
    set_data_channel_classification {
        channel_map: [u8; 5],
    },
    set_discovery_timing {
        phys: u8,
        scan_interval: u16,
        scan_window: u16,
    },
    set_discovery_type {
        phys: u8,
        scan_type: u8,
    },
    set_privacy_mode {
        privacy: u8,
        interval: u8,
    },
    start_advertising {
        handle: u8,
        discover: u8,
        connect: u8,
    },
    start_discovery {
        scanning_phy: u8,
        mode: u8,
    },
    stop_advertising {
        handle: u8,
    },
}

pub enum Response {
    bt5_set_adv_data { result: u16 },
    clear_advertise_configuration { result: u16 },
    connect { result: u16, connection: u8 },
    end_procedure { result: u16 },
    set_advertise_channel_map { result: u16 },
    set_advertise_configuration { result: u16 },
    set_advertise_phy { result: u16 },
    set_advertise_report_scan_request { result: u16 },
    set_advertise_timing { result: u16 },
    set_advertise_tx_power { result: u16 },
    set_conn_parameters { result: u16 },
    set_data_channel_classification { result: u16 },
    set_discovery_timing { result: u16 },
    set_discovery_type { result: u16 },
    set_privacy_mode { result: u16 },
    start_advertising { result: u16 },
    start_discovery { result: u16 },
    stop_advertising { result: u16 },
}

pub enum Event {
    adv_timeout {
        handle: u8,
    },
    scan_request {
        handle: u8,
        address: [u8; 6],
        address_type: u8,
        bonding: u8,
    },
    scan_response {
        rssi: i8,
        packet_type: u8,
        address: [u8; 6],
        address_type: u8,
        bonding: u8,
        data: Box<[u8]>,
    },
}

pub enum AddressType {
    public = 0,          // Public address
    random = 1,          // Random address
    public_identity = 2, // Public identity address resolved by stack
    random_identity = 3, // Random identity address resolved by stack
}

pub enum AdvAddressType {
    identity_address = 0, // Use public or static device address, or identity address if privacy mode is enabled
    non_resolvable = 1, // Use non resolvable address type, advertising mode must also be non-connectable
}

pub enum ConnectableMode {
    non_connectable = 0,           // Non-connectable non-scannable.
    directed_connectable = 1,      // Directed connectable (RESERVED, DO NOT USE)
    connectable_scannable = 2, // Undirected connectable scannable. This mode can only be used in legacy advertising PDUs.
    scannable_non_connectable = 3, // Undirected scannable (Non-connectable but responds to scan requests)
    connectable_non_scannable = 4, // Undirected connectable non-scannable. This mode can only be used in extended advertising PDUs.
}

pub enum DiscoverMode {
    limited = 0,     // Discover only limited discoverable devices
    generic = 1,     // Discover limited and generic discoverable devices
    observation = 2, // Discover all devices
}

pub enum DiscoverableMode {
    non_discoverable = 0,     // Not discoverable
    limited_discoverable = 1, // Discoverable using both limited and general discovery procedures
    general_discoverable = 2, // Discoverable using general discovery procedure
    broadcast = 3, // Device is not discoverable in either limited or generic discovery procedure, but may be discovered by using the Observation procedure
    user_data = 4, // Send advertising and/or scan response data defined by the user using le_gap_bt5_set_adv_data. The limited/general discoverable flags are defined by the user.
}

pub enum PhyType {
    phy_1m = 1,    // LE 1M PHY
    phy_2m = 2,    // LE 2M PHY
    phy_coded = 4, // LE Coded PHY
}
