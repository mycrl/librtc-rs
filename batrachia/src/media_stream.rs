use super::base::*;
use super::media_stream_track::*;
use std::sync::Arc;
use anyhow::Result;
use libc::*;

#[derive(Debug)]
pub struct MediaStream {
    pub id: String,
    pub tracks: Vec<MediaStreamTrack>,
    pub(crate) raw_id: *const c_char,
}

unsafe impl Send for MediaStream {}
unsafe impl Sync for MediaStream {}

impl MediaStream {
    pub fn new(id: String) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            raw_id: to_c_str(&id)?,
            tracks: Vec::with_capacity(10),
            id,
        }))
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
