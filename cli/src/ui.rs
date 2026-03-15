use ratatui::{layout::{Constraint, Direction, Layout}, Frame, widgets::{Block, Borders, Paragraph}};

pub fn draw(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    // Header
    let header = Block::default()
        .title("MinIDE CLI")
        .borders(Borders::ALL);
    f.render_widget(header, chunks[0]);

    // Main content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);

    // File Explorer
    let explorer = Block::default().title("Explorer").borders(Borders::ALL);
    f.render_widget(explorer, main_chunks[0]);

    // Editor
    let editor = Block::default().title("Editor").borders(Borders::ALL);
    f.render_widget(editor, main_chunks[1]);

    // Status bar
    let status_bar = Paragraph::new("[CTRL+S Save] [CTRL+Q Sair] [ENTER Abrir]")
        .block(Block::default());
    f.render_widget(status_bar, chunks[2]);
}
