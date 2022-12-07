use crate::symbols::*;
use std::sync::Arc;
use libc::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawAudioFrame {
    buf: *const u8,
    bits_per_sample: c_int,
    sample_rate: c_int,
    channels: c_int,
    frames: c_int,
}

/// A list of audio frames in pcm format, usually 10ms long.
pub struct AudioFrame {
    raw_ptr: *const RawAudioFrame,
}

unsafe impl Send for AudioFrame {}
unsafe impl Sync for AudioFrame {}

impl AudioFrame {
    /// crate AudiFrame from raw type.
    pub(crate) fn from_raw(raw_ptr: *const RawAudioFrame) -> Arc<Self> {
        assert!(!raw_ptr.is_null());
        Arc::new(Self {
            raw_ptr,
        })
    }
}

impl Drop for AudioFrame {
    fn drop(&mut self) {
        unsafe { free_pcm_frames(self.raw_ptr) }
    }
}
