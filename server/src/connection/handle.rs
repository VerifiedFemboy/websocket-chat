use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::accept_async;
use tungstenite::Message;
use tokio::net::TcpStream;

use crate::user::user::User;

use super::database::Database;

pub async fn handle_connection(stream: TcpStream, database: &Database) {
    let ws_stream = accept_async(stream).await.expect("Failed to accept WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    if let Some(Ok(Message::Text(credentials))) = read.next().await {
        let cloned_credentials = credentials.clone();

        let split = cloned_credentials.split(":");
        let cred = split.collect::<Vec<&str>>();
        if cred[0] == "register" {
            let user = User::from_credentials(&credentials);
            match User::insert_user(database, &user).await {
                Ok(_) => {
                    write.send(Message::Text("User registered".to_string())).await.expect("Failed to send message");
                },
                Err(err) => {
                    write.send(Message::Text(format!("Failed to register user => \n{err}"))).await.expect("Failed to send message");
                }
            };
        } else if cred[0] == "login" {
            // Implement login
        }
    }
}