use crate::symbols::*;
use libc::*;
use std::{
    slice::from_raw_parts,
    sync::Arc,
};

#[repr(C)]
#[derive(Debug)]
pub struct RawAudioFrame {
    remote: bool,
    size: size_t,
    frames: size_t,
    channels: size_t,
    sample_rate: c_int,
    timestamp: i64,
    buf: *const i16,
}

/// A list of audio frames in pcm format, usually 10ms long.
#[derive(Debug)]
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

    pub(crate) fn get_raw(&self) -> *const RawAudioFrame {
        self.raw
    }

    pub fn new(
        sample_rate: usize,
        channels: u8,
        frames: usize,
        timestamp: usize,
        buf: &[u8],
    ) -> Self {
        let buf: &[i16] = unsafe { std::mem::transmute(buf) };
        Self {
            raw: Box::into_raw(Box::new(RawAudioFrame {
                sample_rate: sample_rate as c_int,
                channels: channels as size_t,
                timestamp: timestamp as i64,
                buf: buf.as_ptr(),
                size: buf.len(),
                remote: false,
                frames,
            })),
        }
    }
}

impl AsRef<[i16]> for AudioFrame {
    fn as_ref(&self) -> &[i16] {
        let raw = unsafe { &*self.raw };
        unsafe { from_raw_parts(raw.buf, raw.size) }
    }
}

impl Drop for AudioFrame {
    fn drop(&mut self) {
        let raw = unsafe { &*self.raw };
        unsafe {
            // If remote is false, it means the distribution is
            // on the rust box.
            if raw.remote {
                rtc_free_frame(self.raw as *const c_void)
            } else {
                let _ = Box::from_raw(self.raw.cast_mut());
            };
        }
    }
}
