use std::{
    cell::UnsafeCell,
    sync::Arc,
};

use crate::{
    audio_frame::*,
    stream_ext::*,
    base::*,
};

use super::{
    RawMediaStreamTrack,
    free_media_track,
};

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    fn media_stream_audio_track_on_frame(
        track: *const RawMediaStreamTrack,
        handler: extern "C" fn(*mut Sinker<AudioFrame>, *const RawAudioFrame),
        ctx: *mut Sinker<AudioFrame>,
    );
}

/// The AudioTrack interface represents a single audio track from
/// a MediaStreamTrack.
#[derive(Debug)]
pub struct AudioTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sink: UnsafeCell<Option<*mut Sinker<AudioFrame>>>,
}

unsafe impl Send for AudioTrack {}
unsafe impl Sync for AudioTrack {}

impl AudioTrack {
    /// Used to receive the remote audio stream, the audio frames of the
    /// remote audio track is pushed to the receiver through the channel.
    pub fn register_sink(&self, sink: Sinker<AudioFrame>) {
        let sink = Box::into_raw(Box::new(sink));
        let raw_ptr = unsafe { &mut *self.sink.get() };
        let _ = raw_ptr.insert(sink);

        unsafe {
            media_stream_audio_track_on_frame(
                self.raw,
                on_audio_frame_callback,
                sink,
            )
        }
    }

    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            sink: UnsafeCell::new(None),
            raw,
        })
    }
}

impl Drop for AudioTrack {
    fn drop(&mut self) {
        let raw_ptr = self.raw;
        let raw = unsafe { &*raw_ptr };

        // If it is a track created locally, the label is allocated by rust
        // and needs to be freed by rust.
        if !raw.remote {
            free_cstring(raw.label);
        }

        if let Some(sink) = unsafe { *self.sink.get() } {
            let _ = unsafe { Box::from_raw(sink) };
        }

        unsafe { free_media_track(raw_ptr) }
    }
}

#[no_mangle]
extern "C" fn on_audio_frame_callback(
    ctx: *mut Sinker<AudioFrame>,
    frame: *const RawAudioFrame,
) {
    assert!(!ctx.is_null());
    assert!(!frame.is_null());
    unsafe { &mut *ctx }
        .sink
        .on_data(AudioFrame::from_raw(frame));
}
