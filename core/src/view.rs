use crate::{buffer::Buffer, color, icons::Icons, theme::Theme};
use ratatui::{prelude::*, widgets::{Block, Paragraph}};
use std::path::Path;

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, frame: &mut Frame, buffer: &Buffer, theme: &Theme, icons: &Icons) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(frame.size());

        let content = buffer.lines.join("\n");
        let text_style = Style::default().fg(color::from_hex(&theme.text));
        let bg_style = Style::default().bg(color::from_hex(&theme.background));

        frame.render_widget(
            Paragraph::new(content).style(text_style).block(Block::default().style(bg_style)),
            layout[0],
        );

        let path = buffer.path.as_deref().unwrap_or(Path::new("untitled"));

        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        
        let icon = icons.extensions.get(extension).unwrap_or(&icons.file);

        let status_line_style = Style::default().fg(color::from_hex(&theme.text)).bg(color::from_hex(&theme.background));

        let status_line = Paragraph::new(format!("{} {}", icon, path.display()))
            .style(status_line_style);
        
        frame.render_widget(status_line, layout[1]);
    }
}
