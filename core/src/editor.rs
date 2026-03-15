use crossterm::event::{self, Event, KeyCode};
use crate::{tui, view::View};

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            view: View::new(),
        }
    }

    pub fn run(&mut self) {
        let mut terminal = tui::setup_terminal().expect("Failed to setup terminal");

        while !self.should_quit {
            terminal.draw(|frame| self.view.render(frame)).unwrap();
            self.handle_events();
        }

        tui::restore_terminal(&mut terminal).expect("Failed to restore terminal");
    }

    fn handle_events(&mut self) {
        if let Ok(Event::Key(key)) = event::read() {
            if let KeyCode::Char('q') = key.code {
                self.should_quit = true;
            }
        }
    }
}
