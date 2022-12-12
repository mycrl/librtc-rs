use crate::symbols::*;
use std::sync::Arc;
use libc::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawAudioFrame {
    buf: *const u8,
    len: usize,
    
    bits_per_sample: c_int,
    sample_rate: c_int,
    channels: c_int,
    frames: c_int,
    remote: bool,
}

/// A list of audio frames in pcm format, usually 10ms long.
pub struct AudioFrame {
    raw: *const RawAudioFrame,
}

unsafe impl Send for AudioFrame {}
unsafe impl Sync for AudioFrame {}

impl AudioFrame {
    /// crate AudiFrame from raw type.
    pub(crate) fn from_raw(raw: *const RawAudioFrame) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            raw,
        })
    }
}

impl Drop for AudioFrame {
    fn drop(&mut self) {
        unsafe { free_audio_frame(self.raw) }
    }
}
