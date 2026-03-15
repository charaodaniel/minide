use crate::{explorer::Explorer, ui};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, stdout};

pub struct App {
    should_quit: bool,
    explorer: Explorer,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            explorer: Explorer::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        terminal::enable_raw_mode()?;
        execute!(stdout(), Clear(ClearType::All))?;

        while !self.should_quit {
            self.draw(&mut terminal)?;
            self.handle_events()?;
        }

        terminal::disable_raw_mode()?;
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Up => self.explorer.scroll_up(),
                KeyCode::Down => self.explorer.scroll_down(),
                KeyCode::Enter => self.explorer.open_selected(),
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            ui::draw(f, &mut self.explorer);
        })?;
        Ok(())
    }
}
