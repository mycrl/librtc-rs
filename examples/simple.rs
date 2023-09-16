use clap::*;
use minifb::{Window, WindowOptions};
use serde::{Deserialize, Serialize};
use std::{
    mem::ManuallyDrop,
    ptr::null_mut,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
    time::Duration,
};

use librtc::*;
use futures_util::{stream::*, SinkExt, StreamExt};
use tokio::{net::TcpStream, runtime::Handle, sync::Mutex};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "lowercase")]
enum Payload {
    Offer(RTCSessionDescription),
    Answer(RTCSessionDescription),
    Candidate(RTCIceCandidate),
}

// Remote video track player window.
struct VideoPlayer {
    ready: bool,
    label: String,
    buf: Arc<AtomicPtr<Vec<u8>>>,
}

impl VideoPlayer {
    fn new(label: String) -> Self {
        Self {
            buf: Arc::new(AtomicPtr::new(null_mut())),
            ready: false,
            label,
        }
    }

    // Process video frames decoded from video track.
    fn on_frame(&mut self, frame: &VideoFrame) {
        let width = frame.width() as usize;
        let height = frame.height() as usize;

        // Check whether the window has been created. If not, create the
        // window first. The reason for this is that it must be created
        // according to the size of the video frame
        if self.ready == false {
            self.ready = true;

            let buf = self.buf.clone();
            let label = self.label.clone();
            let delay = Duration::from_millis(1000 / 60);
            std::thread::spawn(move || {
                let mut window = Window::new(
                    &format!("video::{}", label),
                    width,
                    height,
                    WindowOptions::default(),
                )?;

                // Renders the latest frame from the framebuffer at a
                // fixed rate of 60 frames.
                window.limit_update_rate(Some(delay));
                loop {
                    std::thread::sleep(delay);
                    let frame = unsafe { &*buf.load(Ordering::Relaxed) };
                    let (_, shorts, _) = unsafe { frame.align_to::<u32>() };
                    window.update_with_buffer(shorts, width, height)?;
                }

                #[allow(unreachable_code)]
                Ok::<(), anyhow::Error>(())
            });
        }

        // The frame format of the video track output is fixed to I420,
        // but the window only accepts ARGB, so here you need to
        // convert I420 to ARGB.
        let mut buf = vec![0u8; width * height * 4];
        unsafe {
            libyuv::i420_to_argb(
                frame.data_y().as_ptr(),
                frame.stride_y() as i32,
                frame.data_u().as_ptr(),
                frame.stride_u() as i32,
                frame.data_v().as_ptr(),
                frame.stride_v() as i32,
                buf.as_mut_ptr(),
                (width * 4) as i32,
                width as i32,
                height as i32,
            );
        }

        // Write the converted video frame into the frame buffer and
        // release the memory of the previous frame.
        let buf_ptr = Box::into_raw(Box::new(buf));
        let ret = self.buf.swap(buf_ptr, Ordering::Relaxed);
        if !ret.is_null() {
            drop(unsafe { Box::from_raw(ret) });
        }
    }
}

// Implementation of the video track sink.
impl librtc::SinkExt for VideoPlayer {
    type Item = Arc<VideoFrame>;

    // Triggered when a video frame is received.
    fn on_data(&mut self, frame: Arc<VideoFrame>) {
        self.on_frame(frame.as_ref());
    }
}

struct AudioPlayer {
    track: Arc<AudioTrack>,
}

impl AudioPlayer {
    fn new(track: Arc<AudioTrack>) -> Self {
        Self { track }
    }
}

// Implementation of the audio track sink.
impl librtc::SinkExt for AudioPlayer {
    type Item = Arc<AudioFrame>;

    // Triggered when an audio frame is received.
    fn on_data(&mut self, frame: Arc<AudioFrame>) {
        // Echo the audio frame to the peer's audio track.
        self.track.add_frame(frame.as_ref());
    }
}

// data channel sink implementation.
struct ChannelSinkImpl;

impl librtc::SinkExt for ChannelSinkImpl {
    type Item = Vec<u8>;

    // Triggered when the data channel receives data.
    fn on_data(&mut self, data: Vec<u8>) {
        println!("on channel data: {:?}", data);
    }
}

