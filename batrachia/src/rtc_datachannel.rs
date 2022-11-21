use std::sync::Arc;
use libc::*;
use super::base::*;
use tokio::sync::broadcast::*;
use anyhow::{
    anyhow,
    Result,
};

#[link(name = "batrachiatc")]
extern "C" {
    fn data_channel_send(
        channel: *const RawRTCDataChannel, 
        buf: *const u8, 
        size: c_int
    );

    #[allow(improper_ctypes)]
    fn data_channel_on_message(
        channel: *const RawRTCDataChannel, 
        handler: extern "C" fn(*mut Sender<Vec<u8>>, *const u8, u64),
        ctx: *mut Sender<Vec<u8>>,
    );

    fn data_channel_get_state(channel: *const RawRTCDataChannel) -> DataChannelState;
    fn free_data_channel(channel: *const RawRTCDataChannel);
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum DataChannelState {
    Connecting,
    Open,
    Closing,
    Closed
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum DataChannelPriority {
    VeryLow = 1,
    Low,
    Medium,
    High,
}

#[repr(C)]
pub(crate) struct RawDataChannelOptions {
    reliable: bool,
    ordered: bool,
    max_retransmit_time: u64,
    max_retransmits: u64,
    protocol: *const c_char,
    negotiated: bool,
    id: c_int,
    // Priority
    priority: c_int,
}

impl Drop for RawDataChannelOptions {
    fn drop(&mut self) {
        free_cstring(self.protocol)
    }
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct RawRTCDataChannel {
    label: *const c_char,
    channel: *const c_void,
}

pub struct DataChannelOptions {
    // Deprecated. Reliability is assumed, and channel will be unreliable if
    // maxRetransmitTime or MaxRetransmits is set.
    pub reliable: bool, // = false
    // True if ordered delivery is required.
    pub ordered: bool, // = true
    // The max period of time in milliseconds in which retransmissions will be
    // sent. After this time, no more retransmissions will be sent.
    //
    // Cannot be set along with `maxRetransmits`.
    // This is called `maxPacketLifeTime` in the WebRTC JS API.
    // Negative values are ignored, and positive values are clamped to [0-65535]
    pub max_retransmit_time: Option<u64>,
    // The max number of retransmissions.
    //
    // Cannot be set along with `maxRetransmitTime`.
    // Negative values are ignored, and positive values are clamped to [0-65535]
    pub max_retransmits: Option<u64>,
    // This is set by the application and opaque to the WebRTC implementation.
    pub protocol: String, // = ""
    // True if the channel has been externally negotiated and we do not send an
    // in-band signalling in the form of an "open" message. If this is true, `id`
    // below must be set; otherwise it should be unset and will be negotiated
    // in-band.
    pub negotiated: bool, // = false 
    // The stream id, or SID, for SCTP data channels. -1 if unset (see above).
    pub id: i8,
    pub priority: Option<DataChannelPriority>,
}

impl Default for DataChannelOptions {
    fn default() -> Self {
        Self {
            reliable: false,
            ordered: true,
            max_retransmit_time: None,
            max_retransmits: None,
            protocol: "".to_string(),
            negotiated: false,
            id: 0,
            priority: None,
        }
    }
}

impl Into<RawDataChannelOptions> for &DataChannelOptions {
    fn into(self) -> RawDataChannelOptions {
        RawDataChannelOptions {
            reliable: self.reliable,
            ordered: self.ordered,
            max_retransmit_time: self.max_retransmit_time.unwrap_or(0),
            max_retransmits: self.max_retransmits.unwrap_or(0),
            protocol: to_c_str(&self.protocol).unwrap(),
            negotiated: self.negotiated,
            id: self.id as c_int,
            priority: self.priority
                .as_ref()
                .map(|x| *x as c_int)
                .unwrap_or(0),
        }
    }
}

#[derive(Debug)]
pub struct RTCDataChannel {
    raw: *const RawRTCDataChannel,
}

unsafe impl Send for RTCDataChannel {}
unsafe impl Sync for RTCDataChannel {}

impl RTCDataChannel {
    pub(crate) fn from_raw(raw: *const RawRTCDataChannel) -> Result<Arc<Self>> {
        if raw.is_null() {
            Err(anyhow!("create media stream track failed!"))
        } else {
            Ok(Arc::new(Self { raw }))
        }
    }

    pub fn send(&self, buf: &[u8]) {
        unsafe {
            data_channel_send(self.raw, buf.as_ptr(), buf.len() as c_int)
        }
    }

    pub fn get_state(&self) -> DataChannelState {
        unsafe { data_channel_get_state(self.raw) }
    }

    pub fn get_sink(&self) -> RTCDataChannelSink {
        let (tx, receiver) = channel(1);
        let sender = Box::into_raw(Box::new(tx));
        unsafe { data_channel_on_message(self.raw, on_message_callback, sender) }

        RTCDataChannelSink { 
            receiver, 
            sender 
        }
    }
}

impl Drop for RTCDataChannel {
    fn drop(&mut self) {
        unsafe { free_data_channel(self.raw) }
    }
}

pub struct RTCDataChannelSink {
    pub receiver: Receiver<Vec<u8>>,
    sender: *mut Sender<Vec<u8>>,
}

unsafe impl Send for RTCDataChannelSink {}
unsafe impl Sync for RTCDataChannelSink {}

impl Drop for RTCDataChannelSink {
    fn drop(&mut self) {
        unsafe { let _ = Box::from_raw(self.sender); }
    }
}

extern "C" fn on_message_callback(ctx: *mut Sender<Vec<u8>>, buf: *const u8, size: u64) {
    if !buf.is_null() {
        unsafe { &*ctx }.send(unsafe { 
            std::slice::from_raw_parts(buf, size as usize) 
        }.to_vec()).unwrap();
    }
}
