mod message;
mod signaling;

use batrachia::*;
use clap::*;
use message::Payload;
use std::io::SeekFrom;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::fs;
use tokio::sync::broadcast::*;
use tokio::time::{sleep, Duration};

// rtc signaling state change.
async fn signaling_change(observer: Arc<Observer>) {
    while let Some(state) = observer.signaling_change.recv().await {
        println!("signaling_change: {:?}", state);
    }
}

// rtc ice candidate change.
async fn handle_ice_candidate(
    observer: Arc<Observer>,
    write: Sender<String>,
) -> anyhow::Result<()> {
    while let Some(candidate) = observer.ice_candidate.recv().await {
        write.send(Payload::from(candidate).to_string()?)?;
    }

    Ok(())
}

// rtc remote data channel.
async fn handle_data_channel(observer: Arc<Observer>) {
    while let Some(channel) = observer.data_channel.recv().await {
        let mut sink = channel.get_sink();
        while let Ok(data) = sink.receiver.recv().await {
            println!("channel data: {:?}", data.as_slice());
        }
    }
}

// rtc remote track.
async fn handle_track(observer: Arc<Observer>) {
    while let Some(track) = observer.track.recv().await {
        match track.as_ref() {
            MediaStreamTrack::Video(track) => {
                let mut sink = track.get_sink();
                while let Ok(_frame) = sink.receiver.recv().await {
                    println!("on video frame");
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
}

// get signaling data for websocket.
async fn read_ws_message(
    pc: Arc<RTCPeerConnection>,
    mut read: Receiver<String>,
    write: Sender<String>,
) -> anyhow::Result<()> {
    while let Ok(msg) = read.recv().await {
        let payload = Payload::from_str(&msg)?;

        if payload.r#type == "offer" {
            pc.set_remote_description(&Payload::try_into(payload)?)
                .await?;
            let answer = pc.create_answer().await?;
            pc.set_local_description(&answer).await?;
            write.send(Payload::from(answer).to_string()?)?;
        } else if payload.r#type == "candidate" {
            pc.add_ice_candidate(&Payload::try_into(payload)?)?;
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

    // fixed pixel size.
    let need_size = (1920 as f64 * 1080 as f64 * 1.5) as usize;
    let mut buf = vec![0u8; need_size];

    loop {
        sleep(Duration::from_millis(1000 / 24)).await;
        match fs.read_exact(&mut buf).await {
            Err(_) => {
                // if read end for file, seek file start and reread.
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
    /// signaling server listen port
    #[arg(short, long, default_value = "localhost:80")]
    bind: String,

    /// video yuv frames source file path
    #[arg(short, long)]
    video_input: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let frames_fs = fs::File::open(args.video_input).await?;

    let (writer, reader) = channel(10);
    tokio::spawn(signaling::run(
        args.bind,
        reader.resubscribe(),
        writer.clone(),
    ));

    let observer = Arc::new(Observer::new());
    let config = RTCConfiguration::default();
    let pc = RTCPeerConnection::new(&config, &observer)?;

    let stream = MediaStream::new("stream_id")?;
    let track = VideoTrack::new("video_track")?;

    pc.add_track(
        MediaStreamTrack::from_video(track.clone()), 
        stream.clone()
    ).await;

    tokio::spawn(read_frame(frames_fs, track));
    tokio::spawn(handle_track(observer.clone()));
    tokio::spawn(signaling_change(observer.clone()));
    tokio::spawn(handle_data_channel(observer.clone()));
    tokio::spawn(handle_ice_candidate(observer, writer.clone()));
    tokio::spawn(read_ws_message(pc.clone(), reader, writer));

    batrachia::run();
    Ok(())
}
