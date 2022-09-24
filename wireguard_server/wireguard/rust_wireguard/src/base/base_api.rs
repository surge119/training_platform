use std::os::raw::{c_char, c_uint};

use crate::base::util;
use crate::base::util::str_to_c_const_char;

extern "C" {
    fn add_server_device(device_name: *const c_char, port: u16, private_key: *const c_char);
}

pub unsafe fn create_server_device(device_name: &str, port: u16, private_key: &str) {
    let c_device_name: *const c_char = str_to_c_const_char(device_name);
    let c_private_key: *const c_char = str_to_c_const_char(private_key);

    add_server_device(c_device_name, port, c_private_key);
}