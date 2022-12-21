use libc::*;
use std::collections::HashMap;

#[repr(C)]
struct RawParameter {
	key: *const c_char,
	value: *const c_char,
}

#[repr(C)]
struct RawCodec {
	name: *const c_char,
	pars: *const *const RawParameter,
	len: usize,
}

#[repr(C)]
struct RawCodecs {
	codecs: *const *const RawCodec,
	len: usize,
}

pub struct VideoEncoder {
    name: String,
    parameters: HashMap<String, String>,
}

pub struct VideoEncoderFactory {
    codecs: Vec<VideoEncoder>,
}

impl VideoEncoderFactory {
    
}
