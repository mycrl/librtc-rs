use crate::audio_frame::*;
use super::RawMediaStreamTrack;
use tokio::sync::broadcast::*;
use std::sync::Arc;

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    fn media_stream_audio_track_on_frame(
        track: *const RawMediaStreamTrack,
        handler: extern "C" fn(*const Sender<Arc<PCMFrames>>, *const RawPCMFrames),
        ctx: *const Sender<Arc<PCMFrames>>,
    );
}

/// The AudioTrack interface represents a single audio track from 
/// a MediaStreamTrack.
#[derive(Debug)]
pub struct AudioTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
}

unsafe impl Send for AudioTrack {}
unsafe impl Sync for AudioTrack {}

impl AudioTrack {
    /// get audio track frame sink.
    ///
    /// Only valid for remote audio streams.
    pub fn get_sink(&self) -> AudioTrackSink {
        let (tx, receiver) = channel(1);
        let sender = Box::into_raw(Box::new(tx));
        unsafe {
            media_stream_audio_track_on_frame(
                self.raw,
                on_audio_frame_callback,
                sender,
            )
        }

        AudioTrackSink {
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

/// Used to receive the remote audio stream, the audio frames of the 
/// remote audio track is pushed to the receiver through the channel.
pub struct AudioTrackSink {
    pub receiver: Receiver<Arc<PCMFrames>>,
    sender: *mut Sender<Arc<PCMFrames>>,
}

unsafe impl Send for AudioTrackSink {}
unsafe impl Sync for AudioTrackSink {}

impl Drop for AudioTrackSink {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.sender);
        }
    }
}

extern "C" fn on_audio_frame_callback(
    ctx: *const Sender<Arc<PCMFrames>>,
    frame: *const RawPCMFrames,
) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        if let Some(frame) = unsafe { frame.as_ref() } {
            ctx.send(PCMFrames::from_raw(frame)).unwrap();
        }
    }
}
