use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn create_string() -> *mut c_char {
    let s = CString::new("Hello from Rust!").unwrap();
    s.into_raw() // Transfers ownership to the caller (C++)
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        CString::from_raw(s); // Reclaims ownership and drops the CString, freeing the memory
    }
}
