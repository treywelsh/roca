use std::{iter::Peekable, str::Chars};

use crate::common::template_fmt::token::Tokens;

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut iter = input.chars().peekable();
        let first = match iter.next() {
            Some(c) => c,
            None => 0 as char,
        };
        Lexer {
            input: iter,
            ch: first,
        }
    }

    fn read_char(&mut self) {
        println!("read_char");

        self.ch = match self.input.next() {
            Some(c) => c,
            None => 0 as char,
        };
        println!("============== read {:?}", self.ch);
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_space(&mut self) {
        while self.ch.is_whitespace() {
            println!("skip_space");

            self.read_char();
        }
    }

    pub fn next(&mut self) -> Tokens {
        self.skip_space();

        println!("match");

        match self.ch {
            '[' => {
                self.read_char();
                return Tokens::LSBracket;
            }
            ']' => {
                self.read_char();
                return Tokens::RSBracket;
            }
            '=' => {
                self.read_char();
                return Tokens::Assign;
            }
            '\\' => {
                if let Some(e) = self.peek_char() {
                    if *e == '\\' {
                        self.read_char();
                        self.read_char();
                        return Tokens::Esc;
                    } else {
                        return Tokens::Illegal(self.ch);
                    }
                }
            }
            '"' => {
                self.read_char();
                let mut str_value = String::new();
                while self.ch != '"' {
                    str_value.push(self.ch);
                    self.read_char();

                    if self.ch == '\\' {
                        str_value.push(self.ch);
                        self.read_char();

                        if self.ch == '\\' {
                            str_value.push(self.ch);
                            self.read_char();
                        }
                        if self.ch == '"' {
                            str_value.push(self.ch);
                            self.read_char();
                        }
                    }
                }
                self.read_char();
                return Tokens::String(str_value);
            }
            '\n' => {
                println!("newline");
                self.read_char();
                return Tokens::Eol;
            }
            ',' => {
                self.read_char();
                return Tokens::Comma;
            }
            '#' => {
                while self.ch != '\n' {
                    self.read_char()
                }
                return Tokens::Comment;
            }
            '\0' => {
                println!("end of file");
                return Tokens::Eof;
            }
            _ => {
                // identifier start with a letter
                // TODO: add _, -, and whitespace for string value ....?
                if self.ch.is_alphabetic() {
                    let mut word = String::new();
                    while self.ch.is_alphanumeric() || self.ch == '_' || self.ch == '-' {
                        println!("read ident: {}", word);
                        word.push(self.ch);
                        self.read_char();
                    }
                    return Tokens::Ident(word);
                }

                // read number
                if self.ch.is_numeric() {
                    let mut number_str = String::new();
                    // TODO: check for '.' to accept float...
                    while self.ch.is_numeric() {
                        println!("read number: {}", number_str);
                        number_str.push(self.ch);
                        self.read_char();
                    }
                    return Tokens::Number(number_str);
                }

                return Tokens::Illegal(self.ch);
            }
        }

        Tokens::Illegal(self.ch)
    }
}

//impl TryFrom<&str> for Lexer {
//    type Error;
//
//    fn try_from(value: &str) -> Result<Self, Self::Error> {
//        todo!()
//    }
//}

#[cfg(test)]
mod lexer {

    use crate::common::template_fmt::{lex::Lexer, token::Tokens};

    #[test]
    fn generate_token() {
        let mut lex = Lexer::new(
            r#"
            name = toto
	p1 = "it's a\\nfirst string"

	p2=12
	SUNSTONE = [
    DEFAULT_VIEW = "cloud",
GROUP_ADMIN_DEFAULT_VIEW="groupadmin",
    group_admin_views="\\"12\\"", VIEWS=cloud,

	p3 = v3
]"#,
        );

        println!("lexing...");

        assert_eq!(lex.next(), Tokens::Ident("name".to_string()));
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(lex.next(), Tokens::Ident("toto".to_string()));
        assert_eq!(lex.next(), Tokens::Ident("p1".to_string()));
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(
            lex.next(),
            Tokens::String("it's a\\\\nfirst string".to_string())
        );
        assert_eq!(lex.next(), Tokens::Ident("p2".to_string()));
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(lex.next(), Tokens::Number("12".to_string()));
        assert_eq!(lex.next(), Tokens::Ident("SUNSTONE".to_string()));
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(lex.next(), Tokens::LSBracket);
        assert_eq!(lex.next(), Tokens::Ident("DEFAULT_VIEW".to_string()));
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(lex.next(), Tokens::String("cloud".to_string()));
        assert_eq!(lex.next(), Tokens::Comma);
        assert_eq!(
            lex.next(),
            Tokens::Ident("GROUP_ADMIN_DEFAULT_VIEW".to_string())
        );
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(lex.next(), Tokens::String("groupadmin".to_string()));
        assert_eq!(lex.next(), Tokens::Comma);
        assert_eq!(lex.next(), Tokens::Ident("group_admin_views".to_string()));
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(lex.next(), Tokens::String("\\\\\"12\\\\\"".to_string()));
        assert_eq!(lex.next(), Tokens::Comma);
        assert_eq!(lex.next(), Tokens::Ident("VIEWS".to_string()));
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(lex.next(), Tokens::Ident("cloud".to_string()));
        assert_eq!(lex.next(), Tokens::Comma);
        assert_eq!(lex.next(), Tokens::Ident("p3".to_string()));
        assert_eq!(lex.next(), Tokens::Assign);
        assert_eq!(lex.next(), Tokens::Ident("v3".to_string()));
        assert_eq!(lex.next(), Tokens::RSBracket);

        assert_eq!(lex.next(), Tokens::Eof);
    }
}
