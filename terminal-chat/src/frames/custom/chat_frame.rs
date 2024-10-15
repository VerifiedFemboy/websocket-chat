
use futures_util::{SinkExt, StreamExt};
use ratatui::{layout::{self, Alignment}, style::{Color, Style, Stylize}, widgets::{Block, Borders, Paragraph}, Frame};
use tokio_tungstenite::tungstenite::Message;

use crate::{app::App, frames::custom_frame::CustomFrame};

#[derive(Debug, Clone)]
pub struct ChatFrame {
    pub messages: Vec<String>,
    pub input: String,
    pub focus: bool,
}

impl ChatFrame {

    pub fn new() -> Self {
        Self {
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
        let username = app.username.clone();
        let input_message = self.input.clone();
    
        match self.submit_message_to_server(username, input_message, app).await {
            Ok(_) => {
                
            },
            Err(err) => {
                eprintln!("Error submitting message: {err}");
            },
        }
    }

    //TODO: Implement receive_message in a loop
    pub async fn receive_message(&mut self, app: &mut App) {
        if app.socket.is_none() {
            return;
        }

        while let Some(Ok(Message::Text(res))) = app.socket.as_mut().unwrap().next().await {
            let response: Vec<&str> = res.split(':').collect();
            if response.len() == 3 && response[0] == "msg" {
                self.messages.push(format!("{} > {}", response[1], response[2]));
            } else {
                eprintln!("Invalid response format");
            }
        }
    }

    pub fn change_focus(&mut self) {
        self.focus = !self.focus;
    }

    async fn submit_message_to_server(&self, username: String, message: String, app: &mut App) -> Result<String, String> {
        let socket = app.socket.as_mut().unwrap();
    
        match socket.send(Message::Text(format!("msg:{}:{}", username, message))).await {
            Ok(_) => {
                if let Some(Ok(Message::Text(res))) = socket.next().await {
                    let response: Vec<&str> = res.split(':').collect();
                    if response.len() == 3 && response[0] == "msg" {
                        return Ok("Message sent".to_string());
                    } else {
                        return Err("Invalid response format".to_string());
                    }
                }
            },
            Err(err) => return Err(format!("Failed to send message => {err}")),
        }
    
        Err("Failed to send message".to_string())
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
            messages: Vec::new(),
            input: String::new(),
            focus: false,
        }
    }
}