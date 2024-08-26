use derive_more::derive::Constructor;

use crate::{db::database::Database, lang::token::Token};

use super::{interface::Parsable, literal::Literal};

#[derive(Clone, Debug, PartialEq, Constructor)]
pub struct Row {
    pub values: Vec<Literal>,
}

impl Parsable for Row {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        let mut count: usize = 0;
        let mut values: Vec<Literal> = Vec::new();
        if let Token::OpenParen = tokens[count] {
            count += 1;
        } else {
            panic!("Unexpected token: {:?}", tokens[count]);
        }
        loop {
            let toks = &tokens[count..];
            match toks {
                [Token::Text(_), ..] | [Token::Int(_), ..] | [Token::Bool(_), ..] => {
                    let (n, val) = Literal::parse(toks);
                    count += n;
                    values.push(val);
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
        return (count, Row { values });
    }
}
