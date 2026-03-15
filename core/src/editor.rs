use crate::{buffer::Buffer, theme::Theme, tui, view::View};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Editor {
    should_quit: bool,
    view: View,
    buffer: Buffer,
    theme: Theme,
}

impl Editor {
    pub fn new(path: Option<&str>) -> Self {
        Self {
            should_quit: false,
            view: View::new(),
            buffer: Buffer::open(path.unwrap_or_default()).unwrap_or_default(),
            theme: Theme::load("catppuccin-mocha").unwrap(),
        }
    }

    pub fn run(&mut self) {
        let mut terminal = tui::setup_terminal().expect("Failed to setup terminal");

        while !self.should_quit {
            terminal
                .draw(|frame| self.view.render(frame, &self.buffer, &self.theme))
                .unwrap();
            self.handle_events();
        }

        tui::restore_terminal(&mut terminal).expect("Failed to restore terminal");
    }

    fn handle_events(&mut self) {
        if let Ok(Event::Key(key)) = event::read() {
            let KeyEvent { code, modifiers, .. } = key;
            match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::NONE) => {
                    self.should_quit = true;
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    self.buffer.save().unwrap_or_default();
                }
                (KeyCode::Up, _) => {
                    // TODO: handle cursor movement
                }
                (KeyCode::Down, _) => {
                    // TODO: handle cursor movement
                }
                (KeyCode::Left, _) => {
                    // TODO: handle cursor movement
                }
                (KeyCode::Right, _) => {
                    // TODO: handle cursor movement
                }
                _ => {}
            }
        }
    }
}
