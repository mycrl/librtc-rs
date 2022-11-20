<h1 align="center">
    Batrachia
</h1>
<div align="center">
    <strong>Rust ❤️ WebRTC - Facilitating high-level interactions between Rust and WebRTC.</strong>
</div>
<div align="center">
    <img src="https://img.shields.io/github/workflow/status/mycrl/batrachia/tests"/>
    <img src="https://img.shields.io/github/languages/top/mycrl/batrachia"/>
    <img src="https://img.shields.io/github/license/mycrl/batrachia"/>
    <img src="https://img.shields.io/github/issues/mycrl/batrachia"/>
    <img src="https://img.shields.io/github/stars/mycrl/batrachia"/>
</div>
<br/>
<br/>


### Example

```rust
use batrachia::*;

const Width: f64 = 1920.0;
const Height: f64 = 1080.0;

const StreamId: &'static str = "stream_id";
const TrackId: &'static str = "video_track";

#[tokio::main]
async main() -> Result<(), anyhow::Error> {
    let observer = Observer::new();
    let config = RTCConfiguration::default();
    let mut peer = RTCPeerConnection::new(&config, &observer)?;

    tokio::spawn(async move {
        while let Some(_state) = observer.signaling_change.recv().await {
            // signaling change state
        }
    });

    tokio::spawn(async move {
        // peer video track
        while let Some(track) = observer.track.recv().await {
            let mut sink = track.get_sink(); // get video track sink
            while let Ok(_frame) = sink.receiver.recv().await {
               // peer video track frame
            }
        }
    });
    
    // create local video media track
    let stream = MediaStream::new(StreamId.to_string())?;
    let track = MediaStreamTrack::new(TrackId, TrackId, MediaStreamTrackKind::Video)?;

    // push video track in peer
    peer.add_track(track.clone(), stream);
    
    // push empty frame to local video track
    let buf = vec![0u8; (Width * Height * 1.5) as usize];
    let frame = I420Frame::new(Width as usize, Height as usize, &buf[..]);
    track.add_frame(&frame);
    
    // create local offer description
    let offer = peer.create_offer().await?;
    peer.set_local_description(&answer).await?;
    
    // webrtc inner pool
    RTCPeerConnection::run();
    Ok(())
}
```

### Features

##### RTCPeerConnection
* [x] RTCConfiguration
* [x] RTCConfiguration::RTCIceServer
* [x] RTCPeerConnection
* [x] RTCPeerConnection::create_offer
* [x] RTCPeerConnection::create_answer
* [x] RTCPeerConnection::set_local_description
* [x] RTCPeerConnection::set_remote_description
* [x] RTCPeerConnection::add_ice_candidate
* [x] RTCPeerConnection::add_track

##### Description
* [x] RTCSessionDescription
* [x] RTCIceCandidate

##### MediaStream
* [x] MediaStream
* [x] MediaStream::tracks
* [x] MediaStreamTrack
* [x] MediaStreamTrack::add_frame
* [x] MediaStreamTrack::get_sink
* [x] MediaStreamTrackSink
* [x] I420Frame

##### Observer
* [x] Observer
* [x] Observer::signaling_change
* [x] Observer::connection_change
* [x] Observer::ice_gathering_change
* [x] Observer::ice_candidate
* [x] Observer::renegotiation_needed
* [x] Observer::ice_connection_change
* [x] Observer::track

### License
[GPL](./LICENSE) Copyright (c) 2022 Mr.Panda.
