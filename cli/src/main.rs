use std::{fs, io};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};

struct App {
    files: Vec<String>,
    selected: usize,
    content: String,
    file_open: Option<String>,
    edit_mode: bool,
}

impl App {

    fn new() -> Self {

        let files = fs::read_dir(".")
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();

        Self {
            files,
            selected: 0,
            content: String::new(),
            file_open: None,
            edit_mode: false,
        }
    }

    fn load_selected(&mut self) {

        let name = &self.files[self.selected];

        if let Ok(text) = fs::read_to_string(name) {
            self.content = text;
            self.file_open = Some(name.clone());
        }

    }

    fn save(&self) {

        if let Some(file) = &self.file_open {
            let _ = fs::write(file, &self.content);
        }

    }

    fn content_with_lines(&self) -> String {

        self.content
            .lines()
            .enumerate()
            .map(|(i, l)| format!("{:4} | {}", i + 1, l))
            .collect::<Vec<_>>()
            .join("\n")

    }

}

fn main() -> Result<(), io::Error> {

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {

        terminal.draw(|f| {

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(70)
                ])
                .split(f.size());

            let items: Vec<ListItem> = app.files
                .iter()
                .enumerate()
                .map(|(i, f)| {

                    if i == app.selected {
                        ListItem::new(f.clone())
                            .style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(f.clone())
                    }

                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Explorer").borders(Borders::ALL));

            f.render_widget(list, layout[0]);

            let editor = Paragraph::new(app.content_with_lines())
                .block(
                    Block::default()
                        .title(match &app.file_open {
                            Some(f) => format!("Editor: {}", f),
                            None => "Editor".to_string()
                        })
                        .borders(Borders::ALL)
                );

            f.render_widget(editor, layout[1]);

        })?;

        if let Event::Key(key) = event::read()? {

            if app.edit_mode {

                match key.code {

                    KeyCode::Char(c) => {
                        app.content.push(c);
                    }

                    KeyCode::Enter => {
                        app.content.push('\n');
                    }

                    KeyCode::Backspace => {
                        app.content.pop();
                    }

                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.save();
                    }

                    KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.edit_mode = false;
                    }

                    _ => {}
                }

            } else {

                match key.code {

                    KeyCode::Down => {
                        if app.selected < app.files.len() - 1 {
                            app.selected += 1;
                            app.load_selected();
                        }
                    }

                    KeyCode::Up => {
                        if app.selected > 0 {
                            app.selected -= 1;
                            app.load_selected();
                        }
                    }

                    KeyCode::Enter => {
                        app.load_selected();
                    }

                    KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.edit_mode = true;
                    }

                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }

                    _ => {}
                }

            }

        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}