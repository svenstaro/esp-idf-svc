// There are different implementations for the config for different boards. See also
// https://github.com/espressif/esp-idf/tree/master/components/bt/include
#[cfg(esp32)]
struct EspBluetoothControllerConfiguration {
    controller_task_stack_size: u16,
    controller_task_prio: u8,
    hci_uart_no: u8,
    hci_uart_baudrate: u8,
    scan_duplicate_mode: u8,
    scan_duplicate_type: u8,
    normal_adv_size: u16,
    mesh_adv_size: u16,
    send_adv_reserved_size: u16,
    controller_debug_flag: u8,
    mode: u8,
    ble_max_conn: u8,
    bt_max_acl_conn: u8,
    bt_sco_datapath: esp_idf_sysu8,
    auto_latency: bool,
    bt_legacy_auth_vs_evt: bool,
    bt_max_sync_conn: u8,
    ble_sca: u8,
    pcm_role: u8,
    pcm_polar: u8,
    hli: bool,
    magic: u8,
}

#[cfg(esp32)]
impl Default for EspBluetoothControllerConfiguration {
    fn default() -> Self {
        Self {
            controller_task_stack_size: esp_idf_sys::ESP_TASK_BT_CONTROLLER_STACK as u16,
            controller_task_prio: esp_idf_sys::ESP_TASK_BT_CONTROLLER_PRIO as u8,
            hci_uart_no: esp_idf_sys::BT_HCI_UART_NO_DEFAULT as u8,
            hci_uart_baudrate: esp_idf_sys::BT_HCI_UART_BAUDRATE_DEFAULT,
            scan_duplicate_mode: esp_idf_sys::SCAN_DUPLICATE_MODE as u8,
            scan_duplicate_type: esp_idf_sys::SCAN_DUPLICATE_TYPE_VALUE as u8,
            normal_adv_size: esp_idf_sys::NORMAL_SCAN_DUPLICATE_CACHE_SIZE as u16,
            mesh_adv_size: esp_idf_sys::MESH_DUPLICATE_SCAN_CACHE_SIZE as u16,
            send_adv_reserved_size: esp_idf_sys::SCAN_SEND_ADV_RESERVED_SIZE as u16,
            controller_debug_flag: esp_idf_sys::CONTROLLER_ADV_LOST_DEBUG_BIT,
            mode: esp_idf_sys::esp_bt_mode_t_ESP_BT_MODE_BLE as u8,
            ble_max_conn: esp_idf_sys::CONFIG_BTDM_CTRL_BLE_MAX_CONN_EFF as u8,
            bt_max_acl_conn: esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_MAX_ACL_CONN_EFF as u8,
            bt_sco_datapath: esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_SCO_DATA_PATH_EFF as u8,
            auto_latency: esp_idf_sys::BTDM_CTRL_AUTO_LATENCY_EFF != 0,
            bt_legacy_auth_vs_evt: esp_idf_sys::BTDM_CTRL_LEGACY_AUTH_VENDOR_EVT_EFF != 0,
            bt_max_sync_conn: esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_MAX_SYNC_CONN_EFF as u8,
            ble_sca: esp_idf_sys::CONFIG_BTDM_BLE_SLEEP_CLOCK_ACCURACY_INDEX_EFF as u8,
            pcm_role: esp_idf_sys::CONFIG_BTDM_CTRL_PCM_ROLE_EFF as u8,
            pcm_polar: esp_idf_sys::CONFIG_BTDM_CTRL_PCM_POLAR_EFF as u8,
            hli: esp_idf_sys::BTDM_CTRL_HLI != 0,
            magic: esp_idf_sys::ESP_BT_CONTROLLER_CONFIG_MAGIC_VAL,
        }
    }
}

// #[cfg(esp32c3)]
// #[cfg(esp32h2)]
// #[cfg(esp32s3)]
