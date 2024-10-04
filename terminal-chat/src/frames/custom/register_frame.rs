use crate::frames::custom_frame::{CustomFrame, CustomFrameTrait};

pub struct RegisterFrame {
    pub focus: bool,
    pub username: String,
    pub password: String,
    pub error_message: Option<String>,
}

impl CustomFrameTrait for RegisterFrame {
    fn render(&self, frame: &mut ratatui::Frame) {
        todo!()
    }
}