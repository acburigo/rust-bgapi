pub enum Command {
    discover_characteristics {
        connection: u8,
        service: u32,
    },
    discover_characteristics_by_uuid {
        connection: u8,
        service: u32,
        uuid: [u8; 16],
    },
    discover_descriptors {
        connection: u8,
        characteristic: u16,
    },
    discover_primary_services {
        connection: u8,
    },
    discover_primary_services_by_uuid {
        connection: u8,
        uuid: [u8; 16],
    },
    execute_characteristic_value_write {
        connection: u8,
        flags: u8,
    },
    find_included_services {
        connection: u8,
        service: u32,
    },
    prepare_characteristic_value_reliable_write {
        connection: u8,
        characteristic: u16,
        offset: u16,
        value: Box<[u8]>,
    },
    prepare_characteristic_value_write {
        connection: u8,
        characteristic: u16,
        offset: u16,
        value: Box<[u8]>,
    },
    read_characteristic_value {
        connection: u8,
        characteristic: u16,
    },
    read_characteristic_value_by_uuid {
        connection: u8,
        service: u32,
        uuid: [u8; 16],
    },
    read_characteristic_value_from_offset {
        connection: u8,
        characteristic: u16,
        offset: u16,
        maxlen: u16,
    },
    read_descriptor_value {
        connection: u8,
        descriptor: u16,
    },
    read_multiple_characteristic_values {
        connection: u8,
        characteristic_list: Box<[u16]>,
    },
    send_characteristic_confirmation {
        connection: u8,
    },
    set_characteristic_notification {
        connection: u8,
        characteristic: u16,
        flags: u8,
    },
    set_max_mtu {
        max_mtu: u16,
    },
    write_characteristic_value {
        connection: u8,
        characteristic: u16,
        value: Box<[u8]>,
    },
    write_characteristic_value_without_response {
        connection: u8,
        characteristic: u16,
        value: Box<[u8]>,
    },
    write_descriptor_value {
        connection: u8,
        descriptor: u16,
        value: Box<[u8]>,
    },
}

pub enum Response {
    discover_characteristics { result: u16 },
    discover_characteristics_by_uuid { result: u16 },
    discover_descriptors { result: u16 },
    discover_primary_services { result: u16 },
    discover_primary_services_by_uuid { result: u16 },
    execute_characteristic_value_write { result: u16 },
    find_included_services { result: u16 },
    prepare_characteristic_value_reliable_write { result: u16, sent_len: u16 },
    prepare_characteristic_value_write { result: u16, sent_len: u16 },
    read_characteristic_value { result: u16 },
    read_characteristic_value_by_uuid { result: u16 },
    read_characteristic_value_from_offset { result: u16 },
    read_descriptor_value { result: u16 },
    read_multiple_characteristic_values { result: u16 },
    send_characteristic_confirmation { result: u16 },
    set_characteristic_notification { result: u16 },
    set_max_mtu { result: u16, max_mtu: u16 },
    write_characteristic_value { result: u16 },
    write_characteristic_value_without_response { result: u16, sent_len: u16 },
    write_descriptor_value { result: u16 },
}

pub enum Event {
    characteristic {
        connection: u8,
        characteristic: u16,
        properties: u8,
        uuid: [u8; 16],
    },
    characteristic_value {
        connection: u8,
        characteristic: u16,
        att_opcode: u8,
        offset: u16,
        value: Box<[u8]>,
    },
    descriptor {
        connection: u8,
        descriptor: u16,
        uuid: [u8; 16],
    },
    descriptor_value {
        connection: u8,
        descriptor: u16,
        offset: u16,
        value: Box<[u8]>,
    },
    mtu_exchanged {
        connection: u8,
        mtu: u16,
    },
    procedure_completed {
        connection: u8,
        result: u16,
    },
    service {
        connection: u8,
        service: u32,
        uuid: [u8; 16],
    },
}

pub enum AttOpcode {
    read_by_type_request = 8,
    read_by_type_response = 9,
    read_request = 10,
    read_response = 11,
    read_blob_request = 12,
    read_blob_response = 13,
    read_multiple_request = 14,
    read_multiple_response = 15,
    write_request = 18,
    write_response = 19,
    write_command = 82,
    prepare_write_request = 22,
    prepare_write_response = 23,
    execute_write_request = 24,
    execute_write_response = 25,
    handle_value_notification = 27,
    handle_value_indication = 29,
}

pub enum ClientConfigFlag {
    disable = 0,
    notification = 1,
    indication = 2,
}

pub enum execute_write_flag {
    cancel = 0,
    commit = 1,
}
