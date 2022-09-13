mod media_stream;
mod media_stream_track;
mod rtc_datachannel;
mod rtc_icecandidate;
mod rtc_peerconnection;
mod rtc_peerconnection_configure;
mod rtc_session_description;

// #[link(name = "rtc_wrapper")]
// extern "C" {
//     pub fn rtc_run();
//     pub fn create_rtc_peer_connection(
//         config: *const RTCPeerConnectionConfigure,
//     ) -> *const RTCPeerConnection;
//     pub fn rtc_add_ice_candidate(
//         peer: *const RTCPeerConnection,
//         icecandidate: *const RTCIceCandidate,
//     );

//     pub fn media_stream_track_write_frame(
//         track: *const MediaStreamTrack,
//         frame: *const MediaStreamTrackFrame,
//     );

//     pub fn media_stream_track_on_frame(
//         track: *const MediaStreamTrack,
//         callback: extern "C" fn(MediaStreamTrackFrame),
//     );

//     pub fn rtc_add_track(peer: *const RTCPeerConnection, track: *const MediaStreamTrack);

//     pub fn rtc_close(peer: *const RTCPeerConnection);

//     pub fn rtc_create_answer(
//         peer: *const RTCPeerConnection,
//         callback: extern "C" fn(*const c_char, *const RTCSessionDescription, *mut c_void),
//         ctx: *mut c_void,
//     );

//     pub fn rtc_create_offer(
//         peer: *const RTCPeerConnection,
//         callback: extern "C" fn(*const c_char, *const RTCSessionDescription, *mut c_void),
//         ctx: *mut c_void,
//     );

//     pub fn rtc_set_local_description(
//         peer: *const RTCPeerConnection,
//         desc: *const RTCSessionDescription,
//         callback: extern "C" fn(*const c_char, *mut c_void),
//         ctx: *mut c_void,
//     );

//     pub fn rtc_set_remote_description(
//         peer: *const RTCPeerConnection,
//         desc: *const RTCSessionDescription,
//         callback: extern "C" fn(*const c_char, *mut c_void),
//         ctx: *mut c_void,
//     );

//     pub fn rtc_on_connectionstatechange(
//         peer: *const RTCPeerConnection,
//         callback: extern "C" fn(ConnectionState),
//     );

//     pub fn rtc_on_datachannel(
//         peer: *const RTCPeerConnection,
//         callback: extern "C" fn(RTCDataChannel),
//     );

//     pub fn rtc_free_session_description(desc: *const RTCSessionDescription);
// }
