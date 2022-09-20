use libc::*;

pub fn free_cstring(c_str: *mut c_char) {
    if !c_str.is_null() {
        drop(unsafe { std::ffi::CString::from_raw(c_str) })
    }
}
