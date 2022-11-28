use std::mem::ManuallyDrop;
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
    Ok(unsafe { CStr::from_ptr(str).to_str()?.to_string() })
}

/// ```no_run
/// let c_str = to_c_str("test").unwrap();
/// assert!(!c_str.is_null());
///
/// free_cstring(c_str);
/// ```
pub(crate) fn free_cstring(c_str: *const c_char) {
    if !c_str.is_null() {
        drop(unsafe { CString::from_raw(c_str as *mut c_char) })
    }
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

/// ```no_run
/// let vec = vec![0u8; 10];
/// let (ptr, size, capacity) = vec.into_c_layout();
/// assert!(ptr.is_null());
/// assert_eq!(capacity, 10);
/// assert_eq!(size, 10);
/// ```
pub(crate) trait VectorLayout<T> {
    fn into_c_layout(self) -> (*mut T, usize, usize);
}

impl<T> VectorLayout<T> for Vec<T> {
    fn into_c_layout(self) -> (*mut T, usize, usize) {
        let mut me = ManuallyDrop::new(self);
        (me.as_mut_ptr(), me.len(), me.capacity())
    }
}
