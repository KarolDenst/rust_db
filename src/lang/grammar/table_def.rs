use crate::lang::token::Token;

use super::{column_def::ColumnDef, interface::Parsable};
use derive_more::Constructor;

#[derive(Clone, Debug, PartialEq, Constructor)]
pub struct TableDef {
    pub name: String,
    pub columns: Vec<ColumnDef>,
}

impl Parsable for TableDef {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        let mut count: usize = 0;
        let mut columns: Vec<ColumnDef> = Vec::new();
        let table_name: String;
        if let Token::Name(name) = tokens[0].clone() {
            count += 1;
            table_name = name.clone();
        } else {
            panic!("Unexpected token: {:?}", tokens[0]);
        }
        if let Token::OpenParen = tokens[1] {
            count += 1;
        } else {
            panic!("Unexpected token: {:?}", tokens[1]);
        }
        loop {
            let toks = &tokens[count..];
            match toks {
                [Token::Name(_), ..] => {
                    let (n, col_) = ColumnDef::parse(toks);
                    count += n;
                    columns.push(col_);
                }
                [Token::CloseParen, ..] => {
                    count += 1;
                    break;
                }
                [Token::Comma, ..] => {
                    count += 1;
                }
                _ => {
                    panic!("Unexpected token: {:?}", toks);
                }
            }
        }
        return (
            count,
            TableDef {
                name: table_name,
                columns,
            },
        );
    }
}
