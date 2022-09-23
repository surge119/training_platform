mod base;
mod bindings;

extern "C" {
    // fn wg_generate_private_key(private_key: &mut *mut c_uint);
    // fn test();
    fn generate_key();
}

fn main() {
    unsafe {
        generate_key();
    }
}