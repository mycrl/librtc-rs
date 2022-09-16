

pub struct Events {
    on_connectionstatechange: extern "C" fn(*const c_char, *const RawRTCSessionDescription, *mut c_void),
}
