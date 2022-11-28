use std::sync::Arc;
use libc::*;

extern "C" {
    fn free_pcm_frames(frame: *const RawPCMFrames);
}

#[repr(C)]
#[derive(Debug)]
pub struct RawPCMFrames {
    buf: *const u8,
    bits_per_sample: c_int,
    sample_rate: c_int,
    channels: c_int,
    frames: c_int,
}

/// A list of audio frames in pcm format, usually 10ms long.
pub struct PCMFrames {
    raw_ptr: *const RawPCMFrames,
}

unsafe impl Send for PCMFrames {}
unsafe impl Sync for PCMFrames {}

impl PCMFrames {
    pub(crate) fn from_raw(raw_ptr: *const RawPCMFrames) -> Arc<Self> {
        assert!(!raw_ptr.is_null());
        Arc::new(Self {
            raw_ptr,
        })
    }
}

impl Drop for PCMFrames {
    fn drop(&mut self) {
        unsafe { free_pcm_frames(self.raw_ptr) }
    }
}
