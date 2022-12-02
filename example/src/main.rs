mod message;

use batrachia::*;
use futures_util::stream::*;
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;
use futures_util::{SinkExt, StreamExt};

use clap::*;
use message::Payload;
use tokio_tungstenite::WebSocketStream;
use std::io::SeekFrom;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::fs;

use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

type WsWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

// rtc signaling state change.
async fn signaling_change(observer: Arc<Observer>) {
    while let Some(state) = observer.signaling_change.clone().recv().await {
        println!("signaling_change: {:?}", state);
    }
}

// rtc ice candidate change.
async fn handle_ice_candidate(
    observer: Arc<Observer>,
    write: Arc<Mutex<WsWriter>>,
) -> anyhow::Result<()> {
    while let Some(candidate) = observer.ice_candidate.clone().recv().await {
        write
            .lock()
            .await
            .send(Message::Text(Payload::from(candidate).to_string()?))
            .await?;
    }

    Ok(())
}

// rtc remote data channel.
async fn handle_data_channel(observer: Arc<Observer>) {
    while let Some(channel) = observer.data_channel.clone().recv().await {
        let mut sink = channel.get_sink();
        while let Ok(data) = sink.receiver.recv().await {
            println!("channel data: {:?}", data.as_slice());
        }
    }
}

// rtc remote track.
async fn handle_track(
    mut fs: fs::File, 
    observer: Arc<Observer>
) -> anyhow::Result<()> {
    while let Some(track) = observer.track.clone().recv().await {
        match track.as_ref() {
            MediaStreamTrack::Video(track) => {
                let mut sink = track.get_sink();
                while let Ok(frame) = sink.receiver.recv().await {
                    fs.write(frame.data_y()).await?;
                    fs.write(frame.data_u()).await?;
                    fs.write(frame.data_v()).await?;
                    fs.flush().await?;
                }
            },
            MediaStreamTrack::Audio(track) => {
                let mut sink = track.get_sink();
                while let Ok(_frame) = sink.receiver.recv().await {
                    println!("on audio frame");
                }
            },
        }
    }
    
    Ok(())
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
    track: Arc<VideoTrack>,
) -> anyhow::Result<()> {
    let need_size = (1920 as f64 * 1080 as f64 * 1.5) as usize;
    let mut buf = vec![0u8; need_size];
    loop {
        sleep(Duration::from_millis(1000 / 24)).await;
        match fs.read_exact(&mut buf).await {
            Err(_) => {
                fs.seek(SeekFrom::Start(0)).await?;
            },
            Ok(size) => track
                .add_frame(I420Frame::new(1920, 1080, &buf[..size]).as_ref()),
        }
    }
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
    
    /// video yuv frames output file path
    #[arg(long)]
    video_output: String
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let (ws_stream, _) = connect_async(args.signaling).await?;
    let video_input = fs::File::open(args.video_input).await?;
    let video_output = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(args.video_output)
        .await?;
    
    let observer = Arc::new(Observer::new());
    let config = RTCConfiguration::default();
    let pc = RTCPeerConnection::new(&config, &observer)?;

    let stream = MediaStream::new("stream_id")?;
    let track = VideoTrack::new("video_track")?;

    let (write, read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    pc.add_track(MediaStreamTrack::from_video(track.clone()), stream.clone())
        .await;

    tokio::spawn(read_frame(video_input, track));
    tokio::spawn(handle_track(video_output, observer.clone()));
    tokio::spawn(signaling_change(observer.clone()));
    tokio::spawn(handle_data_channel(observer.clone()));
    tokio::spawn(handle_ice_candidate(observer, write.clone()));
    tokio::spawn(read_ws_message(pc.clone(), read, write));

    batrachia::run();
    Ok(())
}
