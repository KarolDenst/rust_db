use super::token::Token;

trait Parsable {
    fn parse(tokens: &[Token]) -> (usize, Self);
}

#[derive(Clone, Debug, PartialEq)]
enum Program {
    CreateExpr(CreateExpr),
}

impl Parsable for Program {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::Create, ..] => {
                let (n, create_expr) = CreateExpr::parse(&tokens[1..]);
                return (n + 1, Program::CreateExpr(create_expr));
            }
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CreateExpr {
    Create(TableDef),
}

impl Parsable for CreateExpr {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::Table, ..] => {
                let (n, table) = TableDef::parse(&tokens[1..]);
                return (n + 1, CreateExpr::Create(table));
            }
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TableDef {
    Table(String, Vec<ColumnDef>),
}

impl Parsable for TableDef {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        let mut count: usize = 0;
        let mut columns: Vec<ColumnDef> = Vec::new();
        let table_name: String;
        if let Token::Text(name) = tokens[0].clone() {
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
                [Token::Text(_), ..] => {
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
        return (count, TableDef::Table(table_name, columns));
    }
}

#[derive(Clone, Debug, PartialEq)]
enum ColumnDef {
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

#[derive(Clone, Debug, PartialEq)]
enum Type {
    Varchar(usize),
    Int,
    Bool,
}

impl Parsable for Type {
    fn parse(tokens: &[Token]) -> (usize, Self) {
        match tokens {
            [Token::TypeVarchar, Token::OpenParen, Token::Int(255), Token::CloseParen, ..] => {
                if let Token::Int(size) = tokens[2] {
                    return (4, Type::Varchar(size as usize));
                } else {
                    panic!("Unexpected token: {:?}", tokens[2]);
                }
            }
            [Token::TypeInt, ..] => {
                return (1, Type::Int);
            }
            [Token::TypeBool, ..] => {
                return (1, Type::Bool);
            }
            _ => panic!("Unexpected token: {:?}", tokens[0]),
        }
    }
}

#[cfg(test)]
mod grammar_tests {
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

        let expected = Program::CreateExpr(CreateExpr::Create(TableDef::Table(
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
