use crate::{buffer::Buffer, tui, view::View};
use crossterm::event::{self, Event, KeyCode};

pub struct Editor {
    should_quit: bool,
    view: View,
    buffer: Buffer,
}

impl Editor {
    pub fn new(path: Option<&str>) -> Self {
        Self {
            should_quit: false,
            view: View::new(),
            buffer: Buffer::open(path.unwrap_or_default()).unwrap_or_default(),
        }
    }

    pub fn run(&mut self) {
        let mut terminal = tui::setup_terminal().expect("Failed to setup terminal");

        while !self.should_quit {
            terminal.draw(|frame| self.view.render(frame, &self.buffer)).unwrap();
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
