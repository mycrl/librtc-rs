use libc::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ConnectionState {
    New,
    Checking,
    Connected,
    Disconnected,
    Close,
    Failed,
}

pub type RawRTCPeerConnection = c_void;

/// RTCPeerConnection
///
/// The RTCPeerConnection interface represents a WebRTC connection between the 
/// local computer and a remote peer. It provides methods to connect to a remote 
/// peer, maintain and monitor the connection, and close the connection once 
/// it's no longer needed.
pub struct RTCPeerConnection {
    raw: RawRTCPeerConnection
}

impl RTCPeerConnection {
    
}
