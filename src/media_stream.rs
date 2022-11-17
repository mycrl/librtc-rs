use super::base::*;
use super::media_stream_track::*;
use libc::*;
use std::ffi::CString;

pub struct MediaStream {
    pub id: String,
    pub tracks: Vec<MediaStreamTrack>,
    pub(crate) raw_id: *mut c_char,
}

unsafe impl Send for MediaStream {}
unsafe impl Sync for MediaStream {}

impl MediaStream {
    pub fn new(id: String) -> Self {
        Self {
            raw_id: CString::new(id.clone()).unwrap().into_raw(),
            tracks: Vec::with_capacity(10),
            id,
        }
    }

    pub(crate) fn get_id(&self) -> *const c_char {
        self.raw_id
    }
}

impl Drop for MediaStream {
    fn drop(&mut self) {
        free_cstring(self.raw_id);
    }
}
