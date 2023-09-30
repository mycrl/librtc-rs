use std::ffi::{c_char, CStr, CString};

#[derive(Debug)]
pub enum StringError {
    NulError,
    Utf8Error,
}

pub(crate) fn to_c_str(str: &str) -> Result<*const c_char, StringError> {
    Ok(CString::new(str)
        .map_err(|_| StringError::NulError)?
        .into_raw())
}

pub(crate) fn from_c_str(str: *const c_char) -> Result<String, StringError> {
    assert!(!str.is_null());
    Ok(unsafe {
        CStr::from_ptr(str)
            .to_str()
            .map_err(|_| StringError::Utf8Error)?
            .to_string()
    })
}

pub(crate) fn c_str_to_str(str: *const c_char) -> Result<&'static str, StringError> {
    Ok(unsafe {
        CStr::from_ptr(str)
            .to_str()
            .map_err(|_| StringError::Utf8Error)?
    })
}

pub(crate) fn free_cstring(str: *const c_char) {
    if !str.is_null() {
        drop(unsafe { CString::from_raw(str as *mut c_char) })
    }
}
