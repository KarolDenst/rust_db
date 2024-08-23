use crate::lang::token::Token;
use std::collections::HashSet;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current: String,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            tokens: Vec::new(),
            current: String::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let breaks: HashSet<char> = HashSet::from([' ', '\t', '\r', '\n', '(', ')', ',']);
        for i in 0..self.source.len() {
            let c = self.source.chars().nth(i).unwrap();
            if breaks.contains(&c) {
                if self.current != "" {
                    self.process_string();
                    self.current.clear();
                }
                self.process_char(c);
                self.current.clear();
            } else {
                self.current.push(c);
            }
        }

        self.tokens.clone()
    }

    fn process_string(&mut self) {
        match self.current.to_lowercase().as_str() {
            "create" => self.tokens.push(Token::Create),
            "table" => self.tokens.push(Token::Table),
            "true" => self.tokens.push(Token::Bool(true)),
            "false" => self.tokens.push(Token::Bool(false)),
            ";" => self.tokens.push(Token::End),
            _ => self.tokens.push(Token::Text(self.current.clone())),
        }
    }

    fn process_char(&mut self, c: char) {
        match c {
            '(' => self.tokens.push(Token::OpenParen),
            ')' => self.tokens.push(Token::CloseParen),
            ',' => self.tokens.push(Token::Comma),
            _ => (),
        }
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn scanner_scans_basic_tokens() {
        let source = "
            CREATE TABLE test (id INT, name VARCHAR(255), active BOOL);
        "
        .to_string();
        let result = vec![
            Token::Create,
            Token::Table,
            Token::Text("test".to_string()),
            Token::OpenParen,
            Token::Text("id".to_string()),
            Token::Type_Int,
            Token::Comma,
            Token::Text("name".to_string()),
            Token::Type_Varchar,
            Token::OpenParen,
            Token::Text("255".to_string()),
            Token::CloseParen,
            Token::Comma,
            Token::Text("active".to_string()),
            Token::Type_Bool,
            Token::CloseParen,
            Token::End,
        ];

        let mut scanner = Lexer::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens, result);
    }
}
