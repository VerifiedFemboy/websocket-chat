use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use std::{io::Result, time::Duration};

use crate::{database::Database, widgets::login_frame::LoginFrame};

pub struct App {
    terminal: DefaultTerminal,
    exit: bool,
    app_state: AppState,
    database: Option<Database>,
}

impl App {

    pub fn new(terminal: DefaultTerminal) -> Self {
        Self {
            terminal,
            exit: false,
            app_state: AppState::Login(LoginFrame::new()),
            database: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        while !self.exit {
            self.render_tui();
            self.handle_input();
        }
        Ok(())
    }
    
    fn render_tui(&mut self) {
        let _ = self.terminal.draw(|frame: &mut ratatui::Frame<'_>| {
            match self.app_state {
                AppState::Login(ref login_frame) => {
                    login_frame.render(frame);
                },
                AppState::Chat => {

                }
                
            }
        });
    }

    fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        self.exit = true;
                    },
                    KeyCode::Char(c) => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => {
                                login_frame.input(c);
                            },
                            AppState::Chat => {

                            }
                        }
                    },

                    KeyCode::Tab => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => {
                                login_frame.focus = !login_frame.focus;
                            },
                            AppState::Chat => {

                            }
                        }
                    },
                    KeyCode::Backspace => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => {
                                login_frame.backspace();
                            },
                            AppState::Chat => {

                            }
                        }
                    },
                    KeyCode::F(1) => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => {
                                login_frame.toggle_password_visibility();
                            },
                            AppState::Chat => {

                            }
                        }
                    },
                    KeyCode::Enter => {
                        match self.app_state {
                            AppState::Login(ref mut login_frame) => {
                                login_frame.submit();
                            },
                            AppState::Chat => {

                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

enum AppState {
    Login(LoginFrame),
    Chat,
    
}