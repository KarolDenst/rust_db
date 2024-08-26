use crate::lang::token::Token;

use super::token::RegexToken;
use regex::Regex;

pub fn get_tokens(source: String) -> Vec<Token> {
    let mut scanner = Lexer::new(source);
    return scanner.get_tokens();
}

struct Lexer {
    source: String,
    pos: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            pos: 0,
            tokens: Vec::new(),
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        while self.pos < self.source.len() {
            if !self.get_token() {
                println!("{:?}", &self.tokens);
                println!("{}", &self.source[self.pos..]);
                panic!("Unexpected character at: {}", self.pos);
            }
        }

        return self.tokens.clone();
    }

    fn get_token(&mut self) -> bool {
        for regex_token in RegexToken::get_dict() {
            if self.match_token(&regex_token) {
                return true;
            }
        }
        for regex_token in RegexToken::get_literals() {
            if self.match_token(&regex_token) {
                return true;
            }
        }

        return false;
    }

    fn match_token(&mut self, regex_token: &RegexToken) -> bool {
        let str = &self.source[self.pos..];
        let prefix_match = self.get_prefix_match(&str, &regex_token.regex.clone());
        if let Some(end) = prefix_match {
            if let Some(token) = &regex_token.token {
                let tok = token.clone();
                match tok {
                    Token::Name(_) => {
                        self.tokens.push(Token::Name(str[..end].to_string()));
                    }
                    Token::Text(_) => {
                        self.tokens.push(Token::Text(str[1..end - 1].to_string()));
                    }
                    Token::Int(_) => {
                        let val = str[..end].parse::<i32>().unwrap();
                        self.tokens.push(Token::Int(val));
                    }
                    // Token::Float(_) => {
                    //     let val = str[..end].parse::<f32>().unwrap();
                    //     self.tokens.push(Token::Float(val));
                    // }
                    _ => self.tokens.push(tok),
                }
            }
            self.pos += end;
            return true;
        }

        return false;
    }

    fn get_prefix_match(&self, str: &str, regex: &Regex) -> Option<usize> {
        return regex
            .captures(str)
            .and_then(|cap| cap.get(0))
            .map(|m| m.end());
    }
}

#[cfg(test)]
mod lexer_tests {
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
            Token::Name("test".to_string()),
            Token::OpenParen,
            Token::Name("id".to_string()),
            Token::TypeInt,
            Token::Comma,
            Token::Name("name".to_string()),
            Token::TypeVarchar,
            Token::OpenParen,
            Token::Int(255),
            Token::CloseParen,
            Token::Comma,
            Token::Name("active".to_string()),
            Token::TypeBool,
            Token::CloseParen,
            Token::End,
        ];

        let tokens = get_tokens(source);
        assert_eq!(tokens, result);
    }
}
