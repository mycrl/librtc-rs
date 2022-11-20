use super::base::*;
use super::media_stream_track::*;
use std::sync::Arc;
use anyhow::Result;
use libc::*;

/// The MediaStream interface represents a stream of media content
/// 
/// A stream consists of several tracks, such as video or audio tracks. Each 
/// track is specified as an instance of MediaStreamTrack.
#[derive(Debug)]
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
    pub fn new(id: String) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            raw_id: to_c_str(&id)?,
            tracks: Vec::with_capacity(10),
            id,
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
