use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy)]
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

pub fn parse(code: &str) -> Vec<(TokenType, &str)> {
    let mut tokens = Vec::new();
    let keywords = vec!["fn", "let", "if", "else", "for", "while", "match"];

    for line in code.lines() {
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                tokens.push((TokenType::Default, &line[tokens.iter().map(|(_, s)| s.len()).sum()..tokens.iter().map(|(_, s)| s.len()).sum() + 1]));
            } else if c == '"' {
                let start = tokens.iter().map(|(_, s)| s.len()).sum();
                let mut len = 1;
                while let Some(next_c) = chars.next() {
                    len += 1;
                    if next_c == '"' {
                        break;
                    }
                }
                tokens.push((TokenType::String, &line[start..start + len]));
            } else if c == '/' && chars.peek() == Some(&'/') {
                let start = tokens.iter().map(|(_, s)| s.len()).sum();
                tokens.push((TokenType::Comment, &line[start..]));
                break;
            } else if c.is_alphabetic() {
                let start = tokens.iter().map(|(_, s)| s.len()).sum();
                let mut len = 1;
                while let Some(next_c) = chars.peek() {
                    if !next_c.is_alphanumeric() {
                        break;
                    }
                    len += 1;
                    chars.next();
                }
                let word = &line[start..start + len];
                let token_type = if keywords.contains(&word) {
                    TokenType::Keyword
                } else {
                    TokenType::Default
                };
                tokens.push((token_type, word));
            } else {
                tokens.push((TokenType::Default, &line[tokens.iter().map(|(_, s)| s.len()).sum()..tokens.iter().map(|(_, s)| s.len()).sum() + 1]));
            }
        }
        tokens.push((TokenType::Default, "\n"));
    }

    tokens
}
