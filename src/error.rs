use num_derive::FromPrimitive;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, FromPrimitive, PartialEq, PartialOrd)]
pub enum Error {
    success = 0x0000,

    // Errors related to hardware
    ps_store_full = 0x0501,    // Flash reserved for PS store is full
    ps_key_not_found = 0x0502, // PS key not found
    i2c_ack_missing = 0x0503,  // Acknowledge for i2c was not received.
    i2c_timeout = 0x0504,      // I2C read or write timed out.

    // Errors related to BGAPI protocol
    invalid_conn_handle = 0x0101,     // Invalid GATT connection handle.
    waiting_response = 0x0102,        // Waiting response from GATT server to previous procedure.
    gatt_connection_timeout = 0x0103, // GATT connection is closed due procedure timeout.
    invalid_param = 0x0180,           // Command contained invalid parameter
    wrong_state = 0x0181,             // Device is in wrong state to receive command
    out_of_memory = 0x0182,           // Device has run out of memory
    not_implemented = 0x0183,         // Feature is not implemented
    invalid_command = 0x0184,         // Command was not recognized
    timeout = 0x0185,                 // Command or Procedure failed due to timeout
    not_connected = 0x0186, // Connection handle passed is to command is not a valid handle
    flow = 0x0187,          // Command would cause either underflow or overflow error
    user_attribute = 0x0188, // User attribute was accessed through API which is not supported
    invalid_license_key = 0x0189, // No valid license key found
    command_too_long = 0x018a, // Command maximum length exceeded
    out_of_bonds = 0x018b, // Bonding procedure can't be started because device has no space left for bond.
    unspecified = 0x018c,  // Unspecified error
    hardware = 0x018d,     // Hardware failure
    buffers_full = 0x018e, // Command not accepted, because internal buffers are full
    disconnected = 0x018f, // Command or Procedure failed due to disconnection
    too_many_requests = 0x0190, // Too many Simultaneous Requests
    not_supported = 0x0191, // Feature is not supported in this firmware build
    no_bonding = 0x0192,   // The bonding does not exist.
    crypto = 0x0193,       // Error using crypto functions
    data_corrupted = 0x0194, // Data was corrupted.
    command_incomplete = 0x0195, // Data received does not form a complete command

    // Errors from Security Manager Protocol
    passkey_entry_failed = 0x0301, // The user input of passkey failed, for example, the user cancelled the operation
    oob_not_available = 0x0302,    // Out of Band data is not available for authentication
    authentication_requirements = 0x0303, // The pairing procedure cannot be performed as authentication requirements cannot be met due to IO capabilities of one or both devices
    confirm_value_failed = 0x0304, // The confirm value does not match the calculated compare value
    pairing_not_supported = 0x0305, // Pairing is not supported by the device
    encryption_key_size = 0x0306, // The resultant encryption key size is insufficient for the security requirements of this device
    command_not_supported = 0x0307, // The SMP command received is not supported on this device
    unspecified_reason = 0x0308,  // Pairing failed due to an unspecified reason
    sm_repeated_attempts = 0x0309, // Pairing or authentication procedure is disallowed because too little time has elapsed since last pairing request or security request
    invalid_parameters = 0x030a, // The Invalid Parameters error code indicates: the command length is invalid or a parameter is outside of the specified range.
    dhkey_check_failed = 0x030b, // Indicates to the remote device that the DHKey Check value received doesn't match the one calculated by the local device.
    numeric_comparison_failed = 0x030c, // Indicates that the confirm values in the numeric comparison protocol do not match.
    bredr_pairing_in_progress = 0x030d, // Indicates that the pairing over the LE transport failed due to a Pairing Request sent over the BR/EDR transport in process.
    cross_transport_key_derivation_generation_not_allowed = 0x030e, // Indicates that the BR/EDR Link Key generated on the BR/EDR transport cannot be used to derive and distribute keys for the LE transport.

    // Bluetooth errors
    unknown_connection_identifier = 0x0202, // Connection does not exist, or connection open request was cancelled.
    page_timeout = 0x0204, // The Page Timeout error code indicates that a page timed out because of the Page Timeout configuration parameter.
    authentication_failure = 0x0205, // Pairing or authentication failed due to incorrect results in the pairing or authentication procedure. This could be due to an incorrect PIN or Link Key
    pin_or_key_missing = 0x0206, // Pairing failed because of missing PIN, or authentication failed because of missing Key
    memory_capacity_exceeded = 0x0207, // Controller is out of memory.
    connection_timeout = 0x0208, // Link supervision timeout has expired.
    connection_limit_exceeded = 0x0209, // Controller is at limit of connections it can support.
    synchronous_connectiontion_limit_exceeded = 0x020a, // The Synchronous Connection Limit to a Device Exceeded error code indicates that the Controller has reached the limit to the number of synchronous connections that can be achieved to a device.
    acl_connection_already_exists = 0x020b, // The ACL Connection Already Exists error code indicates that an attempt to create a new ACL Connection to a device when there is already a connection to this device.
    command_disallowed = 0x020c, // Command requested cannot be executed because the Controller is in a state where it cannot process this command at this time.
    connection_rejected_due_to_limited_resources = 0x020d, // The Connection Rejected Due To Limited Resources error code indicates that an incoming connection was rejected due to limited resources.
    connection_rejected_due_to_security_reasons = 0x020e, // The Connection Rejected Due To Security Reasons error code indicates that a connection was rejected due to security requirements not being fulfilled, like authentication or pairing.
    connection_rejected_due_to_unacceptable_bd_addr = 0x020f, // The Connection was rejected because this device does not accept the BD_ADDR. This may be because the device will only accept connections from specific BD_ADDRs.
    connection_accept_timeout_exceeded = 0x0210, // The Connection Accept Timeout has been exceeded for this connection attempt.
    unsupported_feature_or_parameter_value = 0x0211, // A feature or parameter value in the HCI command is not supported.
    invalid_command_parameters = 0x0212,             // Command contained invalid parameters.
    remote_user_terminated = 0x0213, // User on the remote device terminated the connection.
    remote_device_terminated_connection_due_to_low_resources = 0x0214, // The remote device terminated the connection because of low resources
    remote_powering_off = 0x0215, // Remote Device Terminated Connection due to Power Off
    connection_terminated_by_local_host = 0x0216, // Local device terminated the connection.
    repeated_attempts = 0x0217, // The Controller is disallowing an authentication or pairing procedure because too little time has elapsed since the last authentication or pairing attempt failed.
    pairing_not_allowed = 0x0218, // The device does not allow pairing. This can be for example, when a device only allows pairing during a certain time window after some user input allows pairing
    unknown_lmp_pdu = 0x0219,     // The Controller has received an unknown LMP OpCode.
    unsupported_remote_feature = 0x021a, // The remote device does not support the feature associated with the issued command or LMP PDU.
    sco_offset_rejected = 0x021b, // The offset requested in the LMP_SCO_link_req PDU has been rejected.
    sco_interval_rejected = 0x021c, // The interval requested in the LMP_SCO_link_req PDU has been rejected.
    sco_air_mode_rejected = 0x021d, // The air mode requested in the LMP_SCO_link_req PDU has been rejected.
    invalid_lmp_parameters = 0x021e, // Some LMP PDU / LL Control PDU parameters were invalid.
    unspecified_error = 0x021f,     // No other error code specified is appropriate to use.
    unsupported_lmp_parameter_value = 0x0220, // An LMP PDU or an LL Control PDU contains at least one parameter value that is not supported by the Controller at this time.
    role_change_not_allowed = 0x0221, // Controller will not allow a role change at this time.
    ll_response_timeout = 0x0222,     // Connection terminated due to link-layer procedure timeout.
    lmp_error_transaction_collision = 0x0223, // LMP transaction has collided with the same transaction that is already in progress.
    lmp_pdu_not_allowed = 0x0224, // Controller sent an LMP PDU with an OpCode that was not allowed.
    encryption_mode_not_acceptable = 0x0225, // The requested encryption mode is not acceptable at this time.
    link_key_cannot_be_changed = 0x0226, // Link key cannot be changed because a fixed unit key is being used.
    requested_qos_not_supported = 0x0227, // The requested Quality of Service is not supported.
    instant_passed = 0x0228, // LMP PDU or LL PDU that includes an instant cannot be performed because the instant when this would have occurred has passed.
    pairing_with_unit_key_not_supported = 0x0229, // It was not possible to pair as a unit key was requested and it is not supported.
    different_transaction_collision = 0x022a, // LMP transaction was started that collides with an ongoing transaction.
    qos_unacceptable_parameter = 0x022c, // The specified quality of service parameters could not be accepted at this time, but other parameters may be acceptable.
    qos_rejected = 0x022d, // The specified quality of service parameters cannot be accepted and QoS negotiation should be terminated.
    channel_assesment_not_supported = 0x022e, // The Controller cannot perform channel assessment because it is not supported.
    insufficient_security = 0x022f, // The HCI command or LMP PDU sent is only possible on an encrypted link.
    parameter_out_of_mandatory_range = 0x0230, // A parameter value requested is outside the mandatory range of parameters for the given HCI command or LMP PDU.
    role_switch_pending = 0x0232, // Role Switch is pending. This can be used when an HCI command or LMP PDU cannot be accepted because of a pending role switch. This can also be used to notify a peer device about a pending role switch.
    reserved_slot_violation = 0x0234, // The current Synchronous negotiation was terminated with the negotiation state set to Reserved Slot Violation.
    role_switch_failed = 0x0235, // role switch was attempted but it failed and the original piconet structure is restored. The switch may have failed because the TDD switch or piconet switch failed.
    extended_inquiry_response_too_large = 0x0236, // The extended inquiry response, with the requested requirements for FEC, is too large to fit in any of the packet types supported by the Controller.
    simple_pairing_not_supported_by_host = 0x0237, // The IO capabilities request or response was rejected because the sending Host does not support Secure Simple Pairing even though the receiving Link Manager does.
    host_busy_pairing = 0x0238, // The Host is busy with another pairing operation and unable to support the requested pairing. The receiving device should retry pairing again later.
    connection_rejected_due_to_no_suitable_channel_found = 0x0239, // The Controller could not calculate an appropriate value for the Channel selection operation.
    controller_busy = 0x023a, // Operation was rejected because the controller is busy and unable to process the request.
    unacceptable_connection_interval = 0x023b, // Remote device terminated the connection because of an unacceptable connection interval.
    directed_advertising_timeout = 0x023c, // Directed advertising completed without a connection being created.
    connection_terminated_due_to_mic_failure = 0x023d, // Connection was terminated because the Message Integrity Check (MIC) failed on a received packet.
    connection_failed_to_be_established = 0x023e, // LL initiated a connection but the connection has failed to be established. Controller did not receive any packets from remote end.
    mac_connection_failed = 0x023f, // The MAC of the 802.11 AMP was requested to connect to a peer, but the connection failed.
    coarse_clock_adjustment_rejected_but_will_try_to_adjust_using_clock_dragging = 0x0240, // The master, at this time, is unable to make a coarse adjustment to the piconet clock, using the supplied parameters. Instead the master will attempt to move the clock using clock dragging.

