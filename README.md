<h1 align="center">
    <img src="./logo.png">
</h1>
<div align="center">
    <strong>Rust ❤️ WebRTC</strong>
    </br>
    <span>Facilitating high-level interactions between Rust and WebRTC</span>
</div>
</br>
<div align="center">
    <img src="https://img.shields.io/github/workflow/status/mycrl/batrachia/tests"/>
    <img src="https://img.shields.io/github/languages/top/mycrl/batrachia"/>
    <img src="https://img.shields.io/github/license/mycrl/batrachia"/>
    <img src="https://img.shields.io/github/issues/mycrl/batrachia"/>
    <img src="https://img.shields.io/github/stars/mycrl/batrachia"/>
</div>
<br/>
<br/>


The rust high-level abstraction binding of Google WebRTC [M99](https://groups.google.com/g/discuss-webrtc/c/Yf6c3HW4N3k/m/3SC_Hy15BQAJ). With WebRTC, you can add real-time communication capabilities to your application that works on top of an open standard. It supports video, voice, and generic data to be sent between peers, allowing developers to build powerful voice- and video-communication solutions.


## Table of contents

* [Platform Support](#platform-support)
* [Example](#example)
* [Features](#features)  


## Platform Support

The windows clang target is not a high priority, other platform support is being done quickly.  

|          | x64-msvc | x86-msvc | x64-clang | x86-clang | arm64 |
|----------|----------|----------|-----------|-----------|-------|
| Windows  | √        |   √      | x         | x         | x     |
| Macos    | x        |   x      | ?         | ?         | √     |
| Linux    | x        |   x      | √         | √         | ?     |


## Example

```rust
use batrachia::*;

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
    
    tokio::spawn(async move {
        while let Some(channel) = observer.data_channel.recv().await {
            let mut sink = channel.get_sink();
            while let Ok(data) = sink.receiver.recv().await {
                // rtc channel data
            }
        }
    });
    
    // create local video media track
    let stream = MediaStream::new("stream_id")?;
    let track = MediaStreamTrack::new("video_track", MediaStreamTrackKind::Video)?;

    // push video track in peer
    peer.add_track(track.clone(), stream);
    
    // push empty frame to local video track
    let buf = vec![0u8; (1920.0 * 1080.0 * 1.5) as usize];
    let frame = I420Frame::new(1920, 1080, &buf[..]);
    track.add_frame(&frame);
    
    // create local offer description
    let offer = peer.create_offer().await?;
    peer.set_local_description(&answer).await?;
    
    // webrtc inner pool
    batrachia::run();
    Ok(())
}
```

## Features

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
* [x] RTCPeerConnection::create_data_channel

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

##### DataChannel
* [x] DataChannel
* [x] DataChannel::send
* [x] DataChannel::get_sink

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
