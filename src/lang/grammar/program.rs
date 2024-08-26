use crate::db::database::Database;
use crate::lang::token::Token;

use super::create::Create;
use super::insert::Insert;
use super::interface::Parsable;
use super::select::Select;

#[derive(Clone, Debug, PartialEq)]
pub enum Program {
    Create(Create),
    Insert(Insert),
    Select(Select),
}

impl Program {
    pub fn parse_tokens(tokens: &[Token]) -> Self {
        return Self::parse(tokens).1;
    }
}

impl Parsable for Program {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::Create, ..] => {
                let (n, expr) = Create::parse(&tokens);
                return (n, Program::Create(expr));
            }
            [Token::Insert, ..] => {
                let (n, expr) = Insert::parse(&tokens);
                return (n, Program::Insert(expr));
            }
            [Token::Select, ..] => {
                let (n, expr) = Select::parse(&tokens);
                return (n, Program::Select(expr));
            }
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}

impl Program {
    pub fn execute(&self, db: &mut Database) {
        match self {
            Program::Create(create) => {
                create.execute(db);
            }
            Program::Insert(insert) => {
                insert.execute(db);
            }
            Program::Select(select) => {
                select.execute(db);
            }
        }
    }
}
