use crate::lang::token::Token;

use super::{interface::Parsable, table_def::TableDef};

#[derive(Clone, Debug, PartialEq)]
pub enum Create {
    Create(TableDef),
}

impl Parsable for Create {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::Table, ..] => {
                let (n, table) = TableDef::parse(&tokens[1..]);
                return (n + 1, Create::Create(table));
            }
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}
