use std::fmt;

use crate::lang::token::Token;

use super::interface::Parsable;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Text(String),
    Bool(bool),
}

impl Parsable for Literal {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::Int(i), ..] => (1, Literal::Int(*i)),
            [Token::Text(s), ..] => (1, Literal::Text(s.clone())),
            [Token::Bool(b), ..] => (1, Literal::Bool(*b)),
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Text(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}
