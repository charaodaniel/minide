use std::{fs, io};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    layout::{Layout, Constraint, Direction},
};

struct App {
    files: Vec<String>,
    selected: usize,
    content: String,
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
            content: String::from("Select a file to open"),
        }
    }

    fn open_file(&mut self) {
        let name = &self.files[self.selected];
        if let Ok(text) = fs::read_to_string(name) {
            self.content = text;
        }
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
                .map(|f| ListItem::new(f.clone()))
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Explorer").borders(Borders::ALL))
                .highlight_symbol("➜ ");

            f.render_stateful_widget(
                list,
                layout[0],
                &mut ratatui::widgets::ListState::default()
            );

            let editor = Paragraph::new(app.content.clone())
                .block(Block::default().title("Editor").borders(Borders::ALL));

            f.render_widget(editor, layout[1]);

        })?;

        if let Event::Key(key) = event::read()? {

            match key.code {

                KeyCode::Char('q') => break,

                KeyCode::Down => {
                    if app.selected < app.files.len() - 1 {
                        app.selected += 1;
                    }
                }

                KeyCode::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }

                KeyCode::Enter => {
                    app.open_file();
                }

                _ => {}
            }

        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}