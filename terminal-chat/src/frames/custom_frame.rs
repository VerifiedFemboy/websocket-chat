use ratatui::Frame;

pub struct CustomFrame;


pub trait CustomFrameTrait {
    fn render(&self, frame: &mut Frame);
}