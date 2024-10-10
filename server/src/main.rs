use core::panic;

use connection::{database::Database, handle};
use tokio::net::TcpListener;

const ADRESS: &str = "127.0.0.1:8080";

mod connection;
mod user;

#[tokio::main]
async fn main() {
    
    let server = TcpListener::bind(ADRESS).await.unwrap();   
    let database = Database::new("mongodb://admin:password@localhost:27017".to_string());

    match database.connect().await {
        Ok(db) => {
            println!("Connected to database");
            while let Ok((socket, _)) = server.accept().await {
                let db = db.clone();
                tokio::spawn(async move {
                    handle::handle_connection(socket, &db).await;
                });
            }
        },
        Err(_) => {
            panic!("Database connection failed!");
        }   
    }
    
}