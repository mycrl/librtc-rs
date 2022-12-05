use std::slice::from_raw_parts;

extern "C" {
    // free the i420 video frame allocated by c.
    fn free_i420_frame(frame: *const RawVideoFrame);
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct RawVideoFrame {
    // frame size
    width: u32,
    height: u32,

    //  i420 frame layout
    data_y: *const u8,
    stride_y: u32,
    data_u: *const u8,
    stride_u: u32,
    data_v: *const u8,
    stride_v: u32,

    // Indicates who created this video frame,
    // c created as true, rust created as false
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
#[derive(Debug)]
pub struct VideoFrame {
    raw: *const RawVideoFrame,
}

unsafe impl Send for VideoFrame {}
unsafe impl Sync for VideoFrame {}

impl VideoFrame {
    pub(crate) fn from_raw(raw: *const RawVideoFrame) -> Self {
        assert!(!raw.is_null());
        Self {
            raw,
        }
    }

    pub(crate) fn get_raw(&self) -> *const RawVideoFrame {
        self.raw
    }

    /// Create i420 frame structure from memory buffer.
    ///
    /// The created frame is memory-safe and thread-safe, and can be
    /// transferred and copied in threads
    pub fn new(width: u32, height: u32, buf: &[u8]) -> Self {
        // Check memory buffer compliance
        let len = ((width * height) as f64 * 1.5) as usize;
        assert!(buf.len() >= len);

        // ----> width
        // | Y0 | Y1 | Y2 | Y3
        // | U0 | U1 |
        // | V0 | V0 |

        // y planar: width * height
        // y stride: width (Does not calculate memory alignment)
        let y_size = (width * height) as usize;
        let y_stride = width as u32;

        // uv planar: (width / 2) * (height / 2)
        // uv stride: width / 2 (Does not calculate memory alignment)
        let uv_size = (width * height / 4) as usize;
        let uv_stride = (width / 2) as u32;

        Self {
            raw: Box::into_raw(Box::new(RawVideoFrame {
                remote: false,

                // frame size
                width,
                height,

                //  yuv ptr
                data_y: buf[..y_size].as_ptr(),
                data_u: buf[y_size..y_size + uv_size].as_ptr(),
                data_v: buf[y_size + uv_size..].as_ptr(),

                // yuv stride
                stride_y: y_stride,
                stride_u: uv_stride,
                stride_v: uv_stride,
            })),
        }
    }

    /// get i420 frame y buffer
    pub fn data_y(&self) -> &[u8] {
        let raw = unsafe { &*self.raw };
        let size = (raw.width * raw.height) as usize;
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
        let size = (raw.width * raw.height / 4) as usize;
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
        let size = (raw.width * raw.height / 4) as usize;
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
        if raw.remote {
            unsafe { free_i420_frame(self.raw) }
        } else {
            unsafe {
                // If remote is false, it means the distribution is on the rust
                // box.
                let _ = Box::from_raw(self.raw as *mut RawVideoFrame);
            }
        }
    }
}
