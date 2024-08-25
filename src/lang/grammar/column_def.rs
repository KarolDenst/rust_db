use crate::lang::token::Token;

use super::{interface::Parsable, r#type::Type};

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnDef {
    Column(String, Type),
}

impl Parsable for ColumnDef {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::Text(name), ..] => {
                let (n, type_) = Type::parse(&tokens[1..]);
                return (n + 1, ColumnDef::Column(name.to_string(), type_));
            }
            _ => {
                panic!("Unexpected token: {:?}", tokens[0]);
            }
        }
    }
}
