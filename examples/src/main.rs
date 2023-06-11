mod signaling;

use clap::*;
use librtc_rs::*;
use signaling::*;
use std::{mem::ManuallyDrop, sync::Arc};

use futures_util::{stream::*, SinkExt, StreamExt};

use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

use tokio::{net::TcpStream, sync::Mutex};

// Implementation of the video track sink.
struct VideoSinkImpl {
    video_track: Arc<VideoTrack>,
}

impl librtc_rs::SinkExt for VideoSinkImpl {
    type Item = Arc<VideoFrame>;

    // Triggered when a video frame is received.
    fn on_data(&mut self, frame: Arc<VideoFrame>) {
        // Echo the video frame to the peer's video track.
        self.video_track.add_frame(frame.as_ref());
    }
}

// Implementation of the audio track sink.
struct AudioSinkImpl {
    audio_track: Arc<AudioTrack>,
}

impl librtc_rs::SinkExt for AudioSinkImpl {
    type Item = Arc<AudioFrame>;

    // Triggered when an audio frame is received.
    fn on_data(&mut self, frame: Arc<AudioFrame>) {
        // Echo the audio frame to the peer's audio track.
        self.audio_track.add_frame(frame.as_ref());
    }
}

// data channel sink implementation.
struct ChannelSinkImpl;

impl librtc_rs::SinkExt for ChannelSinkImpl {
    type Item = Vec<u8>;

    // Triggered when the data channel receives data.
    fn on_data(&mut self, data: Vec<u8>) {
        println!("on channel data: {:?}", data);
    }
}

// peerconnection event handler, handle peerconnection event callback.
struct ObserverImpl {
    ws_writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    video_track: MediaStreamTrack,
    audio_track: MediaStreamTrack,
}

impl ObserverExt for ObserverImpl {
    // peerconnection emits ice candidate event, when this event occurs, the
    // generated ice candidate should be sent to the peer.
    fn on_ice_candidate(&mut self, candidate: RTCIceCandidate) {
        let writer = self.ws_writer.clone();
        tokio::spawn(async move {
            writer
                .lock()
                .await
                .send(Message::Text(Payload::from(candidate).to_string().unwrap()))
                .await
                .unwrap();
        });
    }

    // This event is triggered when the peer creates a data channel.
    fn on_data_channel(&mut self, channel: RTCDataChannel) {
        tokio::spawn(async move {
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
        let video_track = self.video_track.clone();
        let audio_track = self.audio_track.clone();

        // Register sinks for audio and video tracks.
        tokio::spawn(async move {
            match &mut track {
                MediaStreamTrack::Video(track) => {
                    if let MediaStreamTrack::Video(vt) = video_track {
                        track
                            .register_sink(
                                0,
                                Sinker::new(VideoSinkImpl {
                                    video_track: vt.clone(),
                                }),
                            )
                            .await;
                    }
                }
                MediaStreamTrack::Audio(track) => {
                    if let MediaStreamTrack::Audio(at) = audio_track {
                        track
                            .register_sink(
                                0,
                                Sinker::new(AudioSinkImpl {
                                    audio_track: at.clone(),
                                }),
                            )
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
    let video_track = MediaStreamTrack::create_video_track("video_track")?;
    let audio_track = MediaStreamTrack::create_audio_track("audio_track")?;

    // Create a peer connection with default configuration.
    let config = RTCConfiguration::default();
    let pc = RTCPeerConnection::new(
        &config,
        Observer::new(ObserverImpl {
            video_track: video_track.clone(),
            audio_track: audio_track.clone(),
            ws_writer: writer.clone(),
        }),
    )?;

    // Add the created audio track and video track to the peer connection.
    pc.add_track(video_track, stream.clone()).await;
    pc.add_track(audio_track, stream.clone()).await;

    tokio::spawn(async move {
        // Read messages from the websocket until unreadable or an error occurs.
        while let Some(Ok(msg)) = reader.next().await {
            // Only websocket messages of type text are accepted, because
            // signaling messages will only be of type text.
            if let Message::Text(msg) = msg {
                let payload = Payload::from_str(&msg)?;

                // Since the session initiator is not here, only offer and
                // candidate messages will be received.
                if payload.kind == "offer" {
                    // Receive offer message, set it to peerconnection, and
                    // create answer.
                    pc.set_remote_description(&Payload::try_into(payload)?)
                        .await?;
                    let answer = pc.create_answer().await?;
                    pc.set_local_description(&answer).await?;

                    // Reply the created answer to the peer via websocket.
                    writer
                        .lock()
                        .await
                        .send(Message::Text(Payload::from(answer).to_string()?))
                        .await?;
                } else if payload.kind == "candidate" {
                    // Processing the candidate message will be much simpler,
                    // just submit it to peerconnection.
                    pc.add_ice_candidate(&Payload::try_into(payload)?)?;
                }
            }
        }

        Ok::<(), anyhow::Error>(())
    });

    // Drive the webrtc main thread and block the current site until the webrtc
    // thread exits.
    librtc_rs::run().await;
    Ok(())
}
