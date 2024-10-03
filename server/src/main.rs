use tokio::net::TcpListener;

const ADRESS: &str = "127.0.0.1:8080";

mod handle;
mod database;

#[tokio::main]
async fn main() {
    
    let server = TcpListener::bind(ADRESS).await.unwrap();   
    handle::handle_connection(server).await;
}