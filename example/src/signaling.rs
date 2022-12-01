use tokio::net::*;
use futures_util::{SinkExt, StreamExt};
use anyhow::Result;
use tokio::sync::*;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

// start signaling websocket server.
pub async fn run(
    addr: String,
    reader: broadcast::Receiver<String>,
    writer: mpsc::UnboundedSender<String>,
) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(accept_socket(
            socket,
            reader.resubscribe(),
            writer.clone(),
        ));
    }

    Ok(())
}

// accept tcp socket to websocket.
async fn accept_socket(
    socket: TcpStream,
    mut reader: broadcast::Receiver<String>,
    writer: mpsc::UnboundedSender<String>,
) -> Result<()> {
    let stream = accept_async(socket).await?;
    let (mut ws_sender, mut ws_receiver) = stream.split();

    tokio::spawn(async move {
        while let Ok(payload) = reader.recv().await {
            if let Err(_) = ws_sender.send(Message::Text(payload)).await {
                break;
            }
        }
    });

    while let Some(Ok(Message::Text(payload))) = ws_receiver.next().await {
        writer.send(payload).unwrap();
    }

    Ok(())
}
