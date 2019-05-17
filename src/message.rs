use bytes::BufMut;
use coex;
use dfu;
use flash;
use gatt;
use gatt_server;
use hardware;
use le_connection;
use le_gap;
use parser::{FromBytes, ToBytes};
use sm;
use system;
use test;
use user;

pub struct Message {
    pub header: MessageHeader,
    pub payload: MessagePayload,
}

impl ToBytes for Message {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes();
        bytes.extend(self.payload.to_bytes().iter());
        bytes
    }
}

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum MessageType {
    command_response = 0x20,
    event = 0xa0,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum MessageClass {
    coex = 0x20,
    dfu = 0x00,
    endpoint = 0x0b,
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
    pub message_type: u8,
    pub payload_length: u8,
    pub message_class: u8,
    pub message_id: u8,
}

#[allow(non_camel_case_types)]
pub enum MessagePayload {
    // coex
    cmd_coex_get_counters(coex::cmd::get_counters),
    rsp_coex_get_counters(coex::rsp::get_counters),
    cmd_coex_set_options(coex::cmd::set_options),
    rsp_coex_set_options(coex::rsp::set_options),

    // dfu
    cmd_dfu_flash_set_address(dfu::cmd::flash_set_address),
    rsp_dfu_flash_set_address(dfu::rsp::flash_set_address),
    cmd_dfu_flash_upload(dfu::cmd::flash_upload),
    rsp_dfu_flash_upload(dfu::rsp::flash_upload),
    cmd_dfu_flash_upload_finish(dfu::cmd::flash_upload_finish),
    rsp_dfu_flash_upload_finish(dfu::rsp::flash_upload_finish),
    cmd_dfu_reset(dfu::cmd::reset),
    evt_dfu_boot(dfu::evt::boot),
    evt_dfu_boot_failure(dfu::evt::boot_failure),

    // flash
    cmd_flash_ps_erase(flash::cmd::ps_erase),
    rsp_flash_ps_erase(flash::rsp::ps_erase),
    cmd_flash_ps_erase_all(flash::cmd::ps_erase_all),
    rsp_flash_ps_erase_all(flash::rsp::ps_erase_all),
    cmd_flash_ps_load(flash::cmd::ps_load),
    rsp_flash_ps_load(flash::rsp::ps_load),
    cmd_flash_ps_save(flash::cmd::ps_save),
    rsp_flash_ps_save(flash::rsp::ps_save),

    // gatt
    cmd_gatt_discover_characteristics(gatt::cmd::discover_characteristics),
    rsp_gatt_discover_characteristics(gatt::rsp::discover_characteristics),
    cmd_gatt_discover_characteristics_by_uuid(gatt::cmd::discover_characteristics_by_uuid),
    rsp_gatt_discover_characteristics_by_uuid(gatt::rsp::discover_characteristics_by_uuid),
    cmd_gatt_discover_descriptors(gatt::cmd::discover_descriptors),
    rsp_gatt_discover_descriptors(gatt::rsp::discover_descriptors),
    cmd_gatt_discover_primary_services(gatt::cmd::discover_primary_services),
    rsp_gatt_discover_primary_services(gatt::rsp::discover_primary_services),
    cmd_gatt_discover_primary_services_by_uuid(gatt::cmd::discover_primary_services_by_uuid),
    rsp_gatt_discover_primary_services_by_uuid(gatt::rsp::discover_primary_services_by_uuid),
    cmd_gatt_execute_characteristic_value_write(gatt::cmd::execute_characteristic_value_write),
    rsp_gatt_execute_characteristic_value_write(gatt::rsp::execute_characteristic_value_write),
    cmd_gatt_find_included_services(gatt::cmd::find_included_services),
    rsp_gatt_find_included_services(gatt::rsp::find_included_services),
    cmd_gatt_prepare_characteristic_value_reliable_write(
        gatt::cmd::prepare_characteristic_value_reliable_write,
    ),
    rsp_gatt_prepare_characteristic_value_reliable_write(
        gatt::rsp::prepare_characteristic_value_reliable_write,
    ),
    cmd_gatt_prepare_characteristic_value_write(gatt::cmd::prepare_characteristic_value_write),
    rsp_gatt_prepare_characteristic_value_write(gatt::rsp::prepare_characteristic_value_write),
    cmd_gatt_read_characteristic_value(gatt::cmd::read_characteristic_value),
    rsp_gatt_read_characteristic_value(gatt::rsp::read_characteristic_value),
    cmd_gatt_read_characteristic_value_by_uuid(gatt::cmd::read_characteristic_value_by_uuid),
    rsp_gatt_read_characteristic_value_by_uuid(gatt::rsp::read_characteristic_value_by_uuid),
    cmd_gatt_read_characteristic_value_from_offset(
        gatt::cmd::read_characteristic_value_from_offset,
    ),
    rsp_gatt_read_characteristic_value_from_offset(
        gatt::rsp::read_characteristic_value_from_offset,
    ),
    cmd_gatt_read_descriptor_value(gatt::cmd::read_descriptor_value),
    rsp_gatt_read_descriptor_value(gatt::rsp::read_descriptor_value),
    cmd_gatt_read_multiple_characteristic_values(gatt::cmd::read_multiple_characteristic_values),
    rsp_gatt_read_multiple_characteristic_values(gatt::rsp::read_multiple_characteristic_values),
    cmd_gatt_send_characteristic_confirmation(gatt::cmd::send_characteristic_confirmation),
    rsp_gatt_send_characteristic_confirmation(gatt::rsp::send_characteristic_confirmation),
    cmd_gatt_set_characteristic_notification(gatt::cmd::set_characteristic_notification),
    rsp_gatt_set_characteristic_notification(gatt::rsp::set_characteristic_notification),
    cmd_gatt_set_max_mtu(gatt::cmd::set_max_mtu),
    rsp_gatt_set_max_mtu(gatt::rsp::set_max_mtu),
    cmd_gatt_write_characteristic_value(gatt::cmd::write_characteristic_value),
    rsp_gatt_write_characteristic_value(gatt::rsp::write_characteristic_value),
    cmd_gatt_write_characteristic_value_without_response(
        gatt::cmd::write_characteristic_value_without_response,
    ),
    rsp_gatt_write_characteristic_value_without_response(
        gatt::rsp::write_characteristic_value_without_response,
    ),
    cmd_gatt_write_descriptor_value(gatt::cmd::write_descriptor_value),
    rsp_gatt_write_descriptor_value(gatt::rsp::write_descriptor_value),
    evt_gatt_characteristic(gatt::evt::characteristic),
    evt_gatt_characteristic_value(gatt::evt::characteristic_value),
    evt_gatt_descriptor(gatt::evt::descriptor),
    evt_gatt_descriptor_value(gatt::evt::descriptor_value),
    evt_gatt_mtu_exchanged(gatt::evt::mtu_exchanged),
    evt_gatt_procedure_completed(gatt::evt::procedure_completed),
    evt_gatt_service(gatt::evt::service),

    // gatt_server
    cmd_gatt_server_find_attribute(gatt_server::cmd::find_attribute),
    rsp_gatt_server_find_attribute(gatt_server::rsp::find_attribute),
    cmd_gatt_server_read_attribute_type(gatt_server::cmd::read_attribute_type),
    rsp_gatt_server_read_attribute_type(gatt_server::rsp::read_attribute_type),
    cmd_gatt_server_read_attribute_value(gatt_server::cmd::read_attribute_value),
    rsp_gatt_server_read_attribute_value(gatt_server::rsp::read_attribute_value),
    cmd_gatt_server_send_characteristic_notification(
        gatt_server::cmd::send_characteristic_notification,
    ),
    rsp_gatt_server_send_characteristic_notification(
        gatt_server::rsp::send_characteristic_notification,
    ),
    cmd_gatt_server_send_user_read_response(gatt_server::cmd::send_user_read_response),
    rsp_gatt_server_send_user_read_response(gatt_server::rsp::send_user_read_response),
    rsp_gatt_server_send_user_write_response(gatt_server::rsp::send_user_write_response),
    cmd_gatt_server_set_capabilities(gatt_server::cmd::set_capabilities),
    rsp_gatt_server_set_capabilities(gatt_server::rsp::set_capabilities),
    cmd_gatt_server_write_attribute_value(gatt_server::cmd::write_attribute_value),
    rsp_gatt_server_write_attribute_value(gatt_server::rsp::write_attribute_value),
    evt_gatt_server_attribute_value(gatt_server::evt::attribute_value),
    evt_gatt_server_characteristic_status(gatt_server::evt::characteristic_status),
    evt_gatt_server_execute_write_completed(gatt_server::evt::execute_write_completed),
    evt_gatt_server_user_read_request(gatt_server::evt::user_read_request),
    evt_gatt_server_user_write_request(gatt_server::evt::user_write_request),

    // hardware
    cmd_hardware_set_lazy_soft_timer(hardware::cmd::set_lazy_soft_timer),
    rsp_hardware_set_lazy_soft_timer(hardware::rsp::set_lazy_soft_timer),
    cmd_hardware_set_soft_timer(hardware::cmd::set_soft_timer),
    rsp_hardware_set_soft_timer(hardware::rsp::set_soft_timer),
    evt_hardware_soft_timer(hardware::evt::soft_timer),

    // le_connection
    cmd_le_connection_close(le_connection::cmd::close),
    rsp_le_connection_close(le_connection::rsp::close),
    cmd_le_connection_disable_slave_latency(le_connection::cmd::disable_slave_latency),
    rsp_le_connection_disable_slave_latency(le_connection::rsp::disable_slave_latency),
    cmd_le_connection_get_rssi(le_connection::cmd::get_rssi),
    rsp_le_connection_get_rssi(le_connection::rsp::get_rssi),
    cmd_le_connection_set_parameters(le_connection::cmd::set_parameters),
    rsp_le_connection_set_parameters(le_connection::rsp::set_parameters),
    cmd_le_connection_set_phy(le_connection::cmd::set_phy),
    rsp_le_connection_set_phy(le_connection::rsp::set_phy),
    evt_le_connection_closed(le_connection::evt::closed),
    evt_le_connection_opened(le_connection::evt::opened),
    evt_le_connection_parameters(le_connection::evt::parameters),
    evt_le_connection_phy_status(le_connection::evt::phy_status),
    evt_le_connection_rssi(le_connection::evt::rssi),

    // le_gap
    cmd_le_gap_bt5_set_adv_data(le_gap::cmd::bt5_set_adv_data),
    rsp_le_gap_bt5_set_adv_data(le_gap::rsp::bt5_set_adv_data),
    cmd_le_gap_clear_advertise_configuration(le_gap::cmd::clear_advertise_configuration),
    rsp_le_gap_clear_advertise_configuration(le_gap::rsp::clear_advertise_configuration),
    cmd_le_gap_connect(le_gap::cmd::connect),
    rsp_le_gap_connect(le_gap::rsp::connect),
    cmd_le_gap_end_procedure(le_gap::cmd::end_procedure),
    rsp_le_gap_end_procedure(le_gap::rsp::end_procedure),
    cmd_le_gap_set_advertise_channel_map(le_gap::cmd::set_advertise_channel_map),
    rsp_le_gap_set_advertise_channel_map(le_gap::rsp::set_advertise_channel_map),
    cmd_le_gap_set_advertise_configuration(le_gap::cmd::set_advertise_configuration),
    rsp_le_gap_set_advertise_configuration(le_gap::rsp::set_advertise_configuration),
    cmd_le_gap_set_advertise_phy(le_gap::cmd::set_advertise_phy),
    rsp_le_gap_set_advertise_phy(le_gap::rsp::set_advertise_phy),
    cmd_le_gap_set_advertise_report_scan_request(le_gap::cmd::set_advertise_report_scan_request),
    rsp_le_gap_set_advertise_report_scan_request(le_gap::rsp::set_advertise_report_scan_request),
    cmd_le_gap_set_advertise_timing(le_gap::cmd::set_advertise_timing),
    rsp_le_gap_set_advertise_timing(le_gap::rsp::set_advertise_timing),
    cmd_le_gap_set_advertise_tx_power(le_gap::cmd::set_advertise_tx_power),
    rsp_le_gap_set_advertise_tx_power(le_gap::rsp::set_advertise_tx_power),
    cmd_le_gap_set_conn_parameters(le_gap::cmd::set_conn_parameters),
    rsp_le_gap_set_conn_parameters(le_gap::rsp::set_conn_parameters),
    cmd_le_gap_set_data_channel_classification(le_gap::cmd::set_data_channel_classification),
    rsp_le_gap_set_data_channel_classification(le_gap::rsp::set_data_channel_classification),
    cmd_le_gap_set_discovery_timing(le_gap::cmd::set_discovery_timing),
    rsp_le_gap_set_discovery_timing(le_gap::rsp::set_discovery_timing),
    cmd_le_gap_set_discovery_type(le_gap::cmd::set_discovery_type),
    rsp_le_gap_set_discovery_type(le_gap::rsp::set_discovery_type),
    cmd_le_gap_set_privacy_mode(le_gap::cmd::set_privacy_mode),
    rsp_le_gap_set_privacy_mode(le_gap::rsp::set_privacy_mode),
    cmd_le_gap_start_advertising(le_gap::cmd::start_advertising),
    rsp_le_gap_start_advertising(le_gap::rsp::start_advertising),
    cmd_le_gap_start_discovery(le_gap::cmd::start_discovery),
    rsp_le_gap_start_discovery(le_gap::rsp::start_discovery),
    cmd_le_gap_stop_advertising(le_gap::cmd::stop_advertising),
    rsp_le_gap_stop_advertising(le_gap::rsp::stop_advertising),
    evt_le_gap_adv_timeout(le_gap::evt::adv_timeout),
    evt_le_gap_scan_request(le_gap::evt::scan_request),
    evt_le_gap_scan_response(le_gap::evt::scan_response),

    // sm
    cmd_sm_bonding_confirm(sm::cmd::bonding_confirm),
    rsp_sm_bonding_confirm(sm::rsp::bonding_confirm),
    cmd_sm_configure(sm::cmd::configure),
    rsp_sm_configure(sm::rsp::configure),
    cmd_sm_delete_bonding(sm::cmd::delete_bonding),
    rsp_sm_delete_bonding(sm::rsp::delete_bonding),
    cmd_sm_delete_bondings(sm::cmd::delete_bondings),
    rsp_sm_delete_bondings(sm::rsp::delete_bondings),
    cmd_sm_enter_passkey(sm::cmd::enter_passkey),
    rsp_sm_enter_passkey(sm::rsp::enter_passkey),
    cmd_sm_increase_security(sm::cmd::increase_security),
    rsp_sm_increase_security(sm::rsp::increase_security),
    cmd_sm_list_all_bondings(sm::cmd::list_all_bondings),
    rsp_sm_list_all_bondings(sm::rsp::list_all_bondings),
    cmd_sm_passkey_confirm(sm::cmd::passkey_confirm),
    rsp_sm_passkey_confirm(sm::rsp::passkey_confirm),
    cmd_sm_set_bondable_mode(sm::cmd::set_bondable_mode),
    rsp_sm_set_bondable_mode(sm::rsp::set_bondable_mode),
    cmd_sm_set_debug_mode(sm::cmd::set_debug_mode),
    rsp_sm_set_debug_mode(sm::rsp::set_debug_mode),
    cmd_sm_set_oob_data(sm::cmd::set_oob_data),
    rsp_sm_set_oob_data(sm::rsp::set_oob_data),
    cmd_sm_set_passkey(sm::cmd::set_passkey),
    rsp_sm_set_passkey(sm::rsp::set_passkey),
    cmd_sm_set_sc_remote_oob_data(sm::cmd::set_sc_remote_oob_data),
    rsp_sm_set_sc_remote_oob_data(sm::rsp::set_sc_remote_oob_data),
    cmd_sm_store_bonding_configuration(sm::cmd::store_bonding_configuration),
    rsp_sm_store_bonding_configuration(sm::rsp::store_bonding_configuration),
    cmd_sm_use_sc_oob(sm::cmd::use_sc_oob),
    rsp_sm_use_sc_oob(sm::rsp::use_sc_oob),
    evt_sm_bonded(sm::evt::bonded),
    evt_sm_bonding_failed(sm::evt::bonding_failed),
    evt_sm_confirm_bonding(sm::evt::confirm_bonding),
    evt_sm_confirm_passkey(sm::evt::confirm_passkey),
    evt_sm_list_all_bondings_complete(sm::evt::list_all_bondings_complete),
    evt_sm_list_bonding_entry(sm::evt::list_bonding_entry),
    evt_sm_passkey_display(sm::evt::passkey_display),
    evt_sm_passkey_request(sm::evt::passkey_request),

    // system
    cmd_system_get_bt_address(system::cmd::get_bt_address),
    rsp_system_get_bt_address(system::rsp::get_bt_address),
    cmd_system_get_counters(system::cmd::get_counters),
    rsp_system_get_counters(system::rsp::get_counters),
    cmd_system_get_random_data(system::cmd::get_random_data),
    rsp_system_get_random_data(system::rsp::get_random_data),
    cmd_system_halt(system::cmd::halt),
    rsp_system_halt(system::rsp::halt),
    cmd_system_hello(system::cmd::hello),
    rsp_system_hello(system::rsp::hello),
    cmd_system_reset(system::cmd::reset),
    cmd_system_set_bt_address(system::cmd::set_bt_address),
    rsp_system_set_bt_address(system::rsp::set_bt_address),
    cmd_system_set_device_name(system::cmd::set_device_name),
    rsp_system_set_device_name(system::rsp::set_device_name),
    cmd_system_set_tx_power(system::cmd::set_tx_power),
    rsp_system_set_tx_power(system::rsp::set_tx_power),
    evt_system_awake(system::evt::awake),
    evt_system_boot(system::evt::boot),
    evt_system_error(system::evt::error),
    evt_system_external_signal(system::evt::external_signal),
    evt_system_hardware_error(system::evt::hardware_error),

    // test
    cmd_test_dtm_end(test::cmd::dtm_end),
    rsp_test_dtm_end(test::rsp::dtm_end),
    cmd_test_dtm_rx(test::cmd::dtm_rx),
    rsp_test_dtm_rx(test::rsp::dtm_rx),
    cmd_test_dtm_tx(test::cmd::dtm_tx),
    rsp_test_dtm_tx(test::rsp::dtm_tx),
    evt_test_dtm_completed(test::evt::dtm_completed),

    // user
    cmd_user_message_to_target(user::cmd::message_to_target),
    rsp_user_message_to_target(user::rsp::message_to_target),
    evt_user_message_to_host(user::evt::message_to_host),
}

impl ToBytes for MessagePayload {
    fn to_bytes(&self) -> Vec<u8> {
        let payload: &ToBytes = self;
        payload.to_bytes()
    }
}

impl MessageHeader {
    pub fn size() -> usize {
        const HEADER_SIZE_BYTES: usize = 4;
        HEADER_SIZE_BYTES
    }
}

impl FromBytes for MessageHeader {
    fn from_bytes(data: &[u8]) -> MessageHeader {
        MessageHeader {
            message_type: data[0],
            payload_length: data[1],
            message_class: data[2],
            message_id: data[3],
        }
    }
}

impl ToBytes for MessageHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.put_u8(self.message_type);
        bytes.put_u8(self.payload_length);
        bytes.put_u8(self.message_class);
        bytes.put_u8(self.message_id);
        bytes
    }
}
