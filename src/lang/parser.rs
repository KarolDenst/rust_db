use crate::lang::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&self) {
        println!("Parsing...");
    }
}

pub enum Expr {
    Number(f32),
    Text(String),
}

pub enum Node {}
