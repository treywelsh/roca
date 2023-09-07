use crate::common::template_elements::Vector;
use crate::common::template_fmt::errors::Errors;
use crate::common::template_fmt::lexer::Lexer;
use crate::common::template_fmt::token::Tokens;
use crate::prelude::TemplateBuilder;

// syntax: https://github.com/OpenNebula/one/blob/2eb07ee1c16140f1aca4d778b2cc2d38ea291159/src/parsers/template_syntax.y

struct Parser<'a> {
    lexer: Lexer<'a>,
    token: Tokens,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        Parser { lexer, token }
    }

    fn read_token(&mut self) {
        self.token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<TemplateBuilder, Errors> {
        let mut template = TemplateBuilder::new();
        loop {
            let key = match self.token.clone() {
                Tokens::Ident(i) => i,
                Tokens::Eof => break,
                _ => {
                    return Err(Errors::Parser(format!(
                        "unexpected token {:?}, expect ident",
                        self.token
                    )))
                }
            };
            self.read_token();

            match self.token {
                Tokens::Assign => {}
                _ => {
                    return Err(Errors::Parser(format!(
                        "unexpected token {:?}, expect assignment",
                        self.token
                    )))
                }
            }
            self.read_token();

            // determine if it's a pair or a vector
            match self.token.clone() {
                Tokens::Ident(value) | Tokens::String(value) | Tokens::Number(value) => {
                    // it's a pair

                    template.put_str(&key, value.as_str())
                }
                Tokens::LSBracket => {
                    // it's a vector

                    let mut vec = Vector::new(&key);

                    self.read_token();

                    // let's parse the pairs inside of the vector
                    while self.token != Tokens::RSBracket && self.token != Tokens::Eof {
                        let key = match self.token.clone() {
                            Tokens::Ident(i) => i,
                            _ => {
                                return Err(Errors::Parser(format!(
                                    "unexpected token {:?}, expect ident",
                                    self.token
                                )))
                            }
                        };
                        self.read_token();

                        match self.token {
                            Tokens::Assign => {}
                            _ => {
                                return Err(Errors::Parser(format!(
                                    "unexpected token {:?}, expect assignment",
                                    self.token
                                )))
                            }
                        }
                        self.read_token();

                        match self.token.clone() {
                            Tokens::Ident(value)
                            | Tokens::String(value)
                            | Tokens::Number(value) => {
                                // it's a pair inside of a vector

                                vec.add_pair(&key, &value)
                            }
                            _ => {
                                return Err(Errors::Parser(format!(
                                    "unexpected token {:?}, expect ident or string or number",
                                    self.token
                                )))
                            }
                        }
                        self.read_token();

                        match self.token {
                            Tokens::Comma => {
                                self.read_token();
                            }
                            Tokens::RSBracket => {}
                            _ => {
                                return Err(Errors::Parser(format!(
                                    "unexpected token {:?}, expect comma",
                                    self.token
                                )))
                            }
                        }
                    }
                    self.read_token();

                    //println!("VECTOR: {}", vec);

                    // it's a vector
                    template.put_vector(vec)
                }
                _ => {
                    println!("NOT HANDLED: {:?}", self.token.clone());

                    break;
                }
            }
            self.read_token();
        }

        Ok(template)
    }
}

#[cfg(test)]
mod parser_test {

    use crate::prelude::TemplateCommonGetters;

    use super::Parser;

    #[test]
    fn generate_template_complex() {
        let mut parser = Parser::new(
            r#"
            name = toto
	p1 = "it's a\\nfirst string"

	p2=12
	SUNSTONE = [
    DEFAULT_VIEW = "cloud",
GROUP_ADMIN_DEFAULT_VIEW="groupadmin",
    group_admin_views="\\"12\\"", VIEWS=cloud,

	p3 = v3
], p4 = v4"#,
        );

        let template = parser.parse();
        assert!(template.is_ok());
        let template = template.unwrap();
        println!("parse result: {}", template);
        let name = template.get_str("name");
        assert!(name.is_ok());
        assert_eq!(name.unwrap(), "toto");

        let p1 = template.get_str("p1");
        assert!(p1.is_ok());
        assert_eq!(p1.unwrap(), "it's a\\\\nfirst string");

        let p2 = template.get_str("p2");
        assert!(p2.is_ok());
        assert_eq!(p2.unwrap(), "12");

        let vec = template.get_vector("SUNSTONE");
        assert!(vec.is_ok());
        let vec = vec.unwrap();

        let default_view = vec.get_str("DEFAULT_VIEW");
        assert!(default_view.is_ok());
        assert_eq!(default_view.unwrap(), "cloud");

        let group_admin_default_view = vec.get_str("GROUP_ADMIN_DEFAULT_VIEW");
        assert!(group_admin_default_view.is_ok());
        assert_eq!(group_admin_default_view.unwrap(), "groupadmin");

        let group_admin_views = vec.get_str("group_admin_views");
        assert!(group_admin_views.is_ok());
        assert_eq!(group_admin_views.unwrap(), "\\\\\"12\\\\\"");

        let views = vec.get_str("VIEWS");
        assert!(views.is_ok());
        assert_eq!(views.unwrap(), "cloud");

        let p3 = vec.get_str("p3");
        assert!(p3.is_ok());
        assert_eq!(p3.unwrap(), "v3");

        let p4 = template.get_str("p4");
        assert!(p4.is_ok());
        assert_eq!(p4.unwrap(), "v4");
    }
}