// peerconnection event handler, handle peerconnection event callback.
struct ObserverImpl {
    ws_writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    audio_track: MediaStreamTrack,
    handle: Handle,
}

impl ObserverExt for ObserverImpl {
    // peerconnection emits ice candidate event, when this event occurs, the
    // generated ice candidate should be sent to the peer.
    fn on_ice_candidate(&mut self, candidate: RTCIceCandidate) {
        let writer = self.ws_writer.clone();
        self.handle.spawn(async move {
            writer
                .lock()
                .await
                .send(Message::Text(
                    serde_json::to_string(&Payload::Candidate(candidate)).unwrap(),
                ))
                .await
                .unwrap();
        });
    }

    // This event is triggered when the peer creates a data channel.
    fn on_data_channel(&mut self, channel: RTCDataChannel) {
        self.handle.spawn(async move {
            // Register a data sink for this data channel.
            channel
                .register_sink(0, Sinker::new(ChannelSinkImpl {}))
                .await;

            // Next, we will continue to use this container to prevent automatic
            // release. This is a bad implementation, and it will not be
            // implemented in a normal process, but for a simpler implementation
            // example, this bad method will be used here.
            let _ = ManuallyDrop::new(channel);
        });
    }

    // This event is triggered when the peer creates a video track or audio
    // track.
    fn on_track(&mut self, mut track: MediaStreamTrack) {
        let audio_track = self.audio_track.clone();

        // Register sinks for audio and video tracks.
        self.handle.spawn(async move {
            match &mut track {
                MediaStreamTrack::Video(track) => {
                    track
                        .register_sink(0, Sinker::new(VideoPlayer::new(track.label().to_string())))
                        .await;
                }
                MediaStreamTrack::Audio(track) => {
                    if let MediaStreamTrack::Audio(at) = audio_track {
                        track
                            .register_sink(0, Sinker::new(AudioPlayer::new(at.clone())))
                            .await;
                    }
                }
            }

            // Next, we will continue to use this container to prevent automatic
            // release. This is a bad implementation, and it will not be
            // implemented in a normal process, but for a simpler implementation
            // example, this bad method will be used here.
            let _ = ManuallyDrop::new(track);
        });
    }
}

// The command line parameters of the sample program, use clap to parse the
// parameters.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Signaling server address, default localhost.
    #[arg(long, default_value = "ws://localhost")]
    signaling: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Connect to the signaling server. Since signaling messages need to be
    // written from multiple places, the writing end is packaged as a
    // thread-safe type.
    let args = Args::parse();
    let (ws_stream, _) = connect_async(&args.signaling).await?;
    let (writer, mut reader) = ws_stream.split();
    let writer = Arc::new(Mutex::new(writer));

    // Create an audio track and video track as the output of echo.
    let stream = MediaStream::new("media_stream")?;
    let audio_track = MediaStreamTrack::create_audio_track("audio_track")?;

    // Create a peer connection with default configuration.
    let config = RTCConfiguration::default();
    let pc = RTCPeerConnection::new(
        &config,
        Observer::new(ObserverImpl {
            audio_track: audio_track.clone(),
            ws_writer: writer.clone(),
            handle: Handle::current(),
        }),
    )?;

    // Add the created audio track and video track to the peer connection.
    pc.add_track(audio_track, stream.clone()).await?;

    // Read messages from the websocket until unreadable or an error occurs.
    while let Some(Ok(msg)) = reader.next().await {
        // Only websocket messages of type text are accepted, because
        // signaling messages will only be of type text.
        if let Message::Text(msg) = msg {
            let ret = serde_json::from_str::<Payload>(&msg);
            match ret? {
                Payload::Offer(offer) => {
                    // Receive offer message, set it to peerconnection, and
                    // create answer.
                    pc.set_remote_description(&offer).await?;
                    let answer = pc.create_answer().await?;
                    pc.set_local_description(&answer).await?;

                    // Reply the created answer to the peer via websocket.
                    writer
                        .lock()
                        .await
                        .send(Message::Text(serde_json::to_string(&Payload::Answer(
                            answer,
                        ))?))
                        .await?;
                }
                Payload::Candidate(candidate) => {
                    // Processing the candidate message will be much simpler,
                    // just submit it to peerconnection.
                    pc.add_ice_candidate(&candidate)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
