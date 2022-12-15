use super::RawMediaStreamTrack;
use tokio::sync::Mutex;
use std::{
    collections::HashMap,
    sync::Arc,
};

use crate::{
    frame::audio_frame::*,
    stream_ext::*,
    symbols::*,
    base::*,
};

/// The AudioTrack interface represents a single audio track from
/// a MediaStreamTrack.
pub struct AudioTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sinks: Mutex<HashMap<u8, Sinker<Arc<AudioFrame>>>>,
}

unsafe impl Send for AudioTrack {}
unsafe impl Sync for AudioTrack {}

impl AudioTrack {
    /// Register audio track frame sink, one track can register multiple sinks.
    /// The sink id cannot be repeated, otherwise the sink implementation will
    /// be overwritten.
    pub async fn register_sink(&self, id: u8, sink: Sinker<Arc<AudioFrame>>) {
        assert!(unsafe { &*self.raw }.remote);
        let mut sinks = self.sinks.lock().await;

        // Register for the first time, register the callback function to
        // webrtc native, and then do not need to register again.
        if sinks.is_empty() {
            unsafe {
                media_stream_audio_track_on_frame(
                    self.raw,
                    on_audio_frame,
                    self,
                )
            }
        }

        sinks.insert(id, sink);
    }

    /// Delete the registered sink, if it exists, it will return the deleted sink.
    pub async fn remove_sink(&self, id: u8) -> Option<Sinker<Arc<AudioFrame>>> {
        assert!(unsafe { &*self.raw }.remote);
        let mut sinks = self.sinks.lock().await;
        let value = sinks.remove(&id);
        if sinks.is_empty() {
            unsafe { media_stream_track_stop_on_frame(self.raw) }
        }

        value
    }

    /// create audio track from raw type ptr.
    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            sinks: Mutex::new(HashMap::new()),
            raw,
        })
    }

    fn on_data(this: &Self, frame: Arc<AudioFrame>) {
        if let Ok(mut sinks) = this.sinks.try_lock() {
            for sinker in sinks.values_mut() {
                sinker.sink.on_data(frame.clone());
            }
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

        unsafe { media_stream_track_stop_on_frame(self.raw) }
        unsafe { free_media_track(self.raw) }
    }
}

#[no_mangle]
extern "C" fn on_audio_frame(ctx: &AudioTrack, frame: *const RawAudioFrame) {
    assert!(!frame.is_null());
    let frame = AudioFrame::from_raw(frame);
    AudioTrack::on_data(ctx, frame);
}
