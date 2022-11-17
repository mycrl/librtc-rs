use super::base::*;
use libc::*;
use std::convert::*;
use std::ffi::CString;

#[repr(C)]
pub struct I420Frame<'a> {
    width: u32,
    height: u32,

    data_y: &'a [u8],
    stride_y: u32,
    data_u: &'a [u8],
    stride_u: u32,
    data_v: &'a [u8],
    stride_v: u32,
}

impl<'a> I420Frame<'a> {
    pub fn new(width: usize, height: usize, buf: &'a [u8]) -> Self {
        let need_size = ((width * height) as f32 * 1.5) as usize;
        assert!(buf.len() >= need_size);

        let y_size = width * height;
        let u_size = width * height / 4;

        let data_y = &buf[..y_size];
        let data_u = &buf[y_size..y_size + u_size];
        let data_v = &buf[y_size + u_size..];

        Self {
            width: width as u32,
            height: height as u32,
            data_y,
            stride_y: width as u32,
            data_u,
            stride_u: (width / 2) as u32,
            data_v,
            stride_v: (width / 2) as u32,
        }
    }
}

#[repr(C)]
pub struct RawMediaStreamVideoTrack {
    track: *const c_void,
}

#[repr(C)]
pub struct RawMediaStreamTrack {
    pub enabled: bool,
    pub id: *const c_char,
    pub kind: *const c_char,
    pub label: *const c_char,
    pub muted: bool,
    pub ready_state: bool,
    pub remote: bool,
    pub width: u32,
    pub height: u32,
    pub frame_rate: u16,
    pub video_track: *const c_void,
}

#[link(name = "batrachiatc")]
extern "C" {
    fn media_stream_video_track_add_frame(
        track: *const RawMediaStreamTrack,
        frame: *const I420Frame,
    );

    fn create_media_stream_video_track(
        id: *const c_char,
        label: *const c_char,
        width: u32,
        height: u32,
        frame_rate: u16,
    ) -> *const RawMediaStreamTrack;
}

pub struct MediaStreamTrack {
    raw_ptr: *const RawMediaStreamTrack,
    raw_id: *mut c_char,
    raw_label: *mut c_char,
}

impl From<&RawMediaStreamTrack> for MediaStreamTrack {
    fn from(value: &RawMediaStreamTrack) -> Self {
        Self {
            raw_ptr: value as *const RawMediaStreamTrack,
            raw_label: std::ptr::null_mut(),
            raw_id: std::ptr::null_mut(),
        }
    }
}

impl MediaStreamTrack {
    pub fn new(id: String, label: String, width: u32, height: u32, frame_rate: u16) -> Self {
        let raw_id = CString::new(id).unwrap().into_raw();
        let raw_label = CString::new(label).unwrap().into_raw();
        Self {
            raw_id,
            raw_label,
            raw_ptr: unsafe {
                create_media_stream_video_track(raw_id, raw_label, width, height, frame_rate)
            },
        }
    }

    pub fn add_frame<'a>(&self, frame: &I420Frame<'a>) {
        unsafe {
            media_stream_video_track_add_frame(self.raw_ptr, frame);
        }
    }

    pub fn get_raw(&self) -> *const RawMediaStreamTrack {
        self.raw_ptr
    }
}

impl Drop for MediaStreamTrack {
    fn drop(&mut self) {
        free_cstring(self.raw_id);
        free_cstring(self.raw_label);
    }
}
