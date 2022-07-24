mod ffi;

use ffi::*;

#[tokio::main]
async fn main() {
    let config = RTCConfiguration::default();
    let peer = RTCPeerConnection::new(&config).unwrap();
    let offer = peer.create_offer().await.unwrap();
    println!("type: {:?}", offer.get_type());
    println!("sdp: {:?}", offer.get_sdp());
}
