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
* [Build](#build)


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


## Build

##### Install depot_tools

Clone the depot_tools repository:

```bash
git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git
```

Add depot_tools to the front of your PATH.

Compile with native toolchain (only Windows):

```bash
$env:DEPOT_TOOLS_WIN_TOOLCHAIN=0
```

Clone the batrachia repository:

```bash
git clone https://github.com/mycrl/batrachia
```

Create a third party directory, enter it, and run fetch webrtc:

```bash
cd batrachia
cd batrachiatc
mkdir third_party
cd third_party
mkdir webrtc
cd webrtc
fetch --nohooks --nohistory webrtc
cd src
```

Switch to the M99 branch (only Windows):

```bash
git checkout branch-heads 4844
```

If it is linux/mac, please use the M105 branch, because M99 does not support the M1 chip version of macos:

```bash
git checkout branch-heads 5195
```

Sync webrtc toolchain and many dependencies.
The checkout size is large due the use of the Chromium build toolchain and many dependencies. Estimated size:
* Linux: 6.4 GB.
* Linux (with Android): 16 GB (of which ~8 GB is Android SDK+NDK images).
* Mac (with iOS support): 5.6GB

```bash
gclient sync
```

Compile the Debug target of the webrtc static library:

```bash
gn gen out/Debug --args="is_debug=true \
    is_component_build=false \
    use_lld=false \
    treat_warnings_as_errors=false \
    use_rtti=true \
    rtc_include_tests=false \
    rtc_build_examples=false \
    enable_iterator_debugging=true \
    use_custom_libcxx=false"
ninja -C out/Debug
```

Compile the Release target of the webrtc static library:

```bash
gn gen out/Release --args="is_debug=false \
    is_component_build=false \
    use_lld=false \
    treat_warnings_as_errors=false \
    use_rtti=true \
    rtc_include_tests=false \
    rtc_build_examples=false \
    use_custom_libcxx=false"
ninja -C out/Release
```

Go back to the root directory of batrachia:

```bash
cd batrachia
cd batrachiatc
mkdir out
cd out
```

generate the batrachiatc static library project:

##### Debug

```bash
cmake .. -DCMAKE_BUILD_TYPE=Debug
```

##### Release

```bash
cmake .. -DCMAKE_BUILD_TYPE=Release
```

If it is windows, please open the batrachiatc.sln compilation project under the out directory, if it is linux/mac, directly make:

```bash
make
```


### License
[GPL](./LICENSE) Copyright (c) 2022 Mr.Panda.
