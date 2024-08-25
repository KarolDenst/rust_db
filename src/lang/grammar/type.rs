use crate::lang::token::Token;

use super::interface::Parsable;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Varchar(usize),
    Int,
    Bool,
}

impl Parsable for Type {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::TypeVarchar, Token::OpenParen, Token::Int(255), Token::CloseParen, ..] => {
                if let Token::Int(size) = tokens[2] {
                    return (4, Type::Varchar(size as usize));
                } else {
                    panic!("Unexpected token: {:?}", tokens[2]);
                }
            }
            [Token::TypeInt, ..] => {
                return (1, Type::Int);
            }
            [Token::TypeBool, ..] => {
                return (1, Type::Bool);
            }
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}
