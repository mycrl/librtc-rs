use super::base::*;
use super::video_frame::*;
use anyhow::{
    anyhow,
    Result,
};
use libc::*;
use std::sync::Arc;
use tokio::sync::broadcast::*;

extern "C" {
    fn media_stream_video_track_add_frame(
        track: *const RawMediaStreamTrack,
        frame: *const RawI420Frame,
    );

    #[allow(improper_ctypes)]
    fn media_stream_video_track_on_frame(
        track: *const RawMediaStreamTrack,
        handler: extern "C" fn(
            *const Sender<Arc<I420Frame>>,
            *const RawI420Frame,
        ),
        ctx: *const Sender<Arc<I420Frame>>,
    );

    fn create_media_stream_video_track(
        label: *const c_char,
    ) -> *const RawMediaStreamTrack;
    fn free_media_track(track: *const RawMediaStreamTrack);
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaStreamTrackKind {
    Video,
    // Audio,
}

#[repr(C)]
pub(crate) struct RawMediaStreamTrack {
    /// Returns a string set to "audio" if the track is an audio track and to
    /// "video", if it is a video track. It doesn't change if the track is
    /// disassociated from its source.
    pub kind: MediaStreamTrackKind,
    /// Returns a string containing a user agent-assigned label that identifies
    /// the track source, as in "internal microphone". The string may be
    /// left empty and is empty as long as no source has been connected.
    /// When the track is disassociated from its source, the label is not
    /// changed.
    pub label: *const c_char,
    /// Returns a Boolean with a value of true if the track is sourced by a
    /// RTCPeerConnection, false otherwise.
    pub remote: bool,

    // video
    video_source: *const c_void,
    video_sink: *const c_void,
}

/// The MediaStreamTrack interface represents a single media track within
/// a stream typically.
///
/// these are audio or video tracks, but other track types may exist as
/// well.
#[derive(Debug)]
pub struct MediaStreamTrack {
    raw: *const RawMediaStreamTrack,
}

unsafe impl Send for MediaStreamTrack {}
unsafe impl Sync for MediaStreamTrack {}

impl MediaStreamTrack {
    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            raw,
        })
    }

    pub fn new(label: &str, kind: MediaStreamTrackKind) -> Result<Arc<Self>> {
        let raw = match kind {
            MediaStreamTrackKind::Video => unsafe {
                create_media_stream_video_track(to_c_str(label)?)
            },
        };

        if raw.is_null() {
            Err(anyhow!("create media stream track failed!"))
        } else {
            Ok(Arc::new(Self {
                raw,
            }))
        }
    }

    pub fn add_frame(&self, frame: &I420Frame) {
        unsafe {
            media_stream_video_track_add_frame(self.raw, frame.get_raw());
        }
    }

    pub fn get_sink(&self) -> MediaStreamTrackSink {
        let (tx, receiver) = channel(1);
        let sender = Box::into_raw(Box::new(tx));
        unsafe {
            media_stream_video_track_on_frame(
                self.raw,
                on_frame_callback,
                sender,
            )
        }

        MediaStreamTrackSink {
            receiver,
            sender,
        }
    }

    pub(crate) fn get_raw(&self) -> *const RawMediaStreamTrack {
        self.raw
    }
}

impl Drop for MediaStreamTrack {
    fn drop(&mut self) {
        let raw = unsafe { &*self.raw };
        if !raw.remote {
            free_cstring(raw.label);
        }

        unsafe { free_media_track(self.raw) }
    }
}

pub struct MediaStreamTrackSink {
    pub receiver: Receiver<Arc<I420Frame>>,
    sender: *mut Sender<Arc<I420Frame>>,
}

unsafe impl Send for MediaStreamTrackSink {}
unsafe impl Sync for MediaStreamTrackSink {}

impl Drop for MediaStreamTrackSink {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.sender);
        }
    }
}

extern "C" fn on_frame_callback(
    ctx: *const Sender<Arc<I420Frame>>,
    frame: *const RawI420Frame,
) {
    if !frame.is_null() {
        let _ = unsafe { &*ctx }.send(I420Frame::from_raw(frame));
    }
}
