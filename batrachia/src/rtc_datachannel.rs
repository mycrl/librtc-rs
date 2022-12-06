use libc::*;
use crate::{
    stream_ext::*,
    base::*,
};

use std::{
    slice::from_raw_parts,
    cell::UnsafeCell,
};

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    fn free_data_channel(channel: *const RawRTCDataChannel);
    
    /// Returns a string which indicates the state of the data channel's
    /// underlying data connection.
    fn data_channel_get_state(channel: *const RawRTCDataChannel) -> DataChannelState;
    
    /// Sends data across the data channel to the remote peer.
    fn data_channel_send(
        channel: *const RawRTCDataChannel,
        buf: *const u8,
        size: c_int,
    );

    fn data_channel_on_message(
        channel: *const RawRTCDataChannel,
        handler: extern "C" fn(*mut Sinker<Vec<u8>>, *const u8, u64),
        ctx: *mut Sinker<Vec<u8>>,
    );
}

/// Indicates the state of the data channel connection.
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum DataChannelState {
    Connecting,
    Open,
    Closing,
    Closed,
}

/// Used to process outgoing WebRTC packets and prioritize outgoing WebRTC
/// packets in case of congestion.
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
    // Negative values are ignored, and positive values are clamped to
    // [0-65535]
    pub max_retransmit_time: Option<u64>,
    // The max number of retransmissions.
    //
    // Cannot be set along with `maxRetransmitTime`.
    // Negative values are ignored, and positive values are clamped to
    // [0-65535]
    pub max_retransmits: Option<u64>,
    // This is set by the application and opaque to the WebRTC implementation.
    pub protocol: String, // = ""
    // True if the channel has been externally negotiated and we do not send an
    // in-band signalling in the form of an "open" message. If this is true,
    // `id` below must be set; otherwise it should be unset and will be
    // negotiated in-band.
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
            priority: self.priority.as_ref().map(|x| *x as c_int).unwrap_or(0),
        }
    }
}

/// The RTCDataChannel interface represents a network channel which can be used
/// for bidirectional peer-to-peer transfers of arbitrary data.
///
/// Every data channel is associated with an RTCPeerConnection, and each peer
/// connection can have up to a theoretical maximum of 65,534 data channels.
#[derive(Debug)]
pub struct RTCDataChannel {
    raw: *const RawRTCDataChannel,
    sink: UnsafeCell<Option<*mut Sinker<Vec<u8>>>>,
}

unsafe impl Send for RTCDataChannel {}
unsafe impl Sync for RTCDataChannel {}

impl RTCDataChannel {
    /// Sends data across the data channel to the remote peer.
    pub fn send(&self, buf: &[u8]) {
        unsafe { data_channel_send(self.raw, buf.as_ptr(), buf.len() as c_int) }
    }

    /// Returns a string which indicates the state of the data channel's
    /// underlying data connection.
    pub fn get_state(&self) -> DataChannelState {
        unsafe { data_channel_get_state(self.raw) }
    }

    /// Used to receive the remote data channel, the channel data of the
    /// remote data channel is pushed to the receiver through the channel.
    pub fn register_sink(&self, sink: Sinker<Vec<u8>>) {
        let sink = Box::into_raw(Box::new(sink));
        let raw_ptr = unsafe { &mut *self.sink.get() };
        let _ = raw_ptr.insert(sink);

        unsafe { data_channel_on_message(self.raw, on_message_callback, sink) }
    }

    pub(crate) fn from_raw(raw: *const RawRTCDataChannel) -> Self {
        assert!(!raw.is_null());
        Self {
            sink: UnsafeCell::new(None),
            raw,
        }
    }
}

impl Drop for RTCDataChannel {
    fn drop(&mut self) {
        unsafe { free_data_channel(self.raw) }
    }
}

#[no_mangle]
extern "C" fn on_message_callback(
    ctx: *mut Sinker<Vec<u8>>,
    buf: *const u8,
    size: u64,
) {
    assert!(!ctx.is_null());
    assert!(!buf.is_null());
    let array = unsafe { from_raw_parts(buf, size as usize) };
    unsafe { &mut *ctx }.sink.on_data(array.to_vec());
}
