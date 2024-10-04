use std::iter::repeat;

use crossterm::style::Color;
use futures_util::SinkExt;
use ratatui::{layout::{self, Alignment, Constraint, Layout, Rect}, style::Stylize, widgets::{Block, Borders, Paragraph}, Frame};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::app::App;

pub struct LoginFrame {
    pub username: String,
    pub password: String,
    pub focus: bool,
    pub password_visible: bool,
    pub error_message: Option<String>,
}

impl LoginFrame {

    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            focus: true,
            password_visible: false,
            error_message: None,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let size = frame.area();
            let width = 50;
            let height = 6;
            let x = (size.width.saturating_sub(width)) / 2;
            let y = (size.height.saturating_sub(height)) / 2;
            let login_area = Rect::new(x, y, width, height);

            let outer_layout = Layout::default()
                .direction(layout::Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(5),
                        Constraint::Min(5),
                    ]
                    .as_ref(),
                )
                .split(login_area);

            let login_text = Paragraph::new(self.username.as_str())
                .block(Block::default().borders(Borders::ALL)
                .fg(
                    if self.focus {
                        Color::Magenta
                    } else {
                        Color::Reset
                    }
                ).title("Username"));
            
            let masked_password = repeat('*').take(self.password.len()).collect::<String>();
            let password_text = Paragraph::new(
                if self.password_visible {
                    self.password.as_str()
                } else {
                    masked_password.as_str()
                }
            ).block(Block::default().borders(Borders::ALL).fg(if !self.focus {
                Color::Magenta
            } else {
                Color::Reset
            }).title("Password"));

            frame.render_widget(login_text, outer_layout[0]);
            frame.render_widget(password_text, outer_layout[1]);

            let pass_visibility_info = Paragraph::new(
            if self.password_visible {
                "Password is Visible"
            } else {
                "Password is Hidden"
            })
            .alignment(Alignment::Center);
            frame.render_widget(pass_visibility_info, Rect::new(0, 0, size.width, 1));
            

            if let Some(error_msg) = self.error_message.clone() {
                let error_text = Paragraph::new(error_msg)
                    .alignment(Alignment::Center)
                    .fg(Color::Red);
                frame.render_widget(error_text, Rect::new(0, 1, size.width, 1));
            }

            let help_text = Paragraph::new("Press Tab to switch fields, Enter to submit, and F1 to toggle password visibility")
            .alignment(Alignment::Center);

            let bottom_y = size.height.saturating_sub(1); // Assuming the help text height is 3
            frame.render_widget(help_text, Rect::new(0, bottom_y, size.width, 3));
    }

    pub fn input(&mut self, c: char) {
        if self.focus {
            self.username.push(c);
        } else {
            self.password.push(c);
        }
    }

    pub fn backspace(&mut self) {
        if self.focus {
            self.username.pop();
        } else {
            self.password.pop();
        }
    }

    pub async fn submit(&self, app: &mut App) -> std::result::Result<(), String> {
        let url = Url::parse("ws://127.0.0.1:8080").unwrap();
        let (socket, _) = connect_async(url).await.expect("Failed to connect to server");
        app.set_socket(socket);

        app.socket.as_mut().unwrap().send(Message::Text(format!("login:{}:{}", self.username, self.password))).await.expect("Failed to send message");
        Ok(())
    }

    pub fn toggle_password_visibility(&mut self) {
        self.password_visible = !self.password_visible;
    }
}

impl Default for LoginFrame {
    fn default() -> Self {
        Self::new()
    }
    
}