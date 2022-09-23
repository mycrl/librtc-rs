use webrtc_wrapper::*;

#[tokio::main]
async fn main() {
    let config = rtc_peerconnection_configure::RTCConfiguration::default();
    let peer = rtc_peerconnection::RTCPeerConnection::new(&config).unwrap();
    let eventer = peer.eventer.clone();

    tokio::spawn(async move {
        let offer = peer.create_offer().await.unwrap();
        println!("================================= offer: {:?}", offer);
    });

    tokio::spawn(async move {
        while let Some(state) = eventer.connectionstatechange_rev.recv().await {
            println!(
                "======================================= connectionstatechange: {:?}",
                state
            );
        }
    });

    rtc_peerconnection::RTCPeerConnection::run()
}
