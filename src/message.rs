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

    //...

    // test
    cmd_test_dtm_end(test::cmd::dtm_end),
    rsp_test_dtm_end(test::rsp::dtm_end),
    cmd_test_dtm_rx(test::cmd::dtm_rx),
    rsp_test_dtm_rx(test::rsp::dtm_rx),
    cmd_test_dtm_tx(test::cmd::dtm_tx),
    rsp_test_dtm_tx(test::rsp::dtm_tx),
    evt_test_dtm_completed(test::evt::dtm_completed),
}

impl ToBytes for MessagePayload {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            // coex
            MessagePayload::cmd_coex_get_counters(x) => x.to_bytes(),
            MessagePayload::rsp_coex_get_counters(x) => x.to_bytes(),
            MessagePayload::cmd_coex_set_options(x) => x.to_bytes(),
            MessagePayload::rsp_coex_set_options(x) => x.to_bytes(),

            // dfu
            MessagePayload::cmd_dfu_flash_set_address(x) => x.to_bytes(),
            MessagePayload::rsp_dfu_flash_set_address(x) => x.to_bytes(),
            MessagePayload::cmd_dfu_flash_upload(x) => x.to_bytes(),
            MessagePayload::rsp_dfu_flash_upload(x) => x.to_bytes(),
            MessagePayload::cmd_dfu_flash_upload_finish(x) => x.to_bytes(),
            MessagePayload::rsp_dfu_flash_upload_finish(x) => x.to_bytes(),
            MessagePayload::cmd_dfu_reset(x) => x.to_bytes(),
            MessagePayload::evt_dfu_boot(x) => x.to_bytes(),
            MessagePayload::evt_dfu_boot_failure(x) => x.to_bytes(),

            // flash
            MessagePayload::cmd_flash_ps_erase(x) => x.to_bytes(),
            MessagePayload::rsp_flash_ps_erase(x) => x.to_bytes(),
            MessagePayload::cmd_flash_ps_erase_all(x) => x.to_bytes(),
            MessagePayload::rsp_flash_ps_erase_all(x) => x.to_bytes(),
            MessagePayload::cmd_flash_ps_load(x) => x.to_bytes(),
            MessagePayload::rsp_flash_ps_load(x) => x.to_bytes(),
            MessagePayload::cmd_flash_ps_save(x) => x.to_bytes(),
            MessagePayload::rsp_flash_ps_save(x) => x.to_bytes(),

            // gatt
            MessagePayload::cmd_gatt_discover_characteristics(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_discover_characteristics(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_discover_characteristics_by_uuid(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_discover_characteristics_by_uuid(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_discover_descriptors(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_discover_descriptors(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_discover_primary_services(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_discover_primary_services(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_discover_primary_services_by_uuid(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_discover_primary_services_by_uuid(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_execute_characteristic_value_write(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_execute_characteristic_value_write(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_find_included_services(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_find_included_services(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_prepare_characteristic_value_reliable_write(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_prepare_characteristic_value_reliable_write(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_prepare_characteristic_value_write(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_prepare_characteristic_value_write(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_read_characteristic_value(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_read_characteristic_value(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_read_characteristic_value_by_uuid(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_read_characteristic_value_by_uuid(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_read_characteristic_value_from_offset(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_read_characteristic_value_from_offset(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_read_descriptor_value(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_read_descriptor_value(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_read_multiple_characteristic_values(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_read_multiple_characteristic_values(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_send_characteristic_confirmation(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_send_characteristic_confirmation(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_set_characteristic_notification(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_set_characteristic_notification(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_set_max_mtu(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_set_max_mtu(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_write_characteristic_value(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_write_characteristic_value(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_write_characteristic_value_without_response(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_write_characteristic_value_without_response(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_write_descriptor_value(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_write_descriptor_value(x) => x.to_bytes(),
            MessagePayload::evt_gatt_characteristic(x) => x.to_bytes(),
            MessagePayload::evt_gatt_characteristic_value(x) => x.to_bytes(),
            MessagePayload::evt_gatt_descriptor(x) => x.to_bytes(),
            MessagePayload::evt_gatt_descriptor_value(x) => x.to_bytes(),
            MessagePayload::evt_gatt_mtu_exchanged(x) => x.to_bytes(),
            MessagePayload::evt_gatt_procedure_completed(x) => x.to_bytes(),
            MessagePayload::evt_gatt_service(x) => x.to_bytes(),

            // gatt_server
            MessagePayload::cmd_gatt_server_find_attribute(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_server_find_attribute(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_server_read_attribute_type(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_server_read_attribute_type(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_server_read_attribute_value(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_server_read_attribute_value(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_server_send_characteristic_notification(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_server_send_characteristic_notification(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_server_send_user_read_response(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_server_send_user_read_response(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_server_send_user_write_response(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_server_set_capabilities(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_server_set_capabilities(x) => x.to_bytes(),
            MessagePayload::cmd_gatt_server_write_attribute_value(x) => x.to_bytes(),
            MessagePayload::rsp_gatt_server_write_attribute_value(x) => x.to_bytes(),
            MessagePayload::evt_gatt_server_attribute_value(x) => x.to_bytes(),
            MessagePayload::evt_gatt_server_characteristic_status(x) => x.to_bytes(),
            MessagePayload::evt_gatt_server_execute_write_completed(x) => x.to_bytes(),
            MessagePayload::evt_gatt_server_user_read_request(x) => x.to_bytes(),
            MessagePayload::evt_gatt_server_user_write_request(x) => x.to_bytes(),

            // hardware
            MessagePayload::cmd_hardware_set_lazy_soft_timer(x) => x.to_bytes(),
            MessagePayload::rsp_hardware_set_lazy_soft_timer(x) => x.to_bytes(),
            MessagePayload::cmd_hardware_set_soft_timer(x) => x.to_bytes(),
            MessagePayload::rsp_hardware_set_soft_timer(x) => x.to_bytes(),
            MessagePayload::evt_hardware_soft_timer(x) => x.to_bytes(),

            // le_connection
            MessagePayload::cmd_le_connection_close(x) => x.to_bytes(),
            MessagePayload::rsp_le_connection_close(x) => x.to_bytes(),
            MessagePayload::cmd_le_connection_disable_slave_latency(x) => x.to_bytes(),
            MessagePayload::rsp_le_connection_disable_slave_latency(x) => x.to_bytes(),
            MessagePayload::cmd_le_connection_get_rssi(x) => x.to_bytes(),
            MessagePayload::rsp_le_connection_get_rssi(x) => x.to_bytes(),
            MessagePayload::cmd_le_connection_set_parameters(x) => x.to_bytes(),
            MessagePayload::rsp_le_connection_set_parameters(x) => x.to_bytes(),
            MessagePayload::cmd_le_connection_set_phy(x) => x.to_bytes(),
            MessagePayload::rsp_le_connection_set_phy(x) => x.to_bytes(),
            MessagePayload::evt_le_connection_closed(x) => x.to_bytes(),
            MessagePayload::evt_le_connection_opened(x) => x.to_bytes(),
            MessagePayload::evt_le_connection_parameters(x) => x.to_bytes(),
            MessagePayload::evt_le_connection_phy_status(x) => x.to_bytes(),
            MessagePayload::evt_le_connection_rssi(x) => x.to_bytes(),

            // ,,,

            // test
            MessagePayload::cmd_test_dtm_end(x) => x.to_bytes(),
            MessagePayload::rsp_test_dtm_end(x) => x.to_bytes(),
            MessagePayload::cmd_test_dtm_rx(x) => x.to_bytes(),
            MessagePayload::rsp_test_dtm_rx(x) => x.to_bytes(),
            MessagePayload::cmd_test_dtm_tx(x) => x.to_bytes(),
            MessagePayload::rsp_test_dtm_tx(x) => x.to_bytes(),
            MessagePayload::evt_test_dtm_completed(x) => x.to_bytes(),

            _ => Vec::new(),
        }
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
