use clap::*;
use serde::*;
use librtc_rs::*;
use futures_util::{
    stream::*,
    SinkExt,
    StreamExt,
};

use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    MaybeTlsStream,
    WebSocketStream,
};

use std::io::Write;
use std::{
    io::SeekFrom,
    sync::Arc,
};
use tokio::io::{
    AsyncReadExt,
    AsyncSeekExt,
};
use tokio::{
    fs,
    net::TcpStream,
    sync::Mutex,
};
use tokio::time::{
    sleep,
    Duration,
};

type WsWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub kind: String,
    sdp: Option<String>,
    sdpMLineIndex: Option<u8>,
    candidate: Option<String>,
    sdpMid: Option<String>,
}

impl Payload {
    pub fn from_str(str: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(str)?)
    }

    pub fn to_string(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TryInto<RTCSessionDescription> for Payload {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RTCSessionDescription, Self::Error> {
        Ok(RTCSessionDescription {
            kind: RTCSessionDescriptionType::Offer,
            sdp: self.sdp.ok_or(anyhow::anyhow!(""))?.clone(),
        })
    }
}

impl TryInto<RTCIceCandidate> for Payload {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RTCIceCandidate, Self::Error> {
        Ok(RTCIceCandidate {
            sdp_mid: self.sdpMid.ok_or(anyhow::anyhow!(""))?.clone(),
            candidate: self.candidate.ok_or(anyhow::anyhow!(""))?.clone(),
            sdp_mline_index: self.sdpMLineIndex.ok_or(anyhow::anyhow!(""))?,
        })
    }
}

impl From<RTCSessionDescription> for Payload {
    fn from(value: RTCSessionDescription) -> Self {
        Self {
            sdpMid: None,
            candidate: None,
            sdpMLineIndex: None,
            sdp: Some(value.sdp.clone()),
            kind: match value.kind {
                RTCSessionDescriptionType::Answer => "answer".to_string(),
                RTCSessionDescriptionType::Offer => "offer".to_string(),
                RTCSessionDescriptionType::PrAnswer => "prAnswer".to_string(),
                RTCSessionDescriptionType::Rollback => "rollback".to_string(),
            },
        }
    }
}

impl From<RTCIceCandidate> for Payload {
    fn from(value: RTCIceCandidate) -> Self {
        Self {
            sdp: None,
            kind: "candidate".to_string(),
            sdpMLineIndex: Some(value.sdp_mline_index),
            candidate: Some(value.candidate),
            sdpMid: Some(value.sdp_mid),
        }
    }
}

struct VideoSinkImpl {
    ffplay: Option<std::process::Child>,
}

impl VideoSinkImpl {
    fn new() -> Sinker<Arc<VideoFrame>> {
        Sinker::new(Self {
            ffplay: None,
        })
    }
}

impl librtc_rs::SinkExt for VideoSinkImpl {
    type Item = Arc<VideoFrame>;

    fn on_data(&mut self, frame: Arc<VideoFrame>) {
        if self.ffplay.is_none() {
            let child = std::process::Command::new("ffplay")
                .arg("-video_size")
                .arg(&format!("{}x{}", frame.width(), frame.height()))
                .arg("-f")
                .arg("rawvideo")
                .arg("-pixel_format")
                .arg("yuv420p")
                .arg("-i")
                .arg("-")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap();
            let _ = self.ffplay.insert(child);
        }

        if let Some(ffplay) = &mut self.ffplay {
            if let Some(stdin) = &mut ffplay.stdin {
                stdin.write_all(frame.data_y()).unwrap();
                stdin.write_all(frame.data_u()).unwrap();
                stdin.write_all(frame.data_v()).unwrap();
            }
        }
    }
}

struct AudioSinkImpl;

impl librtc_rs::SinkExt for AudioSinkImpl {
    type Item = Arc<AudioFrame>;

    fn on_data(&mut self, _value: Arc<AudioFrame>) {}
}

struct ChannelSinkImpl;

impl librtc_rs::SinkExt for ChannelSinkImpl {
    type Item = Vec<u8>;

    fn on_data(&mut self, value: Vec<u8>) {
        println!("on channel data: {:?}", value);
    }
}

struct ObserverImpl {
    tracks: Arc<Mutex<Vec<MediaStreamTrack>>>,
    channels: Arc<Mutex<Vec<RTCDataChannel>>>,
    write: Arc<Mutex<WsWriter>>,
}

impl ObserverImpl {
    fn new(write: Arc<Mutex<WsWriter>>) -> Observer {
        Observer::new(Self {
            tracks: Arc::new(Mutex::new(Vec::with_capacity(10))),
            channels: Arc::new(Mutex::new(Vec::with_capacity(10))),
            write,
        })
    }
}

impl ObserverExt for ObserverImpl {
    fn on_connection_change(&mut self, state: PeerConnectionState) {
        println!("signaling_change: {:?}", state);
    }

    fn on_ice_candidate(&mut self, candidate: RTCIceCandidate) {
        let writer = self.write.clone();
        tokio::spawn(async move {
            writer
                .lock()
                .await
                .send(Message::Text(
                    Payload::from(candidate).to_string().unwrap(),
                ))
                .await
                .unwrap();
        });
    }

    fn on_data_channel(&mut self, channel: RTCDataChannel) {
        let channels = self.channels.clone();
        tokio::spawn(async move {
            channel
                .register_sink(0, Sinker::new(ChannelSinkImpl {}))
                .await;
            channels.lock().await.push(channel);
        });
    }

    fn on_track(&mut self, mut track: MediaStreamTrack) {
        let tracks = self.tracks.clone();
        tokio::spawn(async move {
            match &mut track {
                MediaStreamTrack::Video(track) => {
                    track.register_sink(0, VideoSinkImpl::new()).await;
                },
                MediaStreamTrack::Audio(track) => {
                    track.register_sink(0, Sinker::new(AudioSinkImpl {})).await;
                },
            }

            tracks.lock().await.push(track);
        });
    }
}

// get signaling data for websocket.
async fn read_ws_message(
    pc: Arc<RTCPeerConnection>,
    mut read: WsReader,
    write: Arc<Mutex<WsWriter>>,
) -> anyhow::Result<()> {
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(msg) = msg {
            let payload = Payload::from_str(&msg)?;

            if payload.kind == "offer" {
                pc.set_remote_description(&Payload::try_into(payload)?)
                    .await?;

                let answer = pc.create_answer().await?;
                pc.set_local_description(&answer).await?;

                write
                    .lock()
                    .await
                    .send(Message::Text(Payload::from(answer).to_string()?))
                    .await?;
            } else if payload.kind == "candidate" {
                pc.add_ice_candidate(&Payload::try_into(payload)?)?;
            }
        }
    }

    Ok(())
}

