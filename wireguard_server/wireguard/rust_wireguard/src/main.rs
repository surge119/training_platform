use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::os::raw::{c_char, c_uint};

mod base;

use base::base_api;

extern "C" {
    fn base64_private_key() -> *const c_char;
    fn test1(pr: *const c_char);
    fn generate_private_key() -> *mut c_char;
}

fn main() {
    unsafe {
        let mut key = generate_private_key();
        // let kkey = CStr::from_ptr(key);
        // let keey = kkey.to_str().unwrap();
        // println!("RKey: {:?}", *key[1]);
        print!("RKey: ");
        for i in 0..32 {
            print!("{} | ", *key);
            *key += 1;
        }
        println!();
        // base_api::create_server_device("test-device", 51280, )
    }
}