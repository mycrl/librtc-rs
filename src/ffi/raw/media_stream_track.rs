use libc::*;

#[repr(C)]
pub struct MediaStreamTrack {
    pub enabled: bool,
    pub id: *const c_char,
    pub kind: *const c_char,
    pub label: *const c_char,
    pub muted: bool,
    pub ready_state: bool,
    pub remote: bool,
    pub width: u32,
    pub height: u32,
    pub frame_rate: c_int,
}

#[repr(C)]
pub struct MediaStreamTrackFrame {
    pub buf: *const c_char,
    pub len: u64,
}
