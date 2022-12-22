use libc::*;
use std::collections::HashMap;
use crate::{base::*, abstracts::VectorLayout};

#[repr(C)]
struct RawParameter {
	key: *const c_char,
	value: *const c_char,
}

#[repr(C)]
struct RawCodec {
	name: *const c_char,
	pars: *const *const RawParameter,
    capacity: usize,
	len: usize,
}

#[repr(C)]
struct RawCodecs {
	codecs: *const *const RawCodec,
    capacity: usize,
	len: usize,
}

pub struct VideoEncoder {
    name: String,
    parameters: HashMap<String, String>,
}

pub struct VideoEncoderFactory {
    ptr: *const RawCodecs,
}

impl VideoEncoderFactory {
    pub fn new(codecs: Vec<VideoEncoder>) -> Self {
        let (codecs, len, capacity) = codecs.iter().map(|item| {
            let mut pars = Vec::new();

            item.parameters.iter().for_each(|(k, v)| {
                pars.push(Box::into_raw(Box::new(RawParameter {
                    key: to_c_str(k).unwrap(), 
                    value: to_c_str(v).unwrap(),
                })) as *const RawParameter)
            });

            let (pars, len, capacity) = pars.into_c_layout();
            Box::into_raw(Box::new(RawCodec {
                name: to_c_str(&item.name).unwrap(),
                capacity,
                pars,
                len,
            })) as *const RawCodec
        }).collect::<Vec<*const RawCodec>>().into_c_layout();
        
        Self {
            ptr: Box::into_raw(Box::new(RawCodecs {
                codecs,
                capacity,
                len,
            }))
        }
    }
}
