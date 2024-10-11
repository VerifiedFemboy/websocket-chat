use std::iter::repeat;

use futures_util::{SinkExt, StreamExt};
use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Stylize}, widgets::{Block, Borders, Paragraph}};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::{app::{App, AppState}, encrypion, frames::custom_frame::CustomFrame};

use super::chat_frame::ChatFrame;

pub struct RegisterFrame {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub error_message: Option<String>,
    pub password_visible: bool,
    focus: Focus,
}

impl RegisterFrame {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            error_message: None,
            password_visible: false,
            focus: Focus::Username,
        }
    }

    pub fn input(&mut self, c: char) {
        match self.focus {
            Focus::Username => {
                self.username.push(c);
            },
            Focus::Password => {
                self.password.push(c);
            },
            Focus::ConfirmPassword => {
                self.confirm_password.push(c);
            }
        }
    }

    pub fn change_focus(&mut self) {
        match self.focus {
            Focus::Username => {
                self.focus = Focus::Password;
            },
            Focus::Password => {
                self.focus = Focus::ConfirmPassword;
            },
            Focus::ConfirmPassword => {
                self.focus = Focus::Username;
            }
        }
    }

    pub fn backspace(&mut self) {
        match self.focus {
            Focus::Username => {
                self.username.pop();
            },
            Focus::Password => {
                self.password.pop();
            },
            Focus::ConfirmPassword => {
                self.confirm_password.pop();
            }
        }
    }

    pub fn toggle_password_visibility(&mut self) {
        self.password_visible = !self.password_visible;
    }

    fn password_match(&self) -> bool {
        self.password == self.confirm_password
    }

    pub async fn submit(&self, app: &mut App) -> std::result::Result<(), String> {
        if self.username.is_empty() || self.password.is_empty() {
            return Err("Username and Password cannot be empty".to_string());
        }

        if !self.password_match() {
            return Err("Passwords do not match".to_string());
        }

        let url = Url::parse("ws://127.0.0.1:8080").unwrap();
        let (mut socket, _) = match connect_async(url).await {
            Ok(result) => result,
            Err(e) => return Err(format!("Failed to connect: {}", e)),
        };

        if let Some(Ok(Message::Text(response))) = socket.next().await {
            if response != "connection:success" {
            return Err("Failed to establish connection".to_string());
            }
        } else {
            return Err("Failed to receive connection confirmation".to_string());
        }
        app.set_socket(socket);

        let encrypted_password = encrypion::encrypt_password(self.password.as_str());
        app.socket.as_mut().unwrap().send(Message::Text(format!("register:{}:{}", self.username, encrypted_password))).await.expect("Failed to send message");
        
        if let Some(Ok(Message::Text(response))) = app.socket.as_mut().unwrap().next().await {
            if response == "register:success" {
                let chat_frame = ChatFrame::new();
                chat_frame.clone().username = self.username.clone();
                app.change_state(AppState::Chat(chat_frame));
            } else {
            return Err("Failed to register user".to_string());
            }
        } else {
            return Err("Failed to receive message".to_string());
        }
        Ok(())
    }
}

impl CustomFrame for RegisterFrame {
    fn render(&self, frame: &mut ratatui::Frame) {
        let size = frame.area();
        let width = 50;
        let height = 10;
        let x = (size.width.saturating_sub(width)) / 2;
        let y = (size.height.saturating_sub(height)) / 2;
        let login_area = Rect::new(x, y, width, height);

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
            )
            .split(login_area);

            let login_text = Paragraph::new(self.username.as_str())
                .block(Block::default().borders(Borders::ALL)
                .fg(
            if let Focus::Username = self.focus {
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
            ).block(Block::default().borders(Borders::ALL).fg(
            if let Focus::Password = self.focus {
                Color::Magenta
            } else {
                Color::Reset
            }).title("Password"));


            
            let masked_confirm_password = repeat('*').take(self.confirm_password.len()).collect::<String>();
            let confirm_password_text = Paragraph::new(
                if self.password_visible {
                    self.confirm_password.as_str()
                } else {
                    masked_confirm_password.as_str()
                }
            ).block(Block::default().borders(Borders::ALL).fg(if let Focus::ConfirmPassword = self.focus {
                Color::Magenta
            } else {
                Color::Reset
            }).title("Confirm Password"));

            frame.render_widget(login_text, outer_layout[0]);
            frame.render_widget(password_text, outer_layout[1]);
            frame.render_widget(confirm_password_text, outer_layout[2]);

            let pass_visibility_info = Paragraph::new(
            if self.password_visible {
                "Password is Visible"
            } else {
                "Password is Hidden"
            })
            .alignment(Alignment::Center);
            frame.render_widget(pass_visibility_info, Rect::new(0, 0, size.width, 1));
        
            let help_text = Paragraph::new("Press Tab to switch fields, Enter to submit, F1 to toggle password visibility and F2 to switch to Login")
            .alignment(Alignment::Center);

            let bottom_y = size.height.saturating_sub(1); // Assuming the help text height is 3
            frame.render_widget(help_text, Rect::new(0, bottom_y, size.width, 3));
    
            if let Some(error_msg) = self.error_message.clone() {
                let error_text = Paragraph::new(error_msg)
                    .alignment(Alignment::Center)
                    .fg(Color::Red).bold();
                frame.render_widget(error_text, Rect::new(0, 1, size.width, 1));
            }
    }
}

impl Default for RegisterFrame {
    fn default() -> Self {
        Self::new()
    }
}

enum Focus {
    Username,
    Password,
    ConfirmPassword,
}