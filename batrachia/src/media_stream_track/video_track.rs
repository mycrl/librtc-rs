use super::RawMediaStreamTrack;
use tokio::sync::broadcast::*;
use crate::video_frame::*;
use std::sync::Arc;
use crate::base::*;
use libc::*;
use anyhow::{
    anyhow,
    Result,
};

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    fn create_media_stream_video_track(label: *const c_char) -> *const RawMediaStreamTrack;
    fn media_stream_video_track_add_frame(track: *const RawMediaStreamTrack, frame: *const RawI420Frame);
    fn media_stream_video_track_on_frame(
        track: *const RawMediaStreamTrack,
        handler: extern "C" fn(*const Sender<Arc<I420Frame>>, *const RawI420Frame),
        ctx: *const Sender<Arc<I420Frame>>,
    );
}

/// The VideoTrack interface represents a single video track from 
/// a MediaStreamTrack.
#[derive(Debug)]
pub struct VideoTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
}

unsafe impl Send for VideoTrack {}
unsafe impl Sync for VideoTrack {}

impl VideoTrack {
    /// Create a new video track, may fail to create, such as 
    // insufficient memory.
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
    pub fn add_frame(&self, frame: &I420Frame) {
        unsafe {
            media_stream_video_track_add_frame(self.raw, frame.get_raw());
        }
    }

    /// get video track frame sink.
    ///
    /// Only valid for remote video streams.
    pub fn get_sink(&self) -> VideoTrackSink {
        let (tx, receiver) = channel(1);
        let sender = Box::into_raw(Box::new(tx));
        unsafe {
            media_stream_video_track_on_frame(
                self.raw,
                on_video_frame_callback,
                sender,
            )
        }

        VideoTrackSink {
            receiver,
            sender,
        }
    }

    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            raw,
        })
    }
}

/// Used to receive the remote video stream, the video frame of the 
/// remote video track is pushed to the receiver through the channel.
pub struct VideoTrackSink {
    pub receiver: Receiver<Arc<I420Frame>>,
    sender: *mut Sender<Arc<I420Frame>>,
}

unsafe impl Send for VideoTrackSink {}
unsafe impl Sync for VideoTrackSink {}

impl Drop for VideoTrackSink {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.sender);
        }
    }
}

extern "C" fn on_video_frame_callback(
    ctx: *const Sender<Arc<I420Frame>>,
    frame: *const RawI420Frame,
) {
    if !frame.is_null() {
        let _ = unsafe { &*ctx }.send(I420Frame::from_raw(frame));
    }
}
