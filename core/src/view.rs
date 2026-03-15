use crate::{buffer::Buffer, color, theme::Theme};
use ratatui::{prelude::*, widgets::{Block, Paragraph}};

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, frame: &mut Frame, buffer: &Buffer, theme: &Theme) {
        let content = buffer.lines.join("\n");
        let text_style = Style::default().fg(color::from_hex(&theme.text));
        let bg_style = Style::default().bg(color::from_hex(&theme.background));

        frame.render_widget(
            Paragraph::new(content).style(text_style).block(Block::default().style(bg_style)),
            frame.size(),
        );
    }
}
