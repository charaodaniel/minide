use crate::{explorer::Explorer, ui};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEvent,
        MouseEventKind,
    },
    execute,
    terminal::{self, Clear, ClearType},
};
use ratatui::{backend::CrosstermBackend, layout::{Direction, Layout, Constraint, Rect}, Terminal};
use std::io::{self, stdout};

pub struct App {
    should_quit: bool,
    explorer: Explorer,
    size: Rect,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            explorer: Explorer::new(),
            size: Rect::default(),
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

        self.explorer.preview_selected(); // Initial preview

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
                            if self.explorer.editor.is_editing {
                                if let Some(path) = &self.explorer.editor.path {
                                    std::fs::write(path, self.explorer.editor.get_content_as_string()).ok();
                                }
                            }
                        }
                        KeyCode::Char('f') => {
                            if self.explorer.editor.path.is_some() {
                                self.explorer.editor.is_searching = true;
                            }
                        }
                        KeyCode::Char('n') => self.explorer.editor.next_result(),
                        KeyCode::Char('p') => self.explorer.editor.previous_result(),
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char(c) => self.explorer.editor.insert_char(c),
                        KeyCode::Backspace => self.explorer.editor.delete_char(),
                        KeyCode::Esc => {
                            if self.explorer.editor.is_searching {
                                self.explorer.editor.cancel_search();
                            } else if self.explorer.editor.is_editing {
                                self.explorer.editor.is_editing = false;
                            } else if self.explorer.editor.path.is_some() {
                                self.explorer.editor.close();
                            }
                        }
                        KeyCode::Up => {
                            if self.explorer.editor.is_editing {
                                self.explorer.editor.move_cursor_up();
                            } else {
                                self.explorer.select_previous();
                            }
                        }
                        KeyCode::Down => {
                            if self.explorer.editor.is_editing {
                                self.explorer.editor.move_cursor_down();
                            } else {
                                self.explorer.select_next();
                            }
                        }
                        KeyCode::Left => {
                            if self.explorer.editor.is_editing {
                                self.explorer.editor.move_cursor_left();
                            } else {
                                self.explorer.toggle_directory();
                            }
                        }
                        KeyCode::Right => {
                            if self.explorer.editor.is_editing {
                                self.explorer.editor.move_cursor_right();
                            } else {
                                self.explorer.toggle_directory();
                            }
                        }
                        KeyCode::Enter => {
                            if self.explorer.editor.is_editing {
                                self.explorer.editor.insert_newline();
                            } else {
                                self.explorer.open_selected();
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
                let click_row = mouse_event.row;
                let click_col = mouse_event.column;

                let app_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)].as_ref())
                    .split(self.size);

                let main_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                    .split(app_chunks[1]);

                let explorer_panel = main_chunks[0];
                let editor_panel = main_chunks[1];

                if click_col >= explorer_panel.x && click_col < explorer_panel.x + explorer_panel.width &&
                   click_row >= explorer_panel.y && click_row < explorer_panel.y + explorer_panel.height
                {
                    let list_start_row = explorer_panel.y + 1;
                    if click_row >= list_start_row {
                        let new_selection = (click_row - list_start_row) as usize;
                        if new_selection < self.explorer.flat_list.len() {
                            if self.explorer.selected_index == new_selection {
                                self.explorer.open_selected();
                            } else {
                                self.explorer.selected_index = new_selection;
                                self.explorer.preview_selected();
                            }
                        }
                    }
                } else if click_col >= editor_panel.x && click_col < editor_panel.x + editor_panel.width &&
                          click_row >= editor_panel.y && click_row < editor_panel.y + editor_panel.height
                {
                    if self.explorer.editor.path.is_some() {
                        self.explorer.editor.is_editing = true; // Start editing on click
                        let y = (click_row).saturating_sub(editor_panel.y + 1) as usize;
                        let x = (click_col).saturating_sub(editor_panel.x + 1) as usize;

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
            self.size = f.size();
            ui::draw(f, &mut self.explorer);
        })?;
        Ok(())
    }
}
