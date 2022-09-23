use anyhow::Result;
use libc::*;
use std::ffi::CString;

pub fn free_cstring(c_str: *mut c_char) {
    if !c_str.is_null() {
        drop(unsafe { std::ffi::CString::from_raw(c_str) })
    }
}

pub fn from_raw_ptr<T>(ptr: *const T) -> Option<*const T> {
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

pub fn from_raw_mut_ptr<T>(ptr: *mut T) -> Option<*mut T> {
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

pub fn cstr_to_string(cstr: *const c_char) -> Result<String> {
    Ok(unsafe { CString::from_raw(cstr as *mut c_char).into_string() }?)
}
