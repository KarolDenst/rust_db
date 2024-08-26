use crate::lang::token::Token;

use super::{interface::Parsable, table_def::TableDef};
use crate::db::database::Database;

#[derive(Clone, Debug, PartialEq)]
pub enum Create {
    Table(TableDef),
}

impl Create {
    pub fn execute(&self, db: &mut Database) {
        match self {
            Create::Table(table_def) => {
                db.create_table(table_def.name.as_str(), table_def.columns.clone());
            }
        }
    }
}

impl Parsable for Create {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        if tokens[0] != Token::Create {
            panic!("Unexpected token: {:?}", tokens[0]);
        }
        match tokens[1..] {
            [Token::Table, ..] => {
                let (n, table) = TableDef::parse(&tokens[2..]);
                return (n + 1, Create::Table(table));
            }
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}

mod grammar_tests {
    use crate::lang::grammar::{column_def::ColumnDef, r#type::Type, table_def::TableDef};

    use super::*;

    #[test]
    fn parsing_works_for_create_table() {
        let source = vec![
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

        let expected = Create::Table(TableDef::new(
            "test".to_string(),
            vec![
                ColumnDef::new("id".to_string(), Type::Int),
                ColumnDef::new("name".to_string(), Type::Varchar(255)),
                ColumnDef::new("active".to_string(), Type::Bool),
            ],
        ));

        let (_, ast) = Create::parse(&source);
        assert_eq!(expected, ast);
    }
}
