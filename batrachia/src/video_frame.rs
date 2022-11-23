use std::slice::from_raw_parts;
use std::sync::Arc;

extern "C" {
    fn free_i420_frame(frame: *const RawI420Frame);
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct RawI420Frame {
    width: u32,
    height: u32,
    data_y: *const u8,
    stride_y: u32,
    data_u: *const u8,
    stride_u: u32,
    data_v: *const u8,
    stride_v: u32,
    remote: bool,
}

impl RawI420Frame {
    pub (crate) fn new(layout: &I420Layout, buf: &[u8]) -> Self {
        Self {
            width: layout.width,
            height: layout.height,
            data_y: buf[..layout.y_size].as_ptr(),
            stride_y: layout.y_stride,
            data_u: buf[layout.y_size..layout.y_size + layout.u_size].as_ptr(),
            stride_u: layout.u_stride,
            data_v: buf[layout.y_size + layout.u_size..].as_ptr(),
            stride_v: layout.v_stride,
            remote: false,
        }
    }
}

/// Describe the layout information of i420.
#[derive(Debug, Clone, Copy)]
pub struct I420Layout {
    pub width: u32,
    pub height: u32,
    pub len: usize,
    pub y_size: usize,
    pub y_stride: u32,
    pub u_size: usize,
    pub u_stride: u32,
    pub v_size: usize,
    pub v_stride: u32,
}

impl I420Layout {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            len: ((width * height) as f64 * 1.5) as usize,
            y_size: (width * height) as usize,
            y_stride: width as u32,
            u_size: (width * height / 4) as usize,
            u_stride: (width / 2) as u32,
            v_size: (width * height / 4) as usize,
            v_stride: (width / 2) as u32,
        }
    }
}

/// I420Frame represents the frame of the video, and the format is i420 (yu12).
#[derive(Debug)]
pub struct I420Frame {
    raw_ptr: *const RawI420Frame,
}

unsafe impl Send for I420Frame {}
unsafe impl Sync for I420Frame {}

impl I420Frame {
    pub(crate) fn from_raw(raw_ptr: *const RawI420Frame) -> Arc<Self> {
        assert!(!raw_ptr.is_null());
        Arc::new(Self {
            raw_ptr,
        })
    }

    pub(crate) fn get_raw(&self) -> *const RawI420Frame {
        self.raw_ptr
    }

    pub fn new(layout: &I420Layout, buf: &[u8]) -> Arc<Self> {
        assert!(buf.len() >= layout.len);
        let raw_ptr = Box::into_raw(Box::new(RawI420Frame::new(layout, buf)));
        Arc::new(Self { raw_ptr })
    }

    pub fn from_slice(width: u32, height: u32, buf: &[u8]) -> Arc<Self> {
        Self::new(&I420Layout::new(width, height), buf)
    }

    pub fn data_y(&self) -> &[u8] {
        let raw = unsafe { &*self.raw_ptr };
        let size = (raw.width * raw.height) as usize;
        unsafe { from_raw_parts(raw.data_y, size) }
    }

    pub fn stride_y(&self) -> usize {
        let raw = unsafe { &*self.raw_ptr };
        raw.stride_y as usize
    }

    pub fn data_u(&self) -> &[u8] {
        let raw = unsafe { &*self.raw_ptr };
        let size = (raw.width * raw.height / 4) as usize;
        unsafe { from_raw_parts(raw.data_u, size) }
    }

    pub fn stride_u(&self) -> usize {
        let raw = unsafe { &*self.raw_ptr };
        raw.stride_u as usize
    }

    pub fn data_v(&self) -> &[u8] {
        let raw = unsafe { &*self.raw_ptr };
        let size = (raw.width * raw.height / 4) as usize;
        unsafe { from_raw_parts(raw.data_v, size) }
    }

    pub fn stride_v(&self) -> usize {
        let raw = unsafe { &*self.raw_ptr };
        raw.stride_v as usize
    }
}

impl Drop for I420Frame {
    fn drop(&mut self) {
        let raw = unsafe { &*self.raw_ptr };
        if raw.remote {
            unsafe { free_i420_frame(self.raw_ptr) }
        } else {
            unsafe {
                let _ = Box::from_raw(self.raw_ptr as *mut RawI420Frame);
            }
        }
    }
}
