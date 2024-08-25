use crate::lang::token::Token;

use super::create::Create;
use super::interface::Parsable;

#[derive(Clone, Debug, PartialEq)]
pub enum Program {
    Create(Create),
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
                let (n, create_expr) = Create::parse(&tokens[1..]);
                return (n + 1, Program::Create(create_expr));
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
            Token::Text("test".to_string()),
            Token::OpenParen,
            Token::Text("id".to_string()),
            Token::TypeInt,
            Token::Comma,
            Token::Text("name".to_string()),
            Token::TypeVarchar,
            Token::OpenParen,
            Token::Int(255),
            Token::CloseParen,
            Token::Comma,
            Token::Text("active".to_string()),
            Token::TypeBool,
            Token::CloseParen,
            Token::End,
        ];

        let expected = Program::Create(Create::Create(TableDef::Table(
            "test".to_string(),
            vec![
                ColumnDef::Column("id".to_string(), Type::Int),
                ColumnDef::Column("name".to_string(), Type::Varchar(255)),
                ColumnDef::Column("active".to_string(), Type::Bool),
            ],
        )));

        let (_, ast) = Program::parse(&source);
        assert_eq!(expected, ast);
    }
}
