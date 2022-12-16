use crate::symbols::*;
use std::{
    slice::from_raw_parts,
    sync::Arc,
};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct RawVideoFrame {
    buf: *const u8,
    len: usize,

    width: u32,
    height: u32,
    stride_y: u32,
    stride_u: u32,
    stride_v: u32,
    remote: bool,
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
/// 
/// ```
/// ----> width
/// | Y0 | Y1 | Y2 | Y3
/// | U0 | U1 |
/// | V0 | V0 |
/// ```
///
/// * y planar: width * height
/// * y stride: width (Does not calculate memory alignment)
/// * uv planar: (width / 2) * (height / 2)
/// * uv stride: width / 2 (Does not calculate memory alignment)
#[derive(Debug)]
pub struct VideoFrame {
    raw: *const RawVideoFrame,
}

unsafe impl Send for VideoFrame {}
unsafe impl Sync for VideoFrame {}

impl VideoFrame {
    /// create video frame from raw video frame type.
    pub(crate) fn from_raw(raw: *const RawVideoFrame) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            raw,
        })
    }

    pub(crate) fn get_raw(&self) -> *const RawVideoFrame {
        self.raw
    }

    /// Create i420 frame structure from memory buffer.
    ///
    /// The created frame is memory-safe and thread-safe, and can be
    /// transferred and copied in threads.
    pub fn new(width: u32, height: u32, buf: &[u8]) -> Self {
        assert!(buf.len() >= ((width * height) as f64 * 1.5) as usize);
        let y_stride = width as u32;
        let uv_stride = (width / 2) as u32;
        Self {
            raw: Box::into_raw(Box::new(RawVideoFrame {
                stride_y: y_stride,
                stride_u: uv_stride,
                stride_v: uv_stride,
                buf: buf.as_ptr(),
                len: buf.len(),
                remote: false,
                width,
                height,
            })),
        }
    }
    
    /// get video frame width.
    pub fn width(&self) -> u32 {
        unsafe { &*self.raw }.width
    }
    
    /// get video frame height.
    pub fn height(&self) -> u32 {
        unsafe { &*self.raw }.height
    }

    /// get i420 frame y buffer
    pub fn data_y(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let size = (raw.stride_y * raw.height) as usize;
        unsafe { from_raw_parts(raw.buf, size) }
    }

    /// get i420 frame y stride
    pub fn stride_y(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.stride_y as usize
    }

    /// get i420 frame u buffer
    pub fn data_u(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let y_size = (raw.stride_y * raw.height) as usize;
        let size = (raw.stride_u * (raw.height / 2)) as usize;
        unsafe { from_raw_parts(raw.buf.add(y_size), size) }
    }

    /// get i420 frame u stride
    pub fn stride_u(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.stride_u as usize
    }

    /// get i420 frame v buffer
    pub fn data_v(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let y_size = (raw.stride_y * raw.height) as usize;
        let size = (raw.stride_u * (raw.height / 2)) as usize;
        unsafe { from_raw_parts(raw.buf.add(y_size + size), size) }
    }

    /// get i420 frame v stride
    pub fn stride_v(&self) -> usize {
        let raw = unsafe { &*self.raw };
        raw.stride_v as usize
    }
}

impl AsRef<[u8]> for VideoFrame {
    fn as_ref(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        unsafe { from_raw_parts(raw.buf, raw.len) }
    }
}

impl Drop for VideoFrame {
    fn drop(&mut self) {
        let raw = unsafe { &*self.raw };
        unsafe {
            // If remote is false, it means the distribution is 
            // on the rust box.
            if raw.remote {
                free_video_frame(self.raw)
            } else {
                let _ = Box::from_raw(self.raw.cast_mut());
            };
        }
    }
}
