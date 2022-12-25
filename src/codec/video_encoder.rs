use libc::*;
use std::collections::HashMap;
use crate::{
    base::*,
    abstracts::VectorLayout,
};

#[repr(C)]
struct RawParameter {
    key:   *const c_char,
    value: *const c_char,
}

impl From<(&str, &str)> for RawParameter {
    fn from(value: (&str, &str)) -> Self {
        Self {
            key:   to_c_str(value.0).unwrap(),
            value: to_c_str(value.1).unwrap(),
        }
    }
}

#[repr(C)]
struct RawCodec {
    name:     *const c_char,
    pars:     *const *const RawParameter,
    capacity: usize,
    len:      usize,
}

impl From<&VideoEncoder> for RawCodec {
    fn from(value: &VideoEncoder) -> Self {
        let (pars, len, capacity) = value
            .parameters
            .iter()
            .map(|(k, v)| {
                Box::into_raw(Box::new(RawParameter::from((
                    k.as_str(),
                    v.as_str(),
                )))) as *const RawParameter
            })
            .collect::<Vec<*const RawParameter>>()
            .into_c_layout();
        Self {
            name: to_c_str(&value.name).unwrap(),
            capacity,
            pars,
            len,
        }
    }
}

#[repr(C)]
struct RawCodecs {
    codecs:   *const *const RawCodec,
    capacity: usize,
    len:      usize,
}

impl From<&[VideoEncoder]> for RawCodecs {
    fn from(value: &[VideoEncoder]) -> Self {
        let (codecs, len, capacity) = value
            .iter()
            .map(|item| {
                Box::into_raw(Box::new(RawCodec::from(item))) as *const RawCodec
            })
            .collect::<Vec<*const RawCodec>>()
            .into_c_layout();
        Self {
            codecs,
            capacity,
            len,
        }
    }
}

pub struct VideoEncoderAdapter {

}

pub trait VideoEncoderExt {
    fn encode(&mut self, adapter: &mut VideoEncoderAdapter);
}

pub struct VideoEncoder {
    name:       String,
    parameters: HashMap<String, String>,
}

impl VideoEncoder {
    pub fn new(name: &str, pars: &[(&str, &str)]) -> Self {
        
    }
}

pub struct VideoEncoderFactory {
    ptr: *const RawCodecs,
}

impl VideoEncoderFactory {
    pub fn new(codecs: Vec<VideoEncoder>) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(RawCodecs::from(codecs.as_slice()))),
        }
    }
}
