// jkcoxson

pub struct PreferencePlist {
    pub activation_state: String,
    pub activation_state_acknowledged: bool,
    pub baseband_activation_ticket_version: String,
    pub baseband_cert_id: String,
    pub baseband_chip_id: String,
    pub baseband_hash_information: BasebandHashInformation,
    pub baseband_master_key_hash: String,
    pub baseband_region_sku: String,
    pub baseband_serial_number: String,
}

pub struct BasebandHashInformation {
    pub a_key_status: i8,
    pub s_key_hash: String,
    pub s_key_status: i8,
}
