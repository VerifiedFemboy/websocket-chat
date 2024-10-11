
use futures_util::{SinkExt, StreamExt};
use ratatui::{layout::{self, Alignment}, style::{Color, Style, Stylize}, widgets::{Block, Borders, Paragraph}, Frame};
use tokio_tungstenite::tungstenite::Message;

use crate::{app::App, frames::custom_frame::CustomFrame};

#[derive(Debug, Clone)]
pub struct ChatFrame {
    pub username: String,
    pub messages: Vec<String>,
    pub input: String,
    pub focus: bool,
}

impl ChatFrame {

    pub fn new() -> Self {
        Self {
            username: String::new(),
            messages: Vec::new(),
            input: String::new(),
            focus: false,
        }
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn input(&mut self, c: char) {
        self.input.push(c);
    }

    pub async fn submit_message(&mut self, app: &mut App) {
        
        match self.submit_message_to_server(app.username.clone(), self.input.clone(), app).await {
            Some(msg) => {
                self.messages.push(msg);
                self.input.clear();
            },
            None => self.input = "Failed to send message".to_string(),
        }
    }
    
    pub fn change_focus(&mut self) {
        self.focus = !self.focus;
    }

    async fn submit_message_to_server(&self, username: String, message: String, app: &mut App) -> Option<String> {
        let socket = app.socket.as_mut().unwrap();
    
        match socket.send(Message::Text(format!("msg:{}:{}", username, message))).await {
            Ok(_) => {
                if let Some(Ok(Message::Text(res))) = socket.next().await {
                    let res_cloned = res.clone();
                    let split = res_cloned.split(":");
                    let response = split.collect::<Vec<&str>>();
                    if response[0] == "msg" {
                        let username = response[1];
                        let message = response[2];
        
                        let msg = format!("{username} > {message}");
                        return Some(msg);
                    }
                } 
            },
            Err(_) => return None,
        }
        
        None
    }
}

impl CustomFrame for ChatFrame {
    fn render(&self, frame: &mut Frame) {
        let layout = layout::Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints(
                [
                    layout::Constraint::Min(5),
                    layout::Constraint::Length(3),
                    layout::Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(frame.area());

        let messages_block = Block::default()
            .borders(Borders::ALL)
            .title("Messages")
            .border_style(Style::default().fg(if self.focus { Color::Magenta } else { Color::Reset }));

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title("Input")
            .border_style(if !self.focus { Color::Magenta } else { Color::Reset });

        let input_paragraph = Paragraph::new(self.input.as_str()).block(input_block);

        let messages = self.messages.iter().map(|msg| format!("{}", msg)).collect::<Vec<String>>();
        let messages = messages.join("\n");
        let messages_paragraph = Paragraph::new(messages.as_str()).block(messages_block).fg(Color::Yellow);

        let help_paragraph = Paragraph::new("Press 'Tab' to change focus | Press 'Enter' to submit message | Press 'Backspace' to delete last character | Press 'Esc' to exit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));

        frame.render_widget(messages_paragraph, layout[0]);
        frame.render_widget(input_paragraph, layout[1]);
        frame.render_widget(help_paragraph, layout[2]);
    }

}

impl Default for ChatFrame {
    fn default() -> Self {
        Self {
            username: String::new(),
            messages: Vec::new(),
            input: String::new(),
            focus: false,
        }
    }
}