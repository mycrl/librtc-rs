use libc::*;

#[rustfmt::skip]
type PCC = *const crate::rtc_peerconnection_configure::RawRTCPeerConnectionConfigure;
type PC = *const crate::rtc_peerconnection::RawRTCPeerConnection;
type DTC = *const crate::rtc_datachannel::RawDataChannelOptions;
type DT = *const crate::rtc_datachannel::RawRTCDataChannel;
type DTS = crate::rtc_datachannel::DataChannelState;
type SD = *const crate::rtc_session_description::RawRTCSessionDescription;
type MT = *const crate::media_stream_track::RawMediaStreamTrack;
type IC = *const crate::rtc_icecandidate::RawRTCIceCandidate;
type AF = *const crate::audio_frame::RawAudioFrame;
type VF = *const crate::video_frame::RawVideoFrame;
type EV = *const crate::observer::TEvents;
type OS = *mut crate::observer::Observer;
type RAT = crate::audio_track::AudioTrack;
type RVT = crate::video_track::VideoTrack;
type RDC = crate::DataChannel;

#[rustfmt::skip]
#[allow(improper_ctypes)]
extern "C" {
    pub(crate) fn rtc_run();
    pub(crate) fn rtc_exit();
    pub(crate) fn rtc_create_peer_connection(config: PCC, events: EV, observer: OS) -> PC;
    pub(crate) fn rtc_add_ice_candidate(peer: PC, icecandidate: IC) -> bool;
    pub(crate) fn rtc_add_media_stream_track(peer: PC, track: MT, id: *const c_char,);
    pub(crate) fn rtc_create_data_channel(peer: PC, label: *const c_char, options: DTC) -> DT;
    pub(crate) fn rtc_close(peer: PC);
    pub(crate) fn rtc_create_answer(pc: PC, cb: extern "C" fn(*const c_char, SD, *mut c_void), ctx: *mut c_void);
    pub(crate) fn rtc_create_offer(pc: PC, cb: extern "C" fn(*const c_char, SD, *mut c_void), ctx: *mut c_void);
    pub(crate) fn rtc_set_local_description(peer: PC, desc: SD, callback: extern "C" fn(*const c_char, *mut c_void), ctx: *mut c_void);
    pub(crate) fn rtc_set_remote_description(peer: PC, desc: SD, callback: extern "C" fn(*const c_char, *mut c_void), ctx: *mut c_void);
    pub(crate) fn rtc_create_audio_track(label: *const c_char) -> MT;
    pub(crate) fn rtc_set_audio_track_frame_h(track: MT, handler: extern "C" fn(&RAT, AF), ctx: &RAT);
    pub(crate) fn rtc_add_audio_track_frame(track: MT, frame: AF);
    pub(crate) fn rtc_create_video_track(label: *const c_char) -> MT;
    pub(crate) fn rtc_add_video_track_frame(track: MT, frame: VF);
    pub(crate) fn rtc_set_video_track_frame_h(track: MT, handler: extern "C" fn(&RVT, VF), ctx: &RVT);
    pub(crate) fn rtc_remove_media_stream_track_frame_h(track: MT);
    pub(crate) fn rtc_free_media_stream_track(track: MT);
    pub(crate) fn rtc_free_frame(frame: *const c_void);
    pub(crate) fn rtc_get_data_channel_state(channel: DT) -> DTS;
    pub(crate) fn rtc_send_data_channel_msg(channel: DT, buf: *const u8, size: c_int);
    pub(crate) fn rtc_set_data_channel_msg_h(channel: DT, handler: extern "C" fn(&RDC, *const u8, u64), ctx: &RDC);
    pub(crate) fn rtc_remove_data_channel_msg_h(channel: DT);
    pub(crate) fn rtc_free_data_channel(channel: DT);
}
