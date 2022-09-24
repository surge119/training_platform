use std::ffi::CString;
use std::os::raw::c_char;

pub fn str_to_c_const_char(r_str: &str) -> *const c_char {
    let c_str: CString = CString::new(r_str).unwrap();
    return c_str.as_ptr();
}