    // Application errors
    file_open_failed = 0x0a01,                          // File open failed.
    xml_parse_failed = 0x0a02,                          // XML parsing failed.
    device_connection_failed = 0x0a03,                  // Device connection failed.
    device_comunication_failed = 0x0a04,                // Device communication failed.
    authentication_failed = 0x0a05,                     // Device authentication failed.
    incorrect_gatt_database = 0x0a06,                   // Device has incorrect GATT database.
    disconnected_due_to_procedure_collision = 0x0a07, // Device disconnected due to procedure collision.
    disconnected_due_to_secure_session_failed = 0x0a08, // Device disconnected due to failure to establish or reestablish a secure session.
    encryption_decryption_error = 0x0a09,               // Encrypion/decryption operation failed.
    maximum_retries = 0x0a0a,                           // Maximum allowed retries exceeded.
    data_parse_failed = 0x0a0b,                         // Data parsing failed.
    pairing_removed = 0x0a0c, // Pairing established by the application layer protocol has been removed.
    inactive_timeout = 0x0a0d, // Inactive timeout.

    // Errors from Attribute Protocol
    invalid_handle = 0x0401, // The attribute handle given was not valid on this server
    read_not_permitted = 0x0402, // The attribute cannot be read
    write_not_permitted = 0x0403, // The attribute cannot be written
    invalid_pdu = 0x0404,    // The attribute PDU was invalid
    insufficient_authentication = 0x0405, // The attribute requires authentication before it can be read or written.
    request_not_supported = 0x0406, // Attribute Server does not support the request received from the client.
    invalid_offset = 0x0407,        // Offset specified was past the end of the attribute
    insufficient_authorization = 0x0408, // The attribute requires authorization before it can be read or written.
    prepare_queue_full = 0x0409,         // Too many prepare writes have been queueud
    att_not_found = 0x040a, // No attribute found within the given attribute handle range.
    att_not_long = 0x040b,  // The attribute cannot be read or written using the Read Blob Request
    insufficient_enc_key_size = 0x040c, // The Encryption Key Size used for encrypting this link is insufficient.
    invalid_att_length = 0x040d,        // The attribute value length is invalid for the operation
    unlikely_error = 0x040e, // The attribute request that was requested has encountered an error that was unlikely, and therefore could not be completed as requested.
    insufficient_encryption = 0x040f, // The attribute requires encryption before it can be read or written.
    unsupported_group_type = 0x0410, // The attribute type is not a supported grouping attribute as defined by a higher layer specification.
    insufficient_resources = 0x0411, // Insufficient Resources to complete the request
    out_of_sync = 0x0412,            // The server requests the client to rediscover the database.
    value_not_allowed = 0x0413,      // The attribute parameter value was not allowed.
    application = 0x0480, // When this is returned in a BGAPI response, the application tried to read or write the value of a user attribute from the GATT database.

    // Bluetooth Mesh errors
    already_exists = 0x0c01, // Returned when trying to add a key or some other unique resource with an ID which already exists
    does_not_exist = 0x0c02, // Returned when trying to manipulate a key or some other resource with an ID which does not exist
    limit_reached = 0x0c03, // Returned when an operation cannot be executed because a pre-configured limit for keys, key bindings, elements, models, virtual addresses, provisioned devices, or provisioning sessions is reached
    invalid_address = 0x0c04, // Returned when trying to use a reserved address or add a "pre-provisioned" device using an address already used by some other device
    malformed_data = 0x0c05, // In a BGAPI response, the user supplied malformed data; in a BGAPI event, the remote end responded with malformed or unrecognized data

    // Filesystem errors
    file_not_found = 0x0901, // File not found

    // Security errors
    image_signature_verification_failed = 0x0b01, // Device firmware signature verification failed.
    file_signature_verification_failed = 0x0b02,  // File signature verification failed.
    image_checksum_error = 0x0b03,                // Device firmware checksum is not valid.
}
