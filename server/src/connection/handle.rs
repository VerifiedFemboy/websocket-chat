use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::accept_async;
use tungstenite::Message;
use tokio::net::TcpStream;

use crate::user::user::User;

use super::database::Database;

pub async fn handle_connection(stream: TcpStream, database: &Database) {
    let ws_stream = accept_async(stream).await.expect("Failed to accept WebSocket connection");

    let (mut write, mut read) = ws_stream.split();


    match write.send(Message::Text("connection:success".to_string())).await {
        Ok(_) => {
            if let Some(Ok(Message::Text(credentials))) = read.next().await {
                let cloned_credentials = credentials.clone();
                let split = cloned_credentials.split(":");
                let cred = split.collect::<Vec<&str>>();

                if cred[0] == "register" {
                    let user = User::from_credentials(&credentials);
                    match User::insert_user(database, &user).await {
                        Ok(_) => {
                            write.send(Message::Text("register:success".to_string())).await.expect("Failed to send message");
                        },
                        Err(err) => {
                            write.send(Message::Text(format!("Failed to register user => {err}"))).await.expect("Failed to send message");
                        }
                    };
                } else if cred[0] == "login" {
                    let user = User::from_credentials(&credentials);
                    match User::select_user(database, &user).await {
                        Ok(user) => {
                            match user {
                                Some(_) => {
                                    write.send(Message::Text("login:success".to_string())).await.expect("Failed to send message");
                                },
                                None => {
                                    write.send(Message::Text("Invalid credentials".to_string())).await.expect("Failed to send message");
                                }
                            }
                        },
                        Err(err) => {
                            write.send(Message::Text(format!("Failed to login user => \n{err}"))).await.expect("Failed to send message");
                        }
                    };
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            return;
        }
    };

    while let Some(Ok(Message::Text(message))) = read.next().await {
        let split = message.split(":");
        let msg = split.collect::<Vec<&str>>();
        if msg[0] == "msg" {
            let username = msg[1];
            let message = msg[2];
            let response = format!("msg:{}:{}", username, message);
            println!("{}", response);
            write.send(Message::Text(response)).await.expect("Failed to send message");
        }
    }
}
