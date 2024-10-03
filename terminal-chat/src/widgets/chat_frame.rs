use ratatui::{widgets::{Block, Borders}, Frame};

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

    pub fn render(&self, frame: &mut Frame) {
        let chat_block = Block::default().borders(Borders::ALL).title("Chat");
        let chat_area = chat_block.inner(frame.area());
        frame.render_widget(chat_block, chat_area);

    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn input(&mut self, c: char) {
        self.input.push(c);
    }
    
}