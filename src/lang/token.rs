#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    // Keywords
    Create,
    Table,
    OpenParen,
    CloseParen,
    Comma,
    End,

    // Types
    Type_Varchar,
    Type_Int,
    Type_Float,
    Type_Bool,

    // Literals
    // Int(i32),
    // Float(f32),
    Text(String),
    Bool(bool),
}
