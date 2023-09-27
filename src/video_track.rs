use std::{
    collections::HashMap,
    ffi::c_char,
    sync::{Arc, RwLock},
};

use crate::{
    cstr::{c_str_to_str, free_cstring, to_c_str},
    media_stream::MediaStreamError,
    media_stream_track::{
        rtc_free_media_stream_track, rtc_remove_media_stream_track_frame_h, RawMediaStreamTrack,
    },
    video_frame::RawVideoFrame,
    Sinker, VideoFrame,
};

#[allow(improper_ctypes)]
extern "C" {
    pub(crate) fn rtc_create_video_track(
        label: *const c_char,
    ) -> *const crate::media_stream_track::RawMediaStreamTrack;

    pub(crate) fn rtc_add_video_track_frame(
        track: *const crate::media_stream_track::RawMediaStreamTrack,
        frame: *const crate::video_frame::RawVideoFrame,
    );

    pub(crate) fn rtc_set_video_track_frame_h(
        track: *const crate::media_stream_track::RawMediaStreamTrack,
        handler: extern "C" fn(
            &crate::video_track::VideoTrack,
            *const crate::video_frame::RawVideoFrame,
        ),
        ctx: &crate::video_track::VideoTrack,
    );
}

/// The VideoTrack interface represents a single video track from
/// a MediaStreamTrack.
pub struct VideoTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sinks: RwLock<HashMap<u8, Sinker<Arc<VideoFrame>>>>,
}

unsafe impl Send for VideoTrack {}
unsafe impl Sync for VideoTrack {}

impl VideoTrack {
    pub fn label(&self) -> &str {
        c_str_to_str(unsafe { (*self.raw).label }).expect("get video track label string to failed")
    }

    /// Create a new video track, may fail to create, such as
    /// insufficient memory.
    pub fn new(label: &str) -> Result<Arc<Self>, MediaStreamError> {
        let raw = unsafe {
            let c_label = to_c_str(label).map_err(|e| MediaStreamError::StringError(e))?;
            let ptr = rtc_create_video_track(c_label);
            free_cstring(c_label);
            ptr
        };

        if raw.is_null() {
            Err(MediaStreamError::CreateTrackFailed)
        } else {
            Ok(Self::from_raw(raw))
        }
    }

    /// Push video frames to the current track, currently only
    /// supports pushing video frames in i420 format.
    ///
    /// Only valid for local video streams.
    pub fn add_frame(&self, frame: &VideoFrame) {
        unsafe {
            rtc_add_video_track_frame(self.raw, frame.get_raw());
        }
    }

    /// Register video track frame sink, one track can register multiple sinks.
    /// The sink id cannot be repeated, otherwise the sink implementation will
    /// be overwritten.
    pub fn register_sink(&self, id: u8, sink: Sinker<Arc<VideoFrame>>) {
        let mut sinks = self.sinks.write().unwrap();

        // Register for the first time, register the callback function to
        // webrtc native, and then do not need to register again.
        if sinks.is_empty() {
            unsafe { rtc_set_video_track_frame_h(self.raw, on_video_frame, self) }
        }

        sinks.insert(id, sink);
    }

    /// Delete the registered sink, if it exists, it will return the deleted
    /// sink.
    pub fn remove_sink(&self, id: u8) -> Option<Sinker<Arc<VideoFrame>>> {
        let mut sinks = self.sinks.write().unwrap();
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
            sinks: RwLock::new(HashMap::new()),
            raw,
        })
    }

    fn on_data(this: &Self, frame: Arc<VideoFrame>) {
        for sinker in this.sinks.read().unwrap().values() {
            sinker.sink.on_data(frame.clone());
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
