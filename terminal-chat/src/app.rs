use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use std::{io::Result, time::Duration};

use crate::frames::{custom::{chat_frame::ChatFrame, login_frame::LoginFrame, register_frame::RegisterFrame}, custom_frame::CustomFrame};

pub struct App {
    terminal: DefaultTerminal,
    exit: bool,
    pub app_state: AppState,
    pub socket: Option<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    pub username: String,
}

impl App {

    pub fn new(terminal: DefaultTerminal) -> Self {
        Self {
            terminal,
            exit: false,
            app_state: AppState::Login(LoginFrame::new()),
            socket: None,
            username: String::new(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        while !self.exit {
            self.render_tui();
            self.handle_input().await;
            // self.connection(); How I could implement this to thread without cloning?!?!?! I give up
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
                                        let chat_frame = ChatFrame::new();
                                        self.username = login_frame.username;
                                        self.app_state = AppState::Chat(chat_frame);
                                    },
                                    Err(err) => {
                                        login_frame.error_message = Some(err);
                                        self.app_state = AppState::Login(login_frame);
                                    },
                                };
                            },
                            AppState::Chat(ref mut chat_frame) => {
                                let mut mem_chat = std::mem::take(chat_frame);
                                mem_chat.submit_message(self).await;
                                self.app_state = AppState::Chat(mem_chat);
                            },
                            AppState::Register(ref mut register_frame) => {
                                let mut register_frame = std::mem::take(register_frame);
                                match register_frame.submit(self).await {
                                    Ok(_) => {
                                        let chat_frame = ChatFrame::new();
                                        self.username = register_frame.username;
                                        self.app_state = AppState::Chat(chat_frame);
                                    },
                                    Err(err) => {
                                        register_frame.error_message = Some(err);
                                        self.app_state = AppState::Register(register_frame);
                                    },
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

    pub fn change_state(&mut self, state: AppState) {
        self.app_state = state;
    }

    async fn connection(&mut self) {
            match &mut self.app_state {
                AppState::Chat(ref mut chat_frame) => {
                    let mut mem_chat = std::mem::take(chat_frame);
                    mem_chat.receive_message(self).await;
                },
                _ => {}
            }
        }
}

pub enum AppState {
    Login(LoginFrame),
    Register(RegisterFrame),
    Chat(ChatFrame),
}