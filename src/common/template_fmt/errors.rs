use std::fmt::Display;

#[derive(Debug)]
pub enum Errors {
    Lexer(String),
    Parser(String),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lexer(err) => write!(f, "Template parsing: lexer error: {}", err),
            Self::Parser(err) => write!(f, "Template parsing: parser error: {}", err),
        }
    }
}
