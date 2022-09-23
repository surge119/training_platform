use crate::base::base_api;

extern "C" {
    fn test() -> u32;
}

#[no_mangle]
pub extern "C" fn test_bind() -> u32 {
    unsafe {
        return test();
    }
}

pub fn testt() -> u32 {
    unsafe {
        test()
    }
}