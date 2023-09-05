use std::fmt::Debug;

#[derive(PartialEq)]
pub enum Tokens {
    Illegal(char),
    Eol,
    Eof,

    Ident(String),
    String(String),
    Number(String),
    Comment,

    Assign,
    Comma,
    Quote,
    DQuote,
    Esc,

    LSBracket,
    RSBracket,
}

impl Debug for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Illegal(c) => write!(f, "Illegal char: {}", c),
            Self::Eol => write!(f, "Eol"),
            Self::Eof => write!(f, "Eof"),
            Self::Ident(i) => f.debug_tuple("Ident").field(i).finish(),
            Self::String(v) => f.debug_tuple("String").field(v).finish(),
            Self::Number(v) => f.debug_tuple("Number").field(v).finish(),
            Self::Comment => write!(f, "Comment"),
            Self::Assign => write!(f, "Assign"),
            Self::Comma => write!(f, "Comma"),
            Self::Quote => write!(f, "Quote"),
            Self::DQuote => write!(f, "DQuote"),
            Self::Esc => write!(f, "Esc"),
            Self::LSBracket => write!(f, "LSBracket"),
            Self::RSBracket => write!(f, "RSBracket"),
        }
    }
}
