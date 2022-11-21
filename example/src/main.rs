use batrachia::*;
use anyhow::Error;
use tokio::fs;
use tokio::time::{sleep, Duration};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde::*;
use std::io::SeekFrom;
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    id: String,
    r#type: String,
    sdp: Option<String>,
    sdpMLineIndex: Option<u8>,
    candidate: Option<String>,
    sdpMid: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (ws_stream, _) = connect_async("ws://localhost/server").await?;
    let (write, mut read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    let observer = Observer::new();
    let config = RTCConfiguration::default();
    let mut peer = RTCPeerConnection::new(&config, &observer)?;

    let stream = MediaStream::new("stream_id")?;
    let track = MediaStreamTrack::new("video_track", MediaStreamTrackKind::Video)?;
    peer.add_track(track.clone(), stream.clone());

    tokio::spawn(async move {
        let need_size = (1920 as f64 * 1080 as f64 * 1.5) as usize;
        let mut buf = vec![0u8; need_size];

        let mut fs = fs::File::open("E:/batrachia/target/test.yuv").await.unwrap();

        loop {
            if let Ok(size) = fs.read_exact(&mut buf).await {
                let frame = I420Frame::from_slice(1920, 1080, &buf[..size]);
                track.add_frame(frame.as_ref());
            } else {
                fs.seek(SeekFrom::Start(0)).await.unwrap();
            }

            sleep(Duration::from_millis(1000 / 24)).await;
        }
    });

    let write_1 = write.clone();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(msg) = msg {
                let payload: Payload = serde_json::from_str(&msg).unwrap();
                if payload.r#type == "offer" {
                    peer.set_remote_description(&RTCSessionDescription {
                        kind: RTCSessionDescriptionType::Offer,
                        sdp: payload.sdp.unwrap().clone(),
                    }).await.unwrap();

                    let answer = peer.create_answer().await.unwrap();
                    peer.set_local_description(&answer).await.unwrap();

                    write_1.lock().await.send(Message::Text(serde_json::to_string(&Payload {
                        id: "client".to_string(),
                        r#type: "answer".to_string(),
                        sdp: Some(answer.sdp.clone()),
                        sdpMLineIndex: None,
                        candidate: None,
                        sdpMid: None,
                    }).unwrap())).await.unwrap();
                } else
                if payload.r#type == "candidate" {
                    peer.add_ice_candidate(&RTCIceCandidate {
                        candidate: payload.candidate.unwrap().clone(),
                        sdp_mid: payload.sdpMid.unwrap().clone(),
                        sdp_mline_index: payload.sdpMLineIndex.unwrap(),
                    }).unwrap();
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(state) = observer.signaling_change.recv().await {
            println!("+++++++++++++++++++++++++++++++++++++++++++++++++++ signaling_change: {:?}", state);
        }
    });

    tokio::spawn(async move {
        while let Some(candidate) = observer.ice_candidate.recv().await {
            write.lock().await.send(Message::Text(serde_json::to_string(&Payload {
                id: "client".to_string(),
                r#type: "candidate".to_string(),
                sdp: None,
                sdpMLineIndex: Some(candidate.sdp_mline_index),
                candidate: Some(candidate.candidate),
                sdpMid: Some(candidate.sdp_mid),
            }).unwrap())).await.unwrap();
        }
    });

    let mut start = false;
    tokio::spawn(async move {
        while let Some(track) = observer.track.recv().await {
            let mut sink = track.get_sink();
            while let Ok(_frame) = sink.receiver.recv().await {
                if !start {
                    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++ on video frame");
                    start = true;
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(track) = observer.data_channel.recv().await {
            let mut sink = track.get_sink();
            while let Ok(data) = sink.receiver.recv().await {
                println!("+++++++++++++++++++++++++++++++++++++++++++++++++++ channel data: {:?}", data.as_slice());
            }
        }
    });

    RTCPeerConnection::run();
    Ok(())
}
