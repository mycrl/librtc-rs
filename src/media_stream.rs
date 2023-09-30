use std::{error::Error, ffi::c_char, fmt, sync::Arc};

use crate::{
    cstr::{free_cstring, to_c_str, StringError},
    MediaStreamTrack,
};

#[derive(Debug)]
pub enum MediaStreamError {
    CreateTrackFailed,
    StringError(StringError),
}

impl Error for MediaStreamError {}

impl fmt::Display for MediaStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

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
    pub fn new(id: &str) -> Result<Arc<Self>, MediaStreamError> {
        Ok(Arc::new(Self {
            tracks: Vec::with_capacity(10),
            raw_id: to_c_str(id).map_err(|e| MediaStreamError::StringError(e))?,
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
