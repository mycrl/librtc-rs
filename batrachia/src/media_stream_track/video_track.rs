use super::RawMediaStreamTrack;
use std::sync::Arc;
use anyhow::{
    anyhow,
    Result,
};

use crate::{
    video_frame::*,
    stream_ext::*,
    symbols::*,
    base::*,
};

/// The VideoTrack interface represents a single video track from
/// a MediaStreamTrack.
pub struct VideoTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sinks: UnsafeVec<Sinker<Arc<VideoFrame>>>,
}

unsafe impl Send for VideoTrack {}
unsafe impl Sync for VideoTrack {}

impl VideoTrack {
    /// Create a new video track, may fail to create, such as
    /// insufficient memory.
    pub fn new(label: &str) -> Result<Arc<Self>> {
        let raw = unsafe { create_media_stream_video_track(to_c_str(label)?) };
        if raw.is_null() {
            Err(anyhow!("create media stream track failed!"))
        } else {
            Ok(Self::from_raw(raw))
        }
    }

    /// Push video frames to the current track, currently only
    /// supports pushing video frames in i420 format.
    ///
    /// Only valid for local video streams.
    pub fn add_frame(&self, frame: &VideoFrame) {
        assert!(!unsafe { &*self.raw }.remote);
        unsafe {
            media_stream_video_track_add_frame(
                self.raw, 
                frame.get_raw()
            );
        }
    }

    /// Register video track frame sink, one track can register multiple sinks.
    /// The sink id cannot be repeated, otherwise the sink implementation will
    /// be overwritten.
    pub fn register_sink(&self, sink: Sinker<Arc<VideoFrame>>) -> usize {
        assert!(unsafe { &*self.raw }.remote);
        // Register for the first time, register the callback function to
        // webrtc native, and then do not need to register again.
        if self.sinks.is_empty() {
            unsafe {
                media_stream_video_track_on_frame(
                    self.raw,
                    on_video_frame,
                    self,
                )
            }
        }

        self.sinks.push(sink)
    }

    /// Delete the registered sink, if it exists, it will return the deleted
    /// sink.
    pub fn remove_sink(&self, id: usize) -> Sinker<Arc<VideoFrame>> {
        assert!(unsafe { &*self.raw }.remote);
        self.sinks.remove(id)
    }

    /// create video track from raw type ptr.
    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            sinks: UnsafeVec::with_capacity(5),
            raw,
        })
    }

    fn on_data(self: &Self, frame: Arc<VideoFrame>) {
        for sinker in self.sinks.get_mut_slice() {
            sinker.sink.on_data(frame.clone());
        }
    }
}

impl Drop for VideoTrack {
    fn drop(&mut self) {
        // If it is a track created locally, the label is allocated by rust
        // and needs to be freed by rust.
        if !unsafe { &*self.raw }.remote {
            free_cstring(unsafe { &*self.raw }.label);
        }

        unsafe { free_media_track(self.raw) }
    }
}

#[no_mangle]
extern "C" fn on_video_frame(ctx: &VideoTrack, frame: *const RawVideoFrame) {
    assert!(!frame.is_null());
    let frame = VideoFrame::from_raw(frame);
    VideoTrack::on_data(ctx, frame);
}