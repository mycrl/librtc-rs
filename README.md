<h1 align="center">
    <img src="./logo.png" width="200px">
</h1>
<div align="center">
    <strong>Rust ❤️ WebRTC</strong>
    </br>
    <span>Facilitating high-level interactions between Rust and WebRTC</span>
</div>
</br>
<div align="center">
    <img src="https://img.shields.io/github/languages/top/colourful-rtc/batrachia"/>
    <img src="https://img.shields.io/github/license/colourful-rtc/batrachia"/>
    <img src="https://img.shields.io/github/issues/colourful-rtc/batrachia"/>
    <img src="https://img.shields.io/github/stars/colourful-rtc/batrachia"/>
</div>
<br/>
<br/>


The rust high-level abstraction binding of Google WebRTC [M99](https://groups.google.com/g/discuss-webrtc/c/Yf6c3HW4N3k/m/3SC_Hy15BQAJ). With WebRTC, you can add real-time communication capabilities to your application that works on top of an open standard. It supports video, voice, and generic data to be sent between peers, allowing developers to build powerful voice- and video-communication solutions.


### Quick start

Add the following to your Cargo.toml:

```toml
batrachia = "0.1"
```

There are simple example in the [example](https://github.com/colourful-rtc/example) repo.

### Building

#### Automatic

The batrachia crate will automatically find the precompiled static library files in the git batrachiatc repo release.

#### Manual

A set of environment variables can be used to point batrachia towards. They will override the automatic detection logic.

| WEBRTC_LIBRARY_PATH | webrtc static library path, this will skip downloading and use your static library.      |
|---------------------|------------------------------------------------------------------------------------------|
| SYS_LIBRARY_PATH    | batrachiatc static library path, this will skip downloading and use your static library. |


### License
[GPL](./LICENSE) Copyright (c) 2022 Mr.Panda.
