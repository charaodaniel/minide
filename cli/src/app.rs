use crate::{explorer::Explorer, ui};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEvent, MouseEventKind,
    },
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
        execute!(
            stdout(),
            Clear(ClearType::All),
            EnableMouseCapture
        )?;

        while !self.should_quit {
            self.draw(&mut terminal)?;
            self.handle_events()?;
        }

        terminal::disable_raw_mode()?;
        execute!(
            stdout(),
            Clear(ClearType::All),
            DisableMouseCapture
        )?;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Up => self.explorer.scroll_up(),
                    KeyCode::Down => self.explorer.scroll_down(),
                    KeyCode::Enter => self.explorer.open_selected(),
                    _ => {}
                }
            }
            Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event),
            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_events(&mut self, mouse_event: MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::ScrollUp => self.explorer.scroll_up(),
            MouseEventKind::ScrollDown => self.explorer.scroll_down(),
            MouseEventKind::Down(_) => {
                let click_row = mouse_event.row as usize;
                
                let list_start_row = 4;

                if click_row >= list_start_row {
                    let new_selection = click_row - list_start_row;
                    if new_selection < self.explorer.entries.len() {
                        if self.explorer.selected == new_selection {
                            self.explorer.open_selected();
                        } else {
                            self.explorer.selected = new_selection;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            ui::draw(f, &mut self.explorer);
        })?;
        Ok(())
    }
}
