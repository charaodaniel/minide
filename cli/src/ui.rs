use crate::explorer::Explorer;
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::Text, widgets::{Block, Borders, List, ListItem, Paragraph}, Frame};

pub fn draw(f: &mut Frame, explorer: &mut Explorer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    let header = Block::default()
        .title("MinIDE CLI")
        .borders(Borders::ALL);
    f.render_widget(header, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);

    let items: Vec<ListItem> = explorer
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let style = if i == explorer.selected {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(Text::from(file.as_str())).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Explorer").borders(Borders::ALL));
    f.render_widget(list, main_chunks[0]);

    let editor = Block::default().title("Editor").borders(Borders::ALL);
    f.render_widget(editor, main_chunks[1]);

    let status_bar = Paragraph::new("[CTRL+S Save] [CTRL+Q Sair] [ENTER Abrir]")
        .block(Block::default());
    f.render_widget(status_bar, chunks[2]);
}
