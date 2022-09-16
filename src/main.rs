#![feature(vec_into_raw_parts)]
#![feature(box_into_inner)]

mod ffi;

use ffi::*;

#[tokio::main]
async fn main() {
    let config = rtc_peerconnection_configure::RTCConfiguration::default();
    let peer = rtc_peerconnection::RTCPeerConnection::new(config).unwrap();

    tokio::spawn(async move {
        let offer = peer.create_offer().await.unwrap();
        println!("================================= offer: {:?}", offer);
    });

    rtc_peerconnection::RTCPeerConnection::run()
}
