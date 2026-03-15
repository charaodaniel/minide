use crate::{explorer::Explorer, ui};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEvent,
        MouseEventKind,
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
                if key.modifiers == KeyModifiers::CONTROL {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.should_quit = true;
                        }
                        KeyCode::Char('s') => {
                            if let Some(path) = &self.explorer.editor.path {
                                std::fs::write(path, self.explorer.editor.get_content_as_string()).ok();
                            }
                        }
                        KeyCode::Char('f') => {
                            self.explorer.editor.is_searching = true;
                        }
                        KeyCode::Char('n') => self.explorer.editor.next_result(),
                        KeyCode::Char('p') => self.explorer.editor.previous_result(),
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char(c) => self.explorer.editor.insert_char(c),
                        KeyCode::Backspace => self.explorer.editor.delete_char(),
                        KeyCode::Escape => self.explorer.editor.cancel_search(),
                        KeyCode::Up => {
                            if !self.explorer.editor.is_searching {
                                self.explorer.editor.move_cursor_up()
                            }
                        }
                        KeyCode::Down => {
                            if !self.explorer.editor.is_searching {
                                self.explorer.editor.move_cursor_down()
                            }
                        }
                        KeyCode::Left => {
                            if !self.explorer.editor.is_searching {
                                self.explorer.editor.move_cursor_left()
                            }
                        }
                        KeyCode::Right => {
                            if !self.explorer.editor.is_searching {
                                self.explorer.editor.move_cursor_right()
                            }
                        }
                        KeyCode::Enter => {
                            if !self.explorer.editor.is_searching {
                                self.explorer.open_selected()
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event),
            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_events(&mut self, mouse_event: MouseEvent) {
        if self.explorer.editor.is_searching {
            return;
        }
        match mouse_event.kind {
            MouseEventKind::ScrollUp => self.explorer.scroll_up(),
            MouseEventKind::ScrollDown => self.explorer.scroll_down(),
            MouseEventKind::Down(_) => {
                let click_row = mouse_event.row as usize;
                let click_col = mouse_event.column as usize;
                
                let chunks = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints([
                        ratatui::layout::Constraint::Percentage(25),
                        ratatui::layout::Constraint::Percentage(75),
                    ])
                    .split(ratatui::layout::Rect {
                        x: 0,
                        y: 3,
                        width: mouse_event.column + 1,
                        height: mouse_event.row + 1,
                    });
                let editor_panel = chunks[1];

                if click_col < editor_panel.x {
                    // Click is in the explorer
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
                } else {
                    // Click is in the editor
                    if self.explorer.editor.path.is_some() {
                        let y = (click_row as u16).saturating_sub(editor_panel.y) as usize;
                        let x = (click_col as u16).saturating_sub(editor_panel.x + 1) as usize;

                        self.explorer.editor.cursor_y = y.min(self.explorer.editor.content.len() - 1);
                        self.explorer.editor.cursor_x = x.min(self.explorer.editor.content[self.explorer.editor.cursor_y].len());
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
