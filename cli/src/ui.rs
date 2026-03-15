use crate::{explorer::Explorer, syntax};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, explorer: &mut Explorer) {
    f.render_widget(Block::default(), f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    let header = Block::default()
        .title("MinIDE CLI")
        .borders(Borders::ALL);
    f.render_widget(header, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);

    let items: Vec<ListItem> = explorer
        .get_files_for_display()
        .into_iter()
        .enumerate()
        .map(|(i, file)| {
            let style = if i == explorer.selected {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(Text::from(file)).style(style)
        })
        .collect();

    let list = List::new(items).block(Block::default().title("Explorer").borders(Borders::ALL));
    f.render_widget(list, main_chunks[0]);

    let editor_chunks = if explorer.editor.is_searching {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(main_chunks[1])
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0)].as_ref())
            .split(main_chunks[1])
    };
    
    let editor_panel = editor_chunks[0];
    let editor_block = Block::default().title("Editor").borders(Borders::ALL);

    let lines: Vec<Line> = explorer.editor.content.iter().enumerate().map(|(line_idx, line_str)| {
        if explorer.editor.search_term.is_empty() {
            let tokens = syntax::parse(line_str);
            let spans: Vec<Span> = tokens
                .into_iter()
                .map(|(token_type, text)| Span::styled(text.to_string(), token_type.to_style()))
                .collect();
            Line::from(spans)
        } else {
            let mut spans = Vec::new();
            let mut last_pos = 0;
            let search_term_len = explorer.editor.search_term.len();

            for (match_pos, _) in line_str.match_indices(&explorer.editor.search_term) {
                if match_pos > last_pos {
                    let pre_match_str = &line_str[last_pos..match_pos];
                    let tokens = syntax::parse(pre_match_str);
                    spans.extend(tokens.into_iter().map(|(tt, txt)| Span::styled(txt.to_string(), tt.to_style())));
                }

                let is_selected = explorer.editor.selected_search_result
                    .map_or(false, |selected_idx| {
                        explorer.editor.search_results.get(selected_idx)
                            .map_or(false, |&(r_line, r_col)| r_line == line_idx && r_col == match_pos)
                    });
                
                let style = if is_selected {
                    Style::default().fg(Color::Black).bg(Color::Cyan)
                } else {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                };

                let match_str = &line_str[match_pos..match_pos + search_term_len];
                spans.push(Span::styled(match_str.to_string(), style));
                
                last_pos = match_pos + search_term_len;
            }

            if last_pos < line_str.len() {
                let post_match_str = &line_str[last_pos..];
                let tokens = syntax::parse(post_match_str);
                spans.extend(tokens.into_iter().map(|(tt, txt)| Span::styled(txt.to_string(), tt.to_style())));
            }
            
            Line::from(spans)
        }
    }).collect();

    let editor_content = Paragraph::new(Text::from(lines)).block(editor_block);
    f.render_widget(editor_content, editor_panel);

    if explorer.editor.is_searching {
        let search_panel = editor_chunks[1];
        let search_text = format!("Search: {}", explorer.editor.search_term);
        let num_results = explorer.editor.search_results.len();
        let selected = explorer.editor.selected_search_result.map_or(0, |s| s + 1);
        let count_text = format!("  {}/{}", selected, num_results);

        let search_spans = vec![
            Span::raw(search_text),
            Span::raw(count_text)
        ];

        let search_paragraph = Paragraph::new(Line::from(search_spans))
            .block(Block::default().borders(Borders::ALL).title("Find"));
        f.render_widget(search_paragraph, search_panel);
    }

    if explorer.editor.is_searching {
        if let Some(search_panel) = editor_chunks.get(1) {
            f.set_cursor(
                search_panel.x + 1 + "Search: ".len() as u16 + explorer.editor.search_term.len() as u16,
                search_panel.y + 1,
            )
        }
    } else if explorer.editor.path.is_some() {
        f.set_cursor(
            editor_panel.x + 1 + explorer.editor.cursor_x as u16,
            editor_panel.y + 1 + explorer.editor.cursor_y as u16,
        )
    }

    let status_text = if explorer.editor.is_searching {
        "[ESC Cancel] [CTRL+N Next] [CTRL+P Prev]"
    } else {
        "[CTRL+F Find] [CTRL+S Save] [CTRL+Q Quit]"
    };
    let status_bar = Paragraph::new(status_text).block(Block::default());
    f.render_widget(status_bar, chunks[2]);
}
