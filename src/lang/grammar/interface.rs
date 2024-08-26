use crate::lang::token::Token;

pub trait Parsable {
    fn parse(tokens: &[Token]) -> (usize, Self);
}

pub trait Interpretable {}
