use anyhow::Result;
use libc::*;
use std::ffi::{
    CStr,
    CString,
};
use std::mem::ManuallyDrop;

pub(crate) fn to_c_str(str: &str) -> Result<*const c_char> {
    Ok(CString::new(str)?.into_raw())
}

pub(crate) fn from_c_str(str: *const c_char) -> Result<String> {
    Ok(unsafe { CStr::from_ptr(str).to_str()?.to_string() })
}

pub(crate) fn free_cstring(c_str: *const c_char) {
    if !c_str.is_null() {
        drop(unsafe { CString::from_raw(c_str as *mut c_char) })
    }
}

pub(crate) fn from_raw_ptr<T>(ptr: *const T) -> Option<*const T> {
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

pub(crate) fn from_raw_mut_ptr<T>(ptr: *mut T) -> Option<*mut T> {
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

pub(crate) fn cstr_to_string(cstr: *const c_char) -> Result<String> {
    Ok(unsafe { CStr::from_ptr(cstr as *mut c_char).to_str()?.to_string() })
}

pub(crate) trait VecExt<T> {
    fn ext_into_raw_parts(self) -> (*mut T, usize, usize);
}

impl<T> VecExt<T> for Vec<T> {
    fn ext_into_raw_parts(self) -> (*mut T, usize, usize) {
        let mut me = ManuallyDrop::new(self);
        (me.as_mut_ptr(), me.len(), me.capacity())
    }
}
