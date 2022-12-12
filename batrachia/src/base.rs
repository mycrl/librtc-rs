use anyhow::Result;
use libc::*;
use std::ffi::{
    CString,
    CStr,
};

/// ```no_run
/// let c_str = to_c_str("test").unwrap();
/// assert!(!c_str.is_null());
/// ```
pub(crate) fn to_c_str(str: &str) -> Result<*const c_char> {
    Ok(CString::new(str)?.into_raw())
}

/// ```no_run
/// let c_str = to_c_str("test").unwrap();
/// assert!(!c_str.is_null());
/// let str = from_c_str(c_str).unwrap();
/// assert_eq!(&str, "test");
/// ```
pub(crate) fn from_c_str(str: *const c_char) -> Result<String> {
    assert!(!str.is_null());
    Ok(unsafe { CStr::from_ptr(str).to_str()?.to_string() })
}

/// ```no_run
/// let c_str = to_c_str("test").unwrap();
/// assert!(!c_str.is_null());
///
/// free_cstring(c_str);
/// ```
pub(crate) fn free_cstring(str: *const c_char) {
    assert!(!str.is_null());
    drop(unsafe { CString::from_raw(str as *mut c_char) })
}

/// ```no_run
/// let c_str = to_c_str("test").unwrap();
/// assert!(!c_str.is_null());
///
/// let raw = from_raw_ptr(c_str);
/// assert!(raw.is_some());
/// ```
pub(crate) fn from_raw_ptr<T>(ptr: *const T) -> Option<*const T> {
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

/// ```no_run
/// let c_str = to_c_str("test").unwrap();
/// assert!(!c_str.is_null());
///
/// let raw_mut = from_raw_mut_ptr(c_str);
/// assert!(raw.is_some());
/// ```
pub(crate) fn from_raw_mut_ptr<T>(ptr: *mut T) -> Option<*mut T> {
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}
