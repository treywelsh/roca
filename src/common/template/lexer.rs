use std::str::Chars;

use crate::common::template::token::Tokens;

// lex file:
// https://github.com/OpenNebula/one/blob/master/src/parsers/expr_parser.l

pub struct Lexer<'a> {
    input: Chars<'a>,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut iter = input.chars();
        let first = iter.next();
        Lexer {
            input: iter,
            ch: first,
        }
    }

    fn read_char(&mut self) {
        self.ch = self.input.next();
    }

    fn skip_space(&mut self) {
        while let Some(ch) = self.ch {
            if !ch.is_whitespace() {
                break;
            }
            //println!("skip_space");
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Tokens {
        self.skip_space();

        //println!("match");

        if self.ch.is_none() {
            //println!("end of file");
            return Tokens::Eof;
        }

        match self.ch.unwrap() {
            '[' => {
                self.read_char();

                Tokens::LSBracket
            }
            ']' => {
                self.read_char();

                Tokens::RSBracket
            }
            '=' => {
                self.read_char();

                Tokens::Assign
            }
            '"' => {
                self.read_char();
                let mut str_value = String::new();
                while let Some(ch) = self.ch {
                    if ch == '"' {
                        break;
                    }

                    str_value.push(ch);
                    self.read_char();

                    // manage escaped double quotes in the string
                    if self.ch.is_some() && self.ch.unwrap() == '\\' {
                        str_value.push('\\');
                        self.read_char();
                        if let Some(ch) = self.ch {
                            // add escaping backslash
                            if ch == '\\' {
                                str_value.push('\\');
                                self.read_char();
                            }
                            // add escaped character
                            if self.ch.is_some() && self.ch.unwrap() == '\\' {
                                str_value.push('\\');
                                self.read_char();
                            }
                            if self.ch.is_some() && self.ch.unwrap() == '"' {
                                str_value.push('"');
                                self.read_char();
                            }
                            if self.ch.is_some() && self.ch.unwrap() == 'r' {
                                str_value.push('r');
                                self.read_char();
                            }
                            if self.ch.is_some() && self.ch.unwrap() == 'n' {
                                str_value.push('n');
                                self.read_char();
                            }
                        } else {
                            break;
                        }
                    }
                }
                self.read_char();

                Tokens::String(str_value)
            }
            ',' => {
                self.read_char();

                Tokens::Comma
            }
            '#' => {
                while self.ch.is_some() && self.ch.unwrap() != '\n' {
                    self.read_char()
                }

                Tokens::Comment
            }
            _ => {
                // identifier start with a letter
                if self.ch.is_some() && self.ch.unwrap().is_alphabetic() {
                    let mut word = String::new();
                    while let Some(ch) = self.ch {
                        if !ch.is_alphanumeric() && ch != '_' && ch != '-' && ch != '\'' {
                            break;
                        }

                        word.push(ch);
                        self.read_char();
                    }

                    return Tokens::Ident(word);
                }

                // read number
                if self.ch.is_some() && self.ch.unwrap().is_numeric() {
                    let mut number_str = String::new();
                    // TODO: check for '.' to accept float...
                    while let Some(ch) = self.ch {
                        if !ch.is_numeric() {
                            break;
                        }
                        number_str.push(ch);

                        self.read_char();
                    }

                    return Tokens::Number(number_str);
                }

                Tokens::Illegal(self.ch.unwrap())
            }
        }
    }
}

#[cfg(test)]
mod lexer_test {

    use crate::common::template::{lexer::Lexer, token::Tokens};

    #[test]
    fn generate_token_complex() {
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

        assert_eq!(lex.next_token(), Tokens::Ident("name".to_string()));
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(lex.next_token(), Tokens::Ident("toto".to_string()));
        assert_eq!(lex.next_token(), Tokens::Ident("p1".to_string()));
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(
            lex.next_token(),
            Tokens::String("it's a\\\\nfirst string".to_string())
        );
        assert_eq!(lex.next_token(), Tokens::Ident("p2".to_string()));
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(lex.next_token(), Tokens::Number("12".to_string()));
        assert_eq!(lex.next_token(), Tokens::Ident("SUNSTONE".to_string()));
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(lex.next_token(), Tokens::LSBracket);
        assert_eq!(lex.next_token(), Tokens::Ident("DEFAULT_VIEW".to_string()));
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(lex.next_token(), Tokens::String("cloud".to_string()));
        assert_eq!(lex.next_token(), Tokens::Comma);
        assert_eq!(
            lex.next_token(),
            Tokens::Ident("GROUP_ADMIN_DEFAULT_VIEW".to_string())
        );
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(lex.next_token(), Tokens::String("groupadmin".to_string()));
        assert_eq!(lex.next_token(), Tokens::Comma);
        assert_eq!(
            lex.next_token(),
            Tokens::Ident("group_admin_views".to_string())
        );
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(
            lex.next_token(),
            Tokens::String("\\\\\"12\\\\\"".to_string())
        );
        assert_eq!(lex.next_token(), Tokens::Comma);
        assert_eq!(lex.next_token(), Tokens::Ident("VIEWS".to_string()));
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(lex.next_token(), Tokens::Ident("cloud".to_string()));
        assert_eq!(lex.next_token(), Tokens::Comma);
        assert_eq!(lex.next_token(), Tokens::Ident("p3".to_string()));
        assert_eq!(lex.next_token(), Tokens::Assign);
        assert_eq!(lex.next_token(), Tokens::Ident("v3".to_string()));
        assert_eq!(lex.next_token(), Tokens::RSBracket);

        assert_eq!(lex.next_token(), Tokens::Eof);
    }
}
