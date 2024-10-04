use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use std::{io::Result, time::Duration};

use crate::frames::{custom::{chat_frame::ChatFrame, login_frame::LoginFrame, register_frame::RegisterFrame}, custom_frame::CustomFrame};

pub struct App {
    terminal: DefaultTerminal,
    exit: bool,
    app_state: AppState,
    pub socket: Option<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
}

impl App {

    pub fn new(terminal: DefaultTerminal) -> Self {
        Self {
            terminal,
            exit: false,
            app_state: AppState::Login(LoginFrame::new()),
            socket: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        while !self.exit {
            self.render_tui();
            self.handle_input().await;
        }
        Ok(())
    }
    
    fn render_tui(&mut self) {
        let _ = self.terminal.draw(|frame: &mut ratatui::Frame<'_>| {
            match self.app_state {
                AppState::Login(ref login_frame) => {
                    login_frame.render(frame);
                },
                AppState::Chat(ref chat_frame) => {
                    chat_frame.render(frame);
                }
                AppState::Register(ref register_frame) => {
                    register_frame.render(frame);
                },   
            }
        });
    }

    async fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => {
                        self.exit = true;
                    },
                    KeyCode::Char(c) => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => {
                                login_frame.input(c);
                            },
                            AppState::Chat(ref mut chat_frame) => {
                                chat_frame.input(c);
                            }
                            AppState::Register(ref mut register_frame) => {
                                register_frame.input(c);
                            },
                        }
                    },

                    KeyCode::Tab => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => login_frame.focus = !login_frame.focus,
                            AppState::Chat(ref mut chat_frame ) => chat_frame.change_focus(),
                            AppState::Register(ref mut register_frame) => register_frame.change_focus(),
                        }
                    },
                    KeyCode::Backspace => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => login_frame.backspace(),
                            AppState::Chat(ref mut chat_frame) => chat_frame.backspace(),
                            AppState::Register(ref mut register_frame) => register_frame.backspace(),
                        }
                    },
                    KeyCode::F(1) => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => login_frame.toggle_password_visibility(),
                            AppState::Register(ref mut register_frame) => register_frame.toggle_password_visibility(),
                            AppState::Chat(_) => {}
                        }
                    },
                    KeyCode::F(2) => {
                        match self.app_state {
                            AppState::Login(_) => {
                                self.app_state = AppState::Register(RegisterFrame::new());
                            },
                            AppState::Register(_) => {
                                self.app_state = AppState::Login(LoginFrame::new());
                            },
                            _ => {}
                        }
                    },
                    KeyCode::Enter => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => {
                                let mut login_frame = std::mem::take(login_frame);
                                match login_frame.submit(self).await {
                                    Ok(_) => {
                                        self.app_state = AppState::Chat(ChatFrame::new());
                                    },
                                    Err(err) => login_frame.error_message = Some(err),
                                };
                            },
                            AppState::Chat(ref mut chat_frame) => {
                                chat_frame.submit_message();
                            },
                            AppState::Register(ref mut register_frame) => {
                                let mut register_frame = std::mem::take(register_frame);
                                match register_frame.submit(self).await {
                                    Ok(_) => {
                                        self.app_state = AppState::Chat(ChatFrame::new());
                                    },
                                    Err(err) => register_frame.error_message = Some(err),
                                };
                            },
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn set_socket(&mut self, socket: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>) {
        self.socket = Some(socket);
    }
}

enum AppState {
    Login(LoginFrame),
    Register(RegisterFrame),
    Chat(ChatFrame),
    
}