use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn create_string() -> *mut c_char {
    let s = CString::new("Hello from Rust!").unwrap();
    s.into_raw() // Transfers ownership to the caller (C++)
}


// 外部“C”函数接口，创建并最终释放一个CString
#[no_mangle]
pub extern "C" fn combine() -> *mut c_char {
    // 创建一个CString
    let s = CString::new("Hello from Rust!").unwrap();
    let ptr = s.into_raw(); // 将所有权转移给调用者

    // 假设在一些操作后需要释放这个字符串
    // 通常这部分应该由调用者在适当的时候调用
    // 这里只是为了示例将创建和释放放在一个函数里
    free_string(ptr); // 使用完毕后释放字符串
    std::ptr::null_mut() // 返回空指针，表示不再持有任何有用的数据
}



#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        CString::from_raw(s); // Reclaims ownership and drops the CString, freeing the memory
    }
}
