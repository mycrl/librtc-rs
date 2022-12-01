use tokio::net::*;
use futures_util::*;
use anyhow::Result;
use tokio_tungstenite::*;
use tokio::sync::broadcast::*;
use tokio_tungstenite::tungstenite::protocol::Message;

// start signaling websocket server.
pub async fn run(
    addr: String,
    reader: Receiver<String>,
    writer: Sender<String>,
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
    mut reader: Receiver<String>,
    writer: Sender<String>,
) -> Result<()> {
    let mut stream = accept_async(socket).await?;
    while let Some(Ok(Message::Text(payload))) = stream.next().await {
        writer.send(payload)?;
    }

    tokio::spawn(async move {
        while let Ok(payload) = reader.recv().await {
            stream.send(Message::Text(payload)).await.unwrap();
        }
    });

    Ok(())
}
