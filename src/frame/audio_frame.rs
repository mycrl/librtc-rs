use crate::symbols::*;
use libc::*;
use std::{
    slice::from_raw_parts,
    sync::Arc,
};

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

impl AsRef<[u8]> for AudioFrame {
    fn as_ref(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        unsafe { from_raw_parts(raw.buf, raw.len) }
    }
}

impl Drop for AudioFrame {
    fn drop(&mut self) {
        unsafe { rtc_free_audio_frame(self.raw) }
    }
}
