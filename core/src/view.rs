use crate::buffer::Buffer;
use ratatui::{prelude::*, widgets::Paragraph};

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, frame: &mut Frame, buffer: &Buffer) {
        let content = buffer.lines.join("\n");
        frame.render_widget(Paragraph::new(content), frame.size());
    }
}
