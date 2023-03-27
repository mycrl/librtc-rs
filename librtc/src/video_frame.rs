use crate::symbols::*;
use libc::*;
use std::{
    slice::from_raw_parts,
    sync::Arc,
};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct RawVideoFrame {
    remote: bool,
    width: u32,
    height: u32,
    timestamp: i64,
    data_y: *const u8,
    stride_y: u32,
    data_u: *const u8,
    stride_u: u32,
    data_v: *const u8,
    stride_v: u32,
}

/// VideoFrame represents the frame of the video,
/// and the format is i420 (yu12).
///
/// Also known as Planar YUV 4:2:0, this format is composed of
/// three distinct planes, one plane of luma and two planes of
/// chroma, denoted Y, U and V, and present in this order.
/// The U an V planes are sub-sampled horizontally and vertically
/// by a factor of 2 compared to the Y plane. Each sample in this
/// format is 8 bits.
#[derive(Debug)]
pub struct VideoFrame {
    raw: *const RawVideoFrame,
}

unsafe impl Send for VideoFrame {}
unsafe impl Sync for VideoFrame {}

impl VideoFrame {
    pub fn from_default_layout(
        width: u32,
        height: u32,
        timestamp: usize,
        buf: &[u8],
    ) -> Self {
        assert!(buf.len() >= (width as f64 * height as f64 * 1.5) as usize);
        let size_u = ((width / 2) * (height / 2)) as usize;
        let size_y = (width * height) as usize;
        Self::new(
            width,
            height,
            timestamp,
            &buf[..size_y],
            width as usize,
            &buf[size_y..size_y + size_u],
            (width / 2) as usize,
            &buf[size_y + size_u..],
            (width / 2) as usize,
        )
    }

    pub(crate) fn get_raw(&self) -> *const RawVideoFrame {
        self.raw
    }

    /// create video frame from raw video frame type.
    pub(crate) fn from_raw(raw: *const RawVideoFrame) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            raw,
        })
    }

    /// Create i420 frame structure from memory buffer.
    ///
    /// The created frame is memory-safe and thread-safe, and can be
    /// transferred and copied in threads.
    pub fn new(
        width: u32,
        height: u32,
        timestamp: usize,
        data_y: &[u8],
        stride_y: usize,
        data_u: &[u8],
        stride_u: usize,
        data_v: &[u8],
        stride_v: usize,
    ) -> Self {
        Self {
            raw: Box::into_raw(Box::new(RawVideoFrame {
                remote: false,
                width,
                height,
                timestamp: timestamp as i64,
                data_y: data_y.as_ptr(),
                stride_y: stride_y as u32,
                data_u: data_u.as_ptr(),
                stride_u: stride_u as u32,
                data_v: data_v.as_ptr(),
                stride_v: stride_v as u32,
            })),
        }
    }

    /// get video frame width
    pub fn width(&self) -> u32 {
        unsafe { &*self.raw }.width
    }

    /// get video frame height
    pub fn height(&self) -> u32 {
        unsafe { &*self.raw }.height
    }

    /// get i420 frame y buffer
    pub fn data_y(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let size = (raw.stride_y * raw.height) as usize;
        unsafe { from_raw_parts(raw.data_y, size) }
    }

    /// get i420 frame y stride
    pub fn stride_y(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.stride_y as usize
    }

    /// get i420 frame u buffer
    pub fn data_u(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let size = (raw.stride_u * (raw.height / 2)) as usize;
        unsafe { from_raw_parts(raw.data_u, size) }
    }

    /// get i420 frame u stride
    pub fn stride_u(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.stride_u as usize
    }

    /// get i420 frame v buffer
    pub fn data_v(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let size = (raw.stride_v * (raw.height / 2)) as usize;
        unsafe { from_raw_parts(raw.data_v, size) }
    }

    /// get i420 frame v stride
    pub fn stride_v(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.stride_v as usize
    }
}

impl Drop for VideoFrame {
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
