mod ffi;

#[cfg(test)]
mod tests {
    use super::ffi;

    #[test]
    fn is_works() {
        // unsafe {
        //     let config = ffi::RTCPeerConnectionConfigure::default();
        //     let peer = ffi::create_rtc_peer_connection(&config);
        //     assert!(!peer.is_null());

        //     ffi::rtc_create_offer(peer, |desc| {

        //     });
        // }
    }
}
