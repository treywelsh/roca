use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub enum Tokens {
    Illegal(char),
    //Eol,
    Eof,

    Ident(String),
    String(String), // should this be a token, or
    Number(String),
    Comment,

    Assign,
    Comma,
    Quote,
    //DQuote,    // use this, or use String token
    //Esc(char), // NOT USED OUTSIDE OF STRING ?
    LSBracket,
    RSBracket,
}

impl Debug for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Illegal(c) => write!(f, "Illegal char: {}", c),
            //Self::Eol => write!(f, "Eol"),
            Self::Eof => write!(f, "Eof"),
            Self::Ident(i) => f.debug_tuple("Ident").field(i).finish(),
            Self::String(v) => f.debug_tuple("String").field(v).finish(),
            Self::Number(v) => f.debug_tuple("Number").field(v).finish(),
            Self::Comment => write!(f, "Comment"),
            Self::Assign => write!(f, "Assign"),
            Self::Comma => write!(f, "Comma"),
            Self::Quote => write!(f, "Quote"),
            //Self::DQuote => write!(f, "DQuote"),
            //Self::Esc(c) => write!(f, "Esc: {}", c),
            Self::LSBracket => write!(f, "LSBracket"),
            Self::RSBracket => write!(f, "RSBracket"),
        }
    }
}
