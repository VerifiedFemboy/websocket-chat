use app::App;


mod app;

#[tokio::main]
async fn main() {
    let terminal = ratatui::init();
    let mut app = App::new(terminal);

    match app.run().await {
        Ok(_) => ratatui::restore(),
        Err(e) => eprintln!("Error: {}", e),
    }
}
