use ratatui::{prelude::*, widgets::Paragraph};

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, frame: &mut Frame) {
        frame.render_widget(Paragraph::new("Hello, world!"), frame.size());
    }
}