async fn read_video_frame(
    args: Args,
    mut fs: fs::File,
    track: MediaStreamTrack,
) -> anyhow::Result<()> {
    if let MediaStreamTrack::Video(track) = track {
        let need_size =
            (args.video_width as f64 * args.video_height as f64 * 1.5) as usize;
        let mut buf = vec![0u8; need_size];

        loop {
            sleep(Duration::from_millis(1000 / args.video_frame_rate as u64))
                .await;
            match fs.read_exact(&mut buf).await {
                Err(_) => {
                    fs.seek(SeekFrom::Start(0)).await?;
                },
                Ok(size) => {
                    track.add_frame(&VideoFrame::from_default_layout(
                        1920,
                        1080,
                        0,
                        &buf[..size],
                    ));
                },
            };
        }
    }

    Ok(())
}

async fn read_audio_frame(
    args: Args,
    mut fs: fs::File,
    track: MediaStreamTrack,
) -> anyhow::Result<()> {
    if let MediaStreamTrack::Audio(track) = track {
        let frames = args.audio_sample_rate / 100;
        let mut buf = vec![0u8; frames * 2];

        loop {
            sleep(Duration::from_millis(10)).await;
            match fs.read_exact(&mut buf).await {
                Err(_) => {
                    fs.seek(SeekFrom::Start(0)).await?;
                },
                Ok(size) => {
                    track.add_frame(&AudioFrame::new(
                        args.audio_sample_rate,
                        args.audio_channels,
                        frames,
                        0,
                        &buf[..size],
                    ));
                },
            };
        }
    }

    Ok(())
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "ws://localhost")]
    signaling: String,

    #[arg(long)]
    video_input: String,
    #[arg(long)]
    video_width: usize,
    #[arg(long)]
    video_height: usize,
    #[arg(long)]
    video_frame_rate: usize,

    #[arg(long)]
    audio_input: String,
    #[arg(long)]
    audio_sample_rate: usize,
    #[arg(long)]
    audio_channels: u8,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let (ws_stream, _) = connect_async(&args.signaling).await?;
    let video_input = fs::File::open(&args.video_input).await?;
    let audio_input = fs::File::open(&args.audio_input).await?;

    let (write, read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    let observer = ObserverImpl::new(write.clone());
    let config = RTCConfiguration::default();
    let pc = RTCPeerConnection::new(&config, observer)?;

    let stream = MediaStream::new("stream_id")?;
    let video_track = MediaStreamTrack::create_video_track("video_track")?;
    let audio_track = MediaStreamTrack::create_audio_track("audio_track")?;

    pc.add_track(video_track.clone(), stream.clone()).await;
    pc.add_track(audio_track.clone(), stream.clone()).await;

    tokio::spawn(read_video_frame(args.clone(), video_input, video_track));
    tokio::spawn(read_audio_frame(args.clone(), audio_input, audio_track));
    tokio::spawn(read_ws_message(pc.clone(), read, write));

    librtc_rs::run();
    Ok(())
}
