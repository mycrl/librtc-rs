use batrachia::*;

#[tokio::main]
async fn main() {
    let observer = Observer::new();
    let config = RTCConfiguration::default();
    let peer = RTCPeerConnection::new(&config, &observer).unwrap();

    let video_opt = MediaStreamTrackDescription {
        id: "video_track".to_string(),
        label: "video_track".to_string(),
        width: 1920,
        height: 1040,
        frame_rate: 25
    };

    let stream = MediaStream::new("stream_id".to_string()).unwrap();
    let track = MediaStreamTrack::new(&video_opt).unwrap();
    peer.add_track(&track, &stream);

    tokio::spawn(async move {
        let offer = peer.create_offer().await.unwrap();
        println!("offer: {:?}", offer);
        peer.set_local_description(&offer).await.unwrap();
    });

    tokio::spawn(async move {
        while let Some(state) = observer.signaling_change.recv().await {
            println!("signaling_change: {:?}",state);
        }
    });

    tokio::spawn(async move {
        while let Some(candidate) = observer.ice_candidate.recv().await {
            println!("ice_candidate: {:?}", candidate);
        }
    });

    RTCPeerConnection::run()
}
