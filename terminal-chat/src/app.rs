use ratatui::DefaultTerminal;
use std::io::Result;

pub struct App {
    terminal: DefaultTerminal,
}

impl App {

    pub fn new(terminal: DefaultTerminal) -> Self {
        Self {
            terminal,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            
        }
        Ok(())
    }
    
}