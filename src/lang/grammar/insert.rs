use derive_more::derive::Constructor;

use crate::{db::database::Database, lang::token::Token};

use super::{interface::Parsable, literal::Literal, row::Row};

#[derive(Clone, Debug, PartialEq, Constructor)]
pub struct Insert {
    pub table: String,
    pub values: Vec<Row>,
}

impl Insert {
    pub fn execute(&self, db: &mut Database) {
        db.insert(&self.table, self.values.clone());
    }
}

impl Parsable for Insert {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        let mut count: usize = 0;
        let mut values: Vec<Row> = Vec::new();
        let table_name: String;
        match tokens {
            [Token::Insert, Token::Into, Token::Name(name), Token::Values, ..] => {
                table_name = name.clone();
                count += 4;
            }
            _ => panic!("Unexpected token: {:?}", tokens[count]),
        }
        loop {
            let toks = &tokens[count..];
            match toks {
                [Token::OpenParen, ..] => {
                    let (n, val) = Row::parse(toks);
                    count += n;
                    values.push(val);
                }
                [Token::Comma, ..] => {
                    count += 1;
                }
                [Token::End, ..] => {
                    count += 1;
                    break;
                }
                _ => {
                    panic!("Unexpected token: {:?}", toks);
                }
            }
        }
        return (
            count,
            Insert {
                table: table_name,
                values,
            },
        );
    }
}

mod grammar_tests {
    use crate::lang::grammar::{column_def::ColumnDef, r#type::Type, table_def::TableDef};

    use super::*;

    #[test]
    fn parsing_works_for_insert() {
        let source = vec![
            Token::Insert,
            Token::Into,
            Token::Name("test".to_string()),
            Token::Values,
            Token::OpenParen,
            Token::Text("id1".to_string()),
            Token::Int(1),
            Token::Bool(true),
            Token::CloseParen,
            Token::Comma,
            Token::OpenParen,
            Token::Text("id2".to_string()),
            Token::Int(2),
            Token::Bool(false),
            Token::CloseParen,
            Token::End,
        ];

        let expected = Insert::new(
            "test".to_string(),
            vec![
                Row::new(vec![
                    Literal::Text("id1".to_string()),
                    Literal::Int(1),
                    Literal::Bool(true),
                ]),
                Row::new(vec![
                    Literal::Text("id2".to_string()),
                    Literal::Int(2),
                    Literal::Bool(false),
                ]),
            ],
        );

        let (_, ast) = Insert::parse(&source);
        assert_eq!(expected, ast);
    }
}
