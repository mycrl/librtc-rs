use super::RawMediaStreamTrack;
use std::sync::Arc;
use crate::{
    audio_frame::*,
    stream_ext::*,
    symbols::*,
    base::*,
};

/// The AudioTrack interface represents a single audio track from
/// a MediaStreamTrack.
pub struct AudioTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sinks: UnsafeVec<Sinker<Arc<AudioFrame>>>,
}

unsafe impl Send for AudioTrack {}
unsafe impl Sync for AudioTrack {}

impl AudioTrack {
    /// Register audio track frame sink, one track can register multiple sinks.
    /// The sink id cannot be repeated, otherwise the sink implementation will
    /// be overwritten.
    pub fn register_sink(&self, sink: Sinker<Arc<AudioFrame>>) -> usize {
        assert!(unsafe { &*self.raw }.remote);
        // Register for the first time, register the callback function to
        // webrtc native, and then do not need to register again.
        if self.sinks.is_empty() {
            unsafe {
                media_stream_audio_track_on_frame(
                    self.raw,
                    on_audio_frame,
                    self,
                )
            }
        }

        self.sinks.push(sink)
    }

    /// Delete the registered sink, if it exists, it will return the deleted
    /// sink.
    pub fn remove_sink(&self, id: usize) -> Sinker<Arc<AudioFrame>> {
        assert!(unsafe { &*self.raw }.remote);
        self.sinks.remove(id)
    }

    /// create audio track from raw type ptr.
    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            sinks: UnsafeVec::with_capacity(5),
            raw,
        })
    }

    fn on_data(self: &Self, frame: Arc<AudioFrame>) {
        for sinker in self.sinks.get_mut_slice() {
            sinker.sink.on_data(frame.clone());
        }
    }
}

impl Drop for AudioTrack {
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
extern "C" fn on_audio_frame(ctx: &AudioTrack, frame: *const RawAudioFrame) {
    assert!(!frame.is_null());
    let frame = AudioFrame::from_raw(frame);
    AudioTrack::on_data(ctx, frame);
}
