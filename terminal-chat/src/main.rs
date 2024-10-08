use app::App;

mod app;
mod frames;
mod encrypion;

#[tokio::main]
async fn main() {
    let terminal = ratatui::init();
    let mut app = App::new(terminal);

    match app.run().await {
        Ok(_) => ratatui::restore(),
        Err(e) => eprintln!("Error: {}", e),
    }
}
