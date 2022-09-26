use anyhow::Result;
use libc::*;
use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;

pub fn free_cstring(c_str: *mut c_char) {
    if !c_str.is_null() {
        drop(unsafe { CString::from_raw(c_str) })
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
    Ok(unsafe { CStr::from_ptr(cstr as *mut c_char).to_str()?.to_string() })
}
pub trait VecExt<T> {
    fn ext_into_raw_parts(self) -> (*mut T, usize, usize);
}

impl<T> VecExt<T> for Vec<T> {
    fn ext_into_raw_parts(self) -> (*mut T, usize, usize) {
        let mut me = ManuallyDrop::new(self);
        (me.as_mut_ptr(), me.len(), me.capacity())
    }
}
