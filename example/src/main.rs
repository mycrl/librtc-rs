mod message;
mod window;

use clap::*;
use batrachia::*;
use message::Payload;
use futures_util::{
    stream::*,
    StreamExt,
    SinkExt,
};

use tokio_tungstenite::{
    tungstenite::protocol::Message,
    WebSocketStream,
    MaybeTlsStream,
    connect_async,
};

use std::{
    io::SeekFrom,
    sync::Arc,
};

use tokio::io::{
    AsyncReadExt,
    AsyncSeekExt,
};

use tokio::{
    net::TcpStream,
    sync::Mutex,
    fs,
};

use tokio::time::{
    sleep,
    Duration,
};

type WsWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

struct VideoSinkImpl {
    app: window::MyApp,
}

impl VideoSinkImpl {
    fn new() -> Sinker<Arc<VideoFrame>> {
        Sinker::new(Self {
            app: window::MyApp::new("WebRTC example"),
        })
    }
}

impl batrachia::SinkExt for VideoSinkImpl {
    type Item = Arc<VideoFrame>;

    fn on_data(&mut self, frame: Arc<VideoFrame>) {
        self.app.push_frame(&frame);
    }
}

struct AudioSinkImpl;

impl batrachia::SinkExt for AudioSinkImpl {
    type Item = Arc<AudioFrame>;

    fn on_data(&mut self, _value: Arc<AudioFrame>) {}
}

struct ChannelSinkImpl;

impl batrachia::SinkExt for ChannelSinkImpl {
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
            channel.register_sink(0, Sinker::new(ChannelSinkImpl {})).await;
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

// get video frame for video input file,
// and put frame to rtc video track.
async fn read_frame(
    mut fs: fs::File,
    track: MediaStreamTrack,
) -> anyhow::Result<()> {
    if let MediaStreamTrack::Video(track) = track {
        let need_size = (1920.0 * 1080.0 * 1.5) as usize;
        let mut buf = vec![0u8; need_size];

        loop {
            sleep(Duration::from_millis(1000 / 24)).await;
            match fs.read_exact(&mut buf).await {
                Err(_) => {
                    fs.seek(SeekFrom::Start(0)).await?;
                },
                Ok(size) => {
                    let frame = VideoFrame::new(1920, 1080, &buf[..size]);
                    track.add_frame(&frame);
                },
            }
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// signaling server uri
    #[arg(long, default_value = "ws://localhost")]
    signaling: String,

    /// video yuv frames source file path
    #[arg(long)]
    video_input: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let (ws_stream, _) = connect_async(args.signaling).await?;
    let video_input = fs::File::open(args.video_input).await?;

    let (write, read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    let observer = ObserverImpl::new(write.clone());
    let config = RTCConfiguration::default();
    let pc = RTCPeerConnection::new(&config, observer)?;

    let stream = MediaStream::new("stream_id")?;
    let track = MediaStreamTrack::create_video_track("video_track")?;

    pc.add_track(track.clone(), stream.clone()).await;

    tokio::spawn(read_frame(video_input, track));
    tokio::spawn(read_ws_message(pc.clone(), read, write));

    batrachia::run();
    Ok(())
}
