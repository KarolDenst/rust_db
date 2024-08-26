use crate::{db::database::Database, lang::token::Token};

use super::{interface::Parsable, literal::Literal};

#[derive(Clone, Debug, PartialEq)]
pub struct Select {
    pub table: String,
}

impl Parsable for Select {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::Select, Token::Star, Token::From, Token::Name(name), ..] => {
                return (
                    3,
                    Select {
                        table: name.clone(),
                    },
                );
            }
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}

impl Select {
    pub fn execute(&self, db: &mut Database) {
        db.select(&self.table);
    }
}
