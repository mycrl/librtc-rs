use std::sync::Arc;

use super::base::*;
use anyhow::{anyhow, Result};
use libc::*;
use tokio::sync::broadcast::*;

#[link(name = "batrachiatc")]
extern "C" {
    fn media_stream_video_track_add_frame(
        track: *const RawMediaStreamTrack,
        frame: *const I420Frame,
    );

    #[allow(improper_ctypes)]
    fn media_stream_video_track_on_frame(
        track: *const RawMediaStreamTrack,
        ctx: *const Sender<&I420Frame>,
        handler: extern "C" fn(*const Sender<&I420Frame>, *const I420Frame),
    );

    fn create_media_stream_video_track(
        id: *const c_char,
        label: *const c_char,
    ) -> *const RawMediaStreamTrack;

    fn free_i420_frame(frame: *const I420Frame);
    fn free_media_track(track: *const RawMediaStreamTrack);
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct I420Frame {
    width: u32,
    height: u32,

    data_y: *const u8,
    stride_y: u32,
    data_u: *const u8,
    stride_u: u32,
    data_v: *const u8,
    stride_v: u32,

    remote: bool,
}

unsafe impl Send for I420Frame {}
unsafe impl Sync for I420Frame {}

impl I420Frame {
    pub fn new(width: usize, height: usize, buf: &[u8]) -> Self {
        let need_size = ((width * height) as f32 * 1.5) as usize;
        assert!(buf.len() >= need_size);

        let y_size = width * height;
        let u_size = width * height / 4;

        let data_y = buf[..y_size].as_ptr();
        let data_u = buf[y_size..y_size + u_size].as_ptr();
        let data_v = buf[y_size + u_size..].as_ptr();

        Self {
            width: width as u32,
            height: height as u32,
            data_y,
            stride_y: width as u32,
            data_u,
            stride_u: (width / 2) as u32,
            data_v,
            stride_v: (width / 2) as u32,
            remote: false,
        }
    }
}

impl Drop for I420Frame {
    fn drop(&mut self) {
        if self.remote {
            unsafe { free_i420_frame(self) }
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaStreamTrackKind {
    Video,
    // Audio,
}

#[repr(C)]
pub struct RawMediaStreamTrack {
    /// Returns a string set to "audio" if the track is an audio track and to
    /// "video", if it is a video track. It doesn't change if the track is
    /// disassociated from its source.
    pub kind: MediaStreamTrackKind,
    /// Returns a string containing a user agent-assigned label that identifies the
    /// track source, as in "internal microphone". The string may be left empty and
    /// is empty as long as no source has been connected. When the track is
    /// disassociated from its source, the label is not changed.
    pub label: *const c_char,
    /// Returns a Boolean with a value of true if the track is sourced by a
    /// RTCPeerConnection, false otherwise.
    pub remote: bool,

    // video
    video_source: *const c_void,
    video_sink: *const c_void,
}

/*
MediaStreamTrack

The MediaStreamTrack interface represents a single media track within a stream;
typically, these are audio or video tracks, but other track types may exist as
well.
*/
#[derive(Debug)]
pub struct MediaStreamTrack {
    raw: *const RawMediaStreamTrack,
}

unsafe impl Send for MediaStreamTrack {}
unsafe impl Sync for MediaStreamTrack {}

extern "C" fn on_frame_callback(ctx: *const Sender<&I420Frame>, frame: *const I420Frame) {
    unsafe { &*ctx }.send(unsafe { &*frame }).unwrap();
}

impl MediaStreamTrack {
    pub fn from_raw(raw: *const RawMediaStreamTrack) -> Result<Arc<Self>> {
        if raw.is_null() {
            Err(anyhow!("create media stream track failed!"))
        } else {
            Ok(Arc::new(Self { raw }))
        }
    }

    pub fn new(id: &str, label: &str, kind: MediaStreamTrackKind) -> Result<Arc<Self>> {
        let raw = match kind {
            MediaStreamTrackKind::Video => unsafe {
                create_media_stream_video_track(to_c_str(id)?, to_c_str(label)?)
            },
        };

        if raw.is_null() {
            Err(anyhow!("create media stream track failed!"))
        } else {
            Ok(Arc::new(Self { raw }))
        }
    }

    pub fn add_frame(&self, frame: &I420Frame) {
        unsafe {
            media_stream_video_track_add_frame(self.raw, frame);
        }
    }

    pub fn on_frame<'a>(&'a self) -> MediaStreamTrackSink<'a> {
        let (tx, receiver) = channel(1);
        let sender = Box::into_raw(Box::new(tx));
        unsafe { media_stream_video_track_on_frame(self.raw, sender, on_frame_callback) }

        MediaStreamTrackSink { 
            receiver, 
            sender 
        }
    }

    pub fn get_raw(&self) -> *const RawMediaStreamTrack {
        self.raw
    }
}

impl Drop for MediaStreamTrack {
    fn drop(&mut self) {
        let raw = unsafe { &*self.raw };
        // if !raw.remote {
        //     free_cstring(raw.label);
        // }

        // unsafe { free_media_track(self.raw) }
    }
}

pub struct MediaStreamTrackSink<'a> {
    pub receiver: Receiver<&'a I420Frame>,
    sender: *mut Sender<&'a I420Frame>,
}

unsafe impl Send for MediaStreamTrackSink<'_> {}
unsafe impl Sync for MediaStreamTrackSink<'_> {}

impl Drop for MediaStreamTrackSink<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.sender);
        }
    }
}
