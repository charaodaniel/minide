use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Keyword,
    String,
    Comment,
    Default,
}

impl TokenType {
    pub fn to_style(self) -> Style {
        match self {
            TokenType::Keyword => Style::default().fg(Color::Magenta),
            TokenType::String => Style::default().fg(Color::Green),
            TokenType::Comment => Style::default().fg(Color::Gray),
            TokenType::Default => Style::default(),
        }
    }
}

pub fn parse(line: &str) -> Vec<(TokenType, &str)> {
    let mut tokens = Vec::new();
    let keywords = vec!["fn", "let", "if", "else", "for", "while", "match", "pub", "struct", "impl", "use", "mod", "self"];

    let mut current_pos = 0;
    while current_pos < line.len() {
        let remaining_line = &line[current_pos..];

        if remaining_line.starts_with("//") {
            tokens.push((TokenType::Comment, remaining_line));
            break; // The rest of the line is a comment
        }

        if remaining_line.starts_with('"') {
            let end_quote = remaining_line[1..].find('"').map(|i| i + 2).unwrap_or(remaining_line.len());
            tokens.push((TokenType::String, &remaining_line[..end_quote]));
            current_pos += end_quote;
            continue;
        }

        if let Some(first_char) = remaining_line.chars().next() {
            if first_char.is_alphabetic() {
                let end = remaining_line.find(|c: char| !c.is_alphanumeric()).unwrap_or(remaining_line.len());
                let word = &remaining_line[..end];
                let token_type = if keywords.contains(&word) {
                    TokenType::Keyword
                } else {
                    TokenType::Default
                };
                tokens.push((token_type, word));
                current_pos += end;
            } else if first_char.is_whitespace() {
                let end = remaining_line.find(|c: char| !c.is_whitespace()).unwrap_or(remaining_line.len());
                tokens.push((TokenType::Default, &remaining_line[..end]));
                current_pos += end;
            } else {
                let char_len = first_char.len_utf8();
                tokens.push((TokenType::Default, &remaining_line[..char_len]));
                current_pos += char_len;
            }
        } else {
            break; // End of line
        }
    }
    tokens
}
