use std::{collections::HashMap, ffi::c_char, sync::Arc};

use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

use crate::{
    audio_frame::RawAudioFrame,
    cstr::{free_cstring, to_c_str},
    media_stream_track::{
        rtc_free_media_stream_track, rtc_remove_media_stream_track_frame_h, RawMediaStreamTrack,
    },
    AudioFrame, Sinker,
};

#[allow(improper_ctypes)]
extern "C" {
    pub(crate) fn rtc_create_audio_track(
        label: *const c_char,
    ) -> *const crate::media_stream_track::RawMediaStreamTrack;

    pub(crate) fn rtc_add_audio_track_frame(
        track: *const crate::media_stream_track::RawMediaStreamTrack,
        frame: *const crate::audio_frame::RawAudioFrame,
    );

    pub(crate) fn rtc_set_audio_track_frame_h(
        track: *const crate::media_stream_track::RawMediaStreamTrack,
        handler: extern "C" fn(
            &crate::audio_track::AudioTrack,
            *const crate::audio_frame::RawAudioFrame,
        ),
        ctx: &crate::audio_track::AudioTrack,
    );
}

/// The AudioTrack interface represents a single audio track from
/// a MediaStreamTrack.
pub struct AudioTrack {
    pub(crate) raw: *const RawMediaStreamTrack,
    sinks: Mutex<HashMap<u8, Sinker<Arc<AudioFrame>>>>,
}

unsafe impl Send for AudioTrack {}
unsafe impl Sync for AudioTrack {}

impl AudioTrack {
    /// Create a new audio track, may fail to create, such as
    /// insufficient memory.
    pub fn new(label: &str) -> Result<Arc<Self>> {
        let raw = unsafe {
            let c_label = to_c_str(label)?;
            let ptr = rtc_create_audio_track(c_label);
            free_cstring(c_label);
            ptr
        };

        if raw.is_null() {
            Err(anyhow!("create audio track failed!"))
        } else {
            Ok(Self::from_raw(raw))
        }
    }

    /// Push audio frames to the current track, currently only
    /// supports pushing audio frames in pcm format.
    ///
    /// Only valid for local audio streams.
    pub fn add_frame(&self, frame: &AudioFrame) {
        unsafe {
            rtc_add_audio_track_frame(self.raw, frame.get_raw());
        }
    }

    /// Register audio track frame sink, one track can register multiple sinks.
    /// The sink id cannot be repeated, otherwise the sink implementation will
    /// be overwritten.
    pub async fn register_sink(&self, id: u8, sink: Sinker<Arc<AudioFrame>>) {
        let mut sinks = self.sinks.lock().await;

        // Register for the first time, register the callback function to
        // webrtc native, and then do not need to register again.
        if sinks.is_empty() {
            unsafe { rtc_set_audio_track_frame_h(self.raw, on_audio_frame, self) }
        }

        sinks.insert(id, sink);
    }

    /// Delete the registered sink, if it exists, it will return the deleted
    /// sink.
    pub async fn remove_sink(&self, id: u8) -> Option<Sinker<Arc<AudioFrame>>> {
        let mut sinks = self.sinks.lock().await;
        let value = sinks.remove(&id);
        if sinks.is_empty() {
            unsafe { rtc_remove_media_stream_track_frame_h(self.raw) }
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
        unsafe { rtc_remove_media_stream_track_frame_h(self.raw) }
        unsafe { rtc_free_media_stream_track(self.raw) }
    }
}

#[no_mangle]
extern "C" fn on_audio_frame(ctx: &AudioTrack, frame: *const RawAudioFrame) {
    assert!(!frame.is_null());
    let frame = AudioFrame::from_raw(frame);
    AudioTrack::on_data(ctx, frame);
}
