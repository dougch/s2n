
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0


#![allow(unused_imports, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use libc::{iovec, FILE};

#[test]
fn s2n_async_pkey_op_apply () {
    let ptr = crate::tests::s2n_async_pkey_op_apply as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_async_pkey_op_free () {
    let ptr = crate::tests::s2n_async_pkey_op_free as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_async_pkey_op_perform () {
    let ptr = crate::tests::s2n_async_pkey_op_perform as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_calculate_stacktrace () {
    let ptr = crate::tests::s2n_calculate_stacktrace as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_chain_and_key_free () {
    let ptr = crate::tests::s2n_cert_chain_and_key_free as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_chain_and_key_get_ctx () {
    let ptr = crate::tests::s2n_cert_chain_and_key_get_ctx as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_chain_and_key_get_private_key () {
    let ptr = crate::tests::s2n_cert_chain_and_key_get_private_key as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_chain_and_key_load_pem () {
    let ptr = crate::tests::s2n_cert_chain_and_key_load_pem as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_chain_and_key_new () {
    let ptr = crate::tests::s2n_cert_chain_and_key_new as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_chain_and_key_set_ctx () {
    let ptr = crate::tests::s2n_cert_chain_and_key_set_ctx as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_chain_get_cert () {
    let ptr = crate::tests::s2n_cert_chain_get_cert as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_chain_get_length () {
    let ptr = crate::tests::s2n_cert_chain_get_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_get_der () {
    let ptr = crate::tests::s2n_cert_get_der as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_get_utf8_string_from_extension_data () {
    let ptr = crate::tests::s2n_cert_get_utf8_string_from_extension_data as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_get_utf8_string_from_extension_data_length () {
    let ptr = crate::tests::s2n_cert_get_utf8_string_from_extension_data_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_get_x509_extension_value () {
    let ptr = crate::tests::s2n_cert_get_x509_extension_value as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cert_get_x509_extension_value_length () {
    let ptr = crate::tests::s2n_cert_get_x509_extension_value_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_cleanup () {
    let ptr = crate::tests::s2n_cleanup as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_cb_done () {
    let ptr = crate::tests::s2n_client_hello_cb_done as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_get_cipher_suites () {
    let ptr = crate::tests::s2n_client_hello_get_cipher_suites as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_get_cipher_suites_length () {
    let ptr = crate::tests::s2n_client_hello_get_cipher_suites_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_get_extension_by_id () {
    let ptr = crate::tests::s2n_client_hello_get_extension_by_id as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_get_extension_length () {
    let ptr = crate::tests::s2n_client_hello_get_extension_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_get_extensions () {
    let ptr = crate::tests::s2n_client_hello_get_extensions as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_get_extensions_length () {
    let ptr = crate::tests::s2n_client_hello_get_extensions_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_get_raw_message () {
    let ptr = crate::tests::s2n_client_hello_get_raw_message as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_client_hello_get_raw_message_length () {
    let ptr = crate::tests::s2n_client_hello_get_raw_message_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_accept_max_fragment_length () {
    let ptr = crate::tests::s2n_config_accept_max_fragment_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_add_cert_chain_and_key () {
    let ptr = crate::tests::s2n_config_add_cert_chain_and_key as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_add_cert_chain_and_key_to_store () {
    let ptr = crate::tests::s2n_config_add_cert_chain_and_key_to_store as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_add_dhparams () {
    let ptr = crate::tests::s2n_config_add_dhparams as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_add_pem_to_trust_store () {
    let ptr = crate::tests::s2n_config_add_pem_to_trust_store as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_add_ticket_crypto_key () {
    let ptr = crate::tests::s2n_config_add_ticket_crypto_key as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_append_protocol_preference () {
    let ptr = crate::tests::s2n_config_append_protocol_preference as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_disable_x509_verification () {
    let ptr = crate::tests::s2n_config_disable_x509_verification as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_enable_cert_req_dss_legacy_compat () {
    let ptr = crate::tests::s2n_config_enable_cert_req_dss_legacy_compat as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_free () {
    let ptr = crate::tests::s2n_config_free as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_free_cert_chain_and_key () {
    let ptr = crate::tests::s2n_config_free_cert_chain_and_key as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_free_dhparams () {
    let ptr = crate::tests::s2n_config_free_dhparams as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_get_client_auth_type () {
    let ptr = crate::tests::s2n_config_get_client_auth_type as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_new () {
    let ptr = crate::tests::s2n_config_new as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_send_max_fragment_length () {
    let ptr = crate::tests::s2n_config_send_max_fragment_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_alert_behavior () {
    let ptr = crate::tests::s2n_config_set_alert_behavior as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_async_pkey_callback () {
    let ptr = crate::tests::s2n_config_set_async_pkey_callback as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_cache_delete_callback () {
    let ptr = crate::tests::s2n_config_set_cache_delete_callback as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_cache_retrieve_callback () {
    let ptr = crate::tests::s2n_config_set_cache_retrieve_callback as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_cache_store_callback () {
    let ptr = crate::tests::s2n_config_set_cache_store_callback as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_cert_chain_and_key_defaults () {
    let ptr = crate::tests::s2n_config_set_cert_chain_and_key_defaults as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_cert_tiebreak_callback () {
    let ptr = crate::tests::s2n_config_set_cert_tiebreak_callback as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_check_stapled_ocsp_response () {
    let ptr = crate::tests::s2n_config_set_check_stapled_ocsp_response as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_cipher_preferences () {
    let ptr = crate::tests::s2n_config_set_cipher_preferences as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_client_auth_type () {
    let ptr = crate::tests::s2n_config_set_client_auth_type as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_client_hello_cb () {
    let ptr = crate::tests::s2n_config_set_client_hello_cb as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_client_hello_cb_mode () {
    let ptr = crate::tests::s2n_config_set_client_hello_cb_mode as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_ct_support_level () {
    let ptr = crate::tests::s2n_config_set_ct_support_level as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_extension_data () {
    let ptr = crate::tests::s2n_config_set_extension_data as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_key_log_cb () {
    let ptr = crate::tests::s2n_config_set_key_log_cb as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_max_cert_chain_depth () {
    let ptr = crate::tests::s2n_config_set_max_cert_chain_depth as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_monotonic_clock () {
    let ptr = crate::tests::s2n_config_set_monotonic_clock as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_protocol_preferences () {
    let ptr = crate::tests::s2n_config_set_protocol_preferences as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_session_cache_onoff () {
    let ptr = crate::tests::s2n_config_set_session_cache_onoff as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_session_state_lifetime () {
    let ptr = crate::tests::s2n_config_set_session_state_lifetime as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_session_tickets_onoff () {
    let ptr = crate::tests::s2n_config_set_session_tickets_onoff as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_status_request_type () {
    let ptr = crate::tests::s2n_config_set_status_request_type as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_ticket_decrypt_key_lifetime () {
    let ptr = crate::tests::s2n_config_set_ticket_decrypt_key_lifetime as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_ticket_encrypt_decrypt_key_lifetime () {
    let ptr = crate::tests::s2n_config_set_ticket_encrypt_decrypt_key_lifetime as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_verification_ca_location () {
    let ptr = crate::tests::s2n_config_set_verification_ca_location as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_verify_host_callback () {
    let ptr = crate::tests::s2n_config_set_verify_host_callback as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_config_set_wall_clock () {
    let ptr = crate::tests::s2n_config_set_wall_clock as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_append_protocol_preference () {
    let ptr = crate::tests::s2n_connection_append_protocol_preference as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_client_cert_used () {
    let ptr = crate::tests::s2n_connection_client_cert_used as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_free () {
    let ptr = crate::tests::s2n_connection_free as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_free_handshake () {
    let ptr = crate::tests::s2n_connection_free_handshake as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_actual_protocol_version () {
    let ptr = crate::tests::s2n_connection_get_actual_protocol_version as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_alert () {
    let ptr = crate::tests::s2n_connection_get_alert as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_cipher () {
    let ptr = crate::tests::s2n_connection_get_cipher as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_cipher_iana_value () {
    let ptr = crate::tests::s2n_connection_get_cipher_iana_value as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_client_auth_type () {
    let ptr = crate::tests::s2n_connection_get_client_auth_type as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_client_cert_chain () {
    let ptr = crate::tests::s2n_connection_get_client_cert_chain as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_client_hello () {
    let ptr = crate::tests::s2n_connection_get_client_hello as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_client_hello_version () {
    let ptr = crate::tests::s2n_connection_get_client_hello_version as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_client_protocol_version () {
    let ptr = crate::tests::s2n_connection_get_client_protocol_version as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_ctx () {
    let ptr = crate::tests::s2n_connection_get_ctx as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_curve () {
    let ptr = crate::tests::s2n_connection_get_curve as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_delay () {
    let ptr = crate::tests::s2n_connection_get_delay as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_handshake_type_name () {
    let ptr = crate::tests::s2n_connection_get_handshake_type_name as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_kem_group_name () {
    let ptr = crate::tests::s2n_connection_get_kem_group_name as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_kem_name () {
    let ptr = crate::tests::s2n_connection_get_kem_name as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_last_message_name () {
    let ptr = crate::tests::s2n_connection_get_last_message_name as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_ocsp_response () {
    let ptr = crate::tests::s2n_connection_get_ocsp_response as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_peer_cert_chain () {
    let ptr = crate::tests::s2n_connection_get_peer_cert_chain as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_sct_list () {
    let ptr = crate::tests::s2n_connection_get_sct_list as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_selected_cert () {
    let ptr = crate::tests::s2n_connection_get_selected_cert as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_server_protocol_version () {
    let ptr = crate::tests::s2n_connection_get_server_protocol_version as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_session () {
    let ptr = crate::tests::s2n_connection_get_session as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_session_id () {
    let ptr = crate::tests::s2n_connection_get_session_id as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_session_id_length () {
    let ptr = crate::tests::s2n_connection_get_session_id_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_session_length () {
    let ptr = crate::tests::s2n_connection_get_session_length as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_session_ticket_lifetime_hint () {
    let ptr = crate::tests::s2n_connection_get_session_ticket_lifetime_hint as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_wire_bytes_in () {
    let ptr = crate::tests::s2n_connection_get_wire_bytes_in as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_get_wire_bytes_out () {
    let ptr = crate::tests::s2n_connection_get_wire_bytes_out as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_is_ocsp_stapled () {
    let ptr = crate::tests::s2n_connection_is_ocsp_stapled as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_is_session_resumed () {
    let ptr = crate::tests::s2n_connection_is_session_resumed as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_is_valid_for_cipher_preferences () {
    let ptr = crate::tests::s2n_connection_is_valid_for_cipher_preferences as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_new () {
    let ptr = crate::tests::s2n_connection_new as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_prefer_low_latency () {
    let ptr = crate::tests::s2n_connection_prefer_low_latency as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_prefer_throughput () {
    let ptr = crate::tests::s2n_connection_prefer_throughput as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_release_buffers () {
    let ptr = crate::tests::s2n_connection_release_buffers as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_server_name_extension_used () {
    let ptr = crate::tests::s2n_connection_server_name_extension_used as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_blinding () {
    let ptr = crate::tests::s2n_connection_set_blinding as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_cipher_preferences () {
    let ptr = crate::tests::s2n_connection_set_cipher_preferences as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_client_auth_type () {
    let ptr = crate::tests::s2n_connection_set_client_auth_type as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_config () {
    let ptr = crate::tests::s2n_connection_set_config as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_ctx () {
    let ptr = crate::tests::s2n_connection_set_ctx as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_dynamic_record_threshold () {
    let ptr = crate::tests::s2n_connection_set_dynamic_record_threshold as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_fd () {
    let ptr = crate::tests::s2n_connection_set_fd as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_protocol_preferences () {
    let ptr = crate::tests::s2n_connection_set_protocol_preferences as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_read_fd () {
    let ptr = crate::tests::s2n_connection_set_read_fd as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_recv_cb () {
    let ptr = crate::tests::s2n_connection_set_recv_cb as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_recv_ctx () {
    let ptr = crate::tests::s2n_connection_set_recv_ctx as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_send_cb () {
    let ptr = crate::tests::s2n_connection_set_send_cb as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_send_ctx () {
    let ptr = crate::tests::s2n_connection_set_send_ctx as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_session () {
    let ptr = crate::tests::s2n_connection_set_session as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_verify_host_callback () {
    let ptr = crate::tests::s2n_connection_set_verify_host_callback as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_set_write_fd () {
    let ptr = crate::tests::s2n_connection_set_write_fd as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_use_corked_io () {
    let ptr = crate::tests::s2n_connection_use_corked_io as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_connection_wipe () {
    let ptr = crate::tests::s2n_connection_wipe as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_errno_location () {
    let ptr = crate::tests::s2n_errno_location as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_error_get_type () {
    let ptr = crate::tests::s2n_error_get_type as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_free_stacktrace () {
    let ptr = crate::tests::s2n_free_stacktrace as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_get_application_protocol () {
    let ptr = crate::tests::s2n_get_application_protocol as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_get_openssl_version () {
    let ptr = crate::tests::s2n_get_openssl_version as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_get_server_name () {
    let ptr = crate::tests::s2n_get_server_name as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_get_stacktrace () {
    let ptr = crate::tests::s2n_get_stacktrace as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_init () {
    let ptr = crate::tests::s2n_init as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_mem_set_callbacks () {
    let ptr = crate::tests::s2n_mem_set_callbacks as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_negotiate () {
    let ptr = crate::tests::s2n_negotiate as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_peek () {
    let ptr = crate::tests::s2n_peek as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_print_stacktrace () {
    let ptr = crate::tests::s2n_print_stacktrace as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_rand_set_callbacks () {
    let ptr = crate::tests::s2n_rand_set_callbacks as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_recv () {
    let ptr = crate::tests::s2n_recv as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_send () {
    let ptr = crate::tests::s2n_send as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_sendv () {
    let ptr = crate::tests::s2n_sendv as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_sendv_with_offset () {
    let ptr = crate::tests::s2n_sendv_with_offset as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_set_server_name () {
    let ptr = crate::tests::s2n_set_server_name as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_shutdown () {
    let ptr = crate::tests::s2n_shutdown as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_stack_traces_enabled () {
    let ptr = crate::tests::s2n_stack_traces_enabled as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_stack_traces_enabled_set () {
    let ptr = crate::tests::s2n_stack_traces_enabled_set as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_strerror () {
    let ptr = crate::tests::s2n_strerror as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_strerror_debug () {
    let ptr = crate::tests::s2n_strerror_debug as *const ();
    assert!(!ptr.is_null());
}

#[test]
fn s2n_strerror_name () {
    let ptr = crate::tests::s2n_strerror_name as *const ();
    assert!(!ptr.is_null());
}

