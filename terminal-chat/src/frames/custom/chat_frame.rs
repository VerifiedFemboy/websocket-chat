use ratatui::{layout::{self, Alignment}, style::{Color, Style}, widgets::{Block, Borders, Paragraph}, Frame};

use crate::frames::custom_frame::CustomFrame;

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

    pub fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
    }
    
    pub fn change_focus(&mut self) {
        self.focus = !self.focus;
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

        let messages = self.messages.iter().map(|msg| format!("> {}", msg)).collect::<Vec<String>>();
        let messages = messages.join("\n");
        let messages_paragraph = Paragraph::new(messages.as_str()).block(messages_block);

        let help_paragraph = Paragraph::new("Press 'Tab' to change focus | Press 'Enter' to submit message | Press 'Backspace' to delete last character | Press 'Esc' to exit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));

        frame.render_widget(messages_paragraph, layout[0]);
        frame.render_widget(input_paragraph, layout[1]);
        frame.render_widget(help_paragraph, layout[2]);
    }
}