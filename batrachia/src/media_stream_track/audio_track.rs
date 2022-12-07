use tokio::sync::RwLock;
use std::{
    collections::HashMap,
    sync::Arc,
};

use crate::{
    audio_frame::*,
    stream_ext::*,
    base::*,
    RUNTIME,
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
        handler: extern "C" fn(&AudioTrack, *const RawAudioFrame),
        ctx: &AudioTrack,
    );
}

/// The AudioTrack interface represents a single audio track from
/// a MediaStreamTrack.
pub struct AudioTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sinks: Arc<RwLock<HashMap<u8, Sinker<Arc<AudioFrame>>>>>,
}

unsafe impl Send for AudioTrack {}
unsafe impl Sync for AudioTrack {}

impl AudioTrack {
    /// Used to receive the remote audio stream, the audio frames of the
    /// remote audio track is pushed to the receiver through the channel.
    pub async fn register_sink(&self, id: u8, sink: Sinker<Arc<AudioFrame>>) {
        let mut sinks = self.sinks.write().await;
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

    pub async fn remove_sink(&self, id: u8) -> Option<Sinker<Arc<AudioFrame>>> {
        self.sinks.write().await.remove(&id)
    }

    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            sinks: Arc::new(RwLock::new(HashMap::new())),
            raw,
        })
    }

    fn on_data(self: &Self, frame: Arc<AudioFrame>) {
        let sinks = self.sinks.clone();
        RUNTIME.spawn(async move {
            for sinker in sinks.read().await.values() {
                sinker.sink.on_data(frame.clone());
            }
        });
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

        unsafe { free_media_track(raw_ptr) }
    }
}

#[no_mangle]
extern "C" fn on_audio_frame(ctx: &AudioTrack, frame: *const RawAudioFrame) {
    assert!(!frame.is_null());
    let frame = AudioFrame::from_raw(frame);
    AudioTrack::on_data(ctx, frame);
}
