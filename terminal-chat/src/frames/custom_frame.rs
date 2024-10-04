use ratatui::Frame;

pub trait CustomFrame {
    fn render(&self, frame: &mut Frame);
}