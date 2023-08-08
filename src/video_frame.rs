use std::{ffi::c_void, slice::from_raw_parts, sync::Arc};

use crate::media_stream_track::rtc_free_frame;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct RawVideoFrame {
    remote: bool,
    width: u32,
    height: u32,
    timestamp: i64,
    planes: [*const u8; 4],
    strides: [u32; 4],
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
    pub(crate) fn get_raw(&self) -> *const RawVideoFrame {
        self.raw
    }

    /// create video frame from raw video frame type.
    pub(crate) fn from_raw(raw: *const RawVideoFrame) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self { raw })
    }

    /// Create i420 frame structure from memory buffer.
    ///
    /// The created frame is memory-safe and thread-safe, and can be
    /// transferred and copied in threads.
    pub fn new(
        width: u32,
        height: u32,
        timestamp: usize,
        planes: [&[u8]; 4],
        strides: [u32; 4]
    ) -> Self {
        Self {
            raw: Box::into_raw(Box::new(RawVideoFrame {
                planes: planes.map(|item| item.as_ptr()),
                timestamp: timestamp as i64,
                remote: false,
                strides,
                width,
                height,
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
        let size = (raw.strides[0] * raw.height) as usize;
        unsafe { from_raw_parts(raw.planes[0], size) }
    }

    /// get i420 frame y stride
    pub fn stride_y(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.strides[0] as usize
    }

    /// get i420 frame u buffer
    pub fn data_u(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let size = (raw.strides[1] * (raw.height / 2)) as usize;
        unsafe { from_raw_parts(raw.planes[1], size) }
    }

    /// get i420 frame u stride
    pub fn stride_u(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.strides[1] as usize
    }

    /// get i420 frame v buffer
    pub fn data_v(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let size = (raw.strides[2] * (raw.height / 2)) as usize;
        unsafe { from_raw_parts(raw.planes[2], size) }
    }

    /// get i420 frame v stride
    pub fn stride_v(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.strides[2] as usize
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
