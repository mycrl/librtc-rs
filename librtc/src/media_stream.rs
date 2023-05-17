use anyhow::Result;
use std::sync::Arc;
use libc::*;
use super::{
    media_stream_track::*,
    cstr::*,
};

/// The MediaStream interface represents a stream of media content.
///
/// A stream consists of several tracks, such as video or audio tracks. Each
/// track is specified as an instance of MediaStreamTrack.
pub struct MediaStream {
    pub id: String,
    pub tracks: Vec<MediaStreamTrack>,
    pub(crate) raw_id: *const c_char,
}

unsafe impl Send for MediaStream {}
unsafe impl Sync for MediaStream {}

impl MediaStream {
    /// Creates and returns a new MediaStream object. You can create an empty
    /// stream, a stream which is based upon an existing stream, or a stream
    /// that contains a specified list of tracks.
    pub fn new(id: &str) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            tracks: Vec::with_capacity(10),
            raw_id: to_c_str(id)?,
            id: id.to_string(),
        }))
    }

    /// A string containing a 36-character universally unique identifier (UUID)
    /// for the object.
    pub(crate) fn get_id(&self) -> *const c_char {
        self.raw_id
    }
}

impl Drop for MediaStream {
    fn drop(&mut self) {
        free_cstring(self.raw_id);
    }
}
