use super::RawMediaStreamTrack;
use tokio::sync::Mutex;
use anyhow::{
    anyhow,
    Result,
};

use std::{
    collections::HashMap,
    sync::Arc,
};

use crate::{
    frame::video_frame::*,
    stream_ext::*,
    symbols::*,
    base::*,
};

/// The VideoTrack interface represents a single video track from
/// a MediaStreamTrack.
pub struct VideoTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sinks:          Mutex<HashMap<u8, Sinker<Arc<VideoFrame>>>>,
}

unsafe impl Send for VideoTrack {}
unsafe impl Sync for VideoTrack {}

impl VideoTrack {
    /// Create a new video track, may fail to create, such as
    /// insufficient memory.
    pub fn new(label: &str) -> Result<Arc<Self>> {
        let raw = unsafe {
            let c_label = to_c_str(label)?;
            let ptr = rtc_create_video_track(c_label);
            free_cstring(c_label);
            ptr
        };

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
            rtc_add_video_track_frame(self.raw, frame.get_raw());
        }
    }

    /// Register video track frame sink, one track can register multiple sinks.
    /// The sink id cannot be repeated, otherwise the sink implementation will
    /// be overwritten.
    pub async fn register_sink(&self, id: u8, sink: Sinker<Arc<VideoFrame>>) {
        assert!(unsafe { &*self.raw }.remote);
        let mut sinks = self.sinks.lock().await;

        // Register for the first time, register the callback function to
        // webrtc native, and then do not need to register again.
        if sinks.is_empty() {
            unsafe {
                rtc_set_video_track_frame_h(self.raw, on_video_frame, self)
            }
        }

        sinks.insert(id, sink);
    }

    /// Delete the registered sink, if it exists, it will return the deleted
    /// sink.
    pub async fn remove_sink(&self, id: u8) -> Option<Sinker<Arc<VideoFrame>>> {
        assert!(unsafe { &*self.raw }.remote);
        let mut sinks = self.sinks.lock().await;
        let value = sinks.remove(&id);
        if sinks.is_empty() {
            unsafe { rtc_remove_media_stream_track_frame_h(self.raw) }
        }

        value
    }

    /// create video track from raw type ptr.
    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            sinks: Mutex::new(HashMap::new()),
            raw,
        })
    }

    fn on_data(this: &Self, frame: Arc<VideoFrame>) {
        if let Ok(mut sinks) = this.sinks.try_lock() {
            for sinker in sinks.values_mut() {
                sinker.sink.on_data(frame.clone());
            }
        }
    }
}

impl Drop for VideoTrack {
    fn drop(&mut self) {
        unsafe { rtc_remove_media_stream_track_frame_h(self.raw) }
        unsafe { rtc_free_media_stream_track(self.raw) }
    }
}

#[no_mangle]
extern "C" fn on_video_frame(ctx: &VideoTrack, frame: *const RawVideoFrame) {
    assert!(!frame.is_null());
    let frame = VideoFrame::from_raw(frame);
    VideoTrack::on_data(ctx, frame);
}
