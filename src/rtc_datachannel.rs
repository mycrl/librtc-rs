use tokio::sync::Mutex;
use libc::*;
use crate::{
    stream_ext::*,
    symbols::*,
    base::*,
};

use std::{
    collections::HashMap,
    slice::from_raw_parts,
    sync::Arc,
};

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
    remote: bool,
}

/// An object providing configuration options for the data channel.
pub struct DataChannelOptions {
    /// Reliability is assumed, and channel will be unreliable if
    /// maxRetransmitTime or MaxRetransmits is set.
    pub reliable: bool, // = false
    /// True if ordered delivery is required.
    pub ordered: bool, // = true
    /// The max period of time in milliseconds in which retransmissions will be
    /// sent. After this time, no more retransmissions will be sent.
    ///
    /// Cannot be set along with `maxRetransmits`.
    /// This is called `maxPacketLifeTime` in the WebRTC JS API.
    /// Negative values are ignored, and positive values are clamped to
    /// [0-65535]
    pub max_retransmit_time: Option<u64>,
    /// The max number of retransmissions.
    ///
    /// Cannot be set along with `maxRetransmitTime`.
    /// Negative values are ignored, and positive values are clamped to
    /// [0-65535]
    pub max_retransmits: Option<u64>,
    /// This is set by the application and opaque to the WebRTC implementation.
    pub protocol: String, // = ""
    /// True if the channel has been externally negotiated and we do not send
    /// an in-band signalling in the form of an "open" message. If this is
    /// true, `id` below must be set; otherwise it should be unset and will
    /// be negotiated in-band.
    pub negotiated: bool, // = false
    /// The stream id, or SID, for SCTP data channels. -1 if unset (see above).
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
            id: self.id as c_int,
            reliable: self.reliable,
            ordered: self.ordered,
            negotiated: self.negotiated,
            protocol: to_c_str(&self.protocol).unwrap(),
            max_retransmits: self.max_retransmits.unwrap_or(0),
            max_retransmit_time: self.max_retransmit_time.unwrap_or(0),
            priority: self.priority.as_ref().map(|x| *x as c_int).unwrap_or(0),
        }
    }
}

/// The RTCDataChannel interface represents a network channel which can be used
/// for bidirectional peer-to-peer transfers of arbitrary data.
///
/// Every data channel is associated with an RTCPeerConnection, and each peer
/// connection can have up to a theoretical maximum of 65,534 data channels.
pub struct DataChannel {
    raw: *const RawRTCDataChannel,
    sinks: Mutex<HashMap<u8, Sinker<Vec<u8>>>>,
}

unsafe impl Send for DataChannel {}
unsafe impl Sync for DataChannel {}

/// Arc DataChannel.
pub type RTCDataChannel = Arc<DataChannel>;

impl DataChannel {
    /// Sends data across the data channel to the remote peer.
    pub fn send(&self, buf: &[u8]) {
        assert!(!unsafe { &*self.raw }.remote);
        unsafe {
            rtc_send_data_channel_msg(
                self.raw,
                buf.as_ptr(),
                buf.len() as c_int,
            )
        }
    }

    /// Returns a string which indicates the state of the data channel's
    /// underlying data connection.
    pub fn get_state(&self) -> DataChannelState {
        unsafe { rtc_get_data_channel_state(self.raw) }
    }

    /// Register channel data sink, one channel can register multiple sinks.
    /// The sink id cannot be repeated, otherwise the sink implementation will
    /// be overwritten.
    pub async fn register_sink(&self, id: u8, sink: Sinker<Vec<u8>>) {
        assert!(unsafe { &*self.raw }.remote);
        let mut sinks = self.sinks.lock().await;

        // Register for the first time, register the callback function to
        // webrtc native, and then do not need to register again.
        if sinks.is_empty() {
            unsafe {
                rtc_set_data_channel_msg_h(self.raw, on_channal_data, self)
            }
        }

        sinks.insert(id, sink);
    }

    /// Delete the registered sink, if it exists, it will return the deleted
    /// sink.
    pub async fn remove_sink(&self, id: u8) -> Option<Sinker<Vec<u8>>> {
        assert!(unsafe { &*self.raw }.remote);
        let mut sinks = self.sinks.lock().await;
        let value = sinks.remove(&id);
        if sinks.is_empty() {
            unsafe { rtc_remove_data_channel_msg_h(self.raw) }
        }

        value
    }

    /// Create data channel from raw type ptr.
    pub(crate) fn from_raw(raw: *const RawRTCDataChannel) -> Arc<Self> {
        assert!(!raw.is_null());
        Arc::new(Self {
            sinks: Mutex::new(HashMap::new()),
            raw,
        })
    }

    fn on_data(this: &Self, data: Vec<u8>) {
        if let Ok(mut sinks) = this.sinks.try_lock() {
            for sinker in sinks.values_mut() {
                sinker.sink.on_data(data.clone());
            }
        }
    }
}

impl Drop for DataChannel {
    fn drop(&mut self) {
        unsafe { rtc_remove_data_channel_msg_h(self.raw) }
        unsafe { rtc_free_data_channel(self.raw) }
    }
}

#[no_mangle]
extern "C" fn on_channal_data(ctx: &DataChannel, buf: *const u8, size: u64) {
    assert!(!buf.is_null());
    let array = unsafe { from_raw_parts(buf, size as usize) };
    DataChannel::on_data(ctx, array.to_vec());
}
