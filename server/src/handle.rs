use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;

pub async fn handle_connection(server: TcpListener) {
    while let Ok((stream, _)) = server.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");

            let (mut write, mut read) = ws_stream.split();

            while let Some(Ok(message)) = read.next().await {
                println!("Received a message: {:?}", message.clone().into_text().unwrap().as_str());
                if let Err(e) = write.send(message).await {
                    eprintln!("Error sending message: {}", e);
                    break;
                }
            }
        });
    }
}