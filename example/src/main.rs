use batrachia::*;

#[tokio::main]
async fn main() {
    let config = RTCConfiguration::default();
    let peer = RTCPeerConnection::new(&config).unwrap();
    let eventer = peer.eventer.clone();

    tokio::spawn(async move {
        let offer = peer.create_offer().await.unwrap();
        println!("================================= offer: {:?}", offer);
        peer.set_local_description(&offer).await.unwrap();
    });

    tokio::spawn(async move {
        while let Some(state) = eventer.signalingchange_rev.recv().await {
            println!(
                "======================================= signalingchange_rev: {:?}",
                state
            );
        }
    });

    RTCPeerConnection::run()
}
