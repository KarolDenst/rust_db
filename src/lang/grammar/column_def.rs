use std::fmt;

use crate::lang::token::Token;
use derive_more::Constructor;

use super::{interface::Parsable, r#type::Type};

#[derive(Clone, Debug, PartialEq, Constructor)]
pub struct ColumnDef {
    pub name: String,
    pub type_: Type,
}

impl Parsable for ColumnDef {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::Name(name), ..] => {
                let (n, type_) = Type::parse(&tokens[1..]);
                return (
                    n + 1,
                    ColumnDef {
                        name: name.to_string(),
                        type_,
                    },
                );
            }
            _ => {
                panic!("Unexpected token: {:?}", tokens[0]);
            }
        }
    }
}

impl fmt::Display for ColumnDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
