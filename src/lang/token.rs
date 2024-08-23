use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Keywords
    Create,
    Table,
    OpenParen,
    CloseParen,
    Comma,
    End,

    // Types
    TypeVarchar,
    TypeInt,
    TypeFloat,
    TypeBool,

    // Literals
    Int(i32),
    Float(f32),
    Text(String),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct RegexToken {
    pub regex: Regex,
    pub token: Option<Token>,
}

impl RegexToken {
    pub fn new(regex: &str, token: Token) -> RegexToken {
        RegexToken {
            regex: Regex::new(regex).unwrap(),
            token: Some(token),
        }
    }

    pub fn get_dict() -> Vec<RegexToken> {
        vec![
            // Whitespace
            RegexToken {
                regex: Regex::new(r"^\s+").unwrap(),
                token: None,
            },
            // Keywords
            RegexToken::new(r"^CREATE", Token::Create),
            RegexToken::new(r"^TABLE", Token::Table),
            RegexToken::new(r"^\(", Token::OpenParen),
            RegexToken::new(r"^\)", Token::CloseParen),
            RegexToken::new(r"^,", Token::Comma),
            RegexToken::new(r"^;", Token::End),
            RegexToken::new(r"^VARCHAR", Token::TypeVarchar),
            RegexToken::new(r"^INT", Token::TypeInt),
            RegexToken::new(r"^FLOAT", Token::TypeFloat),
            RegexToken::new(r"^BOOL", Token::TypeBool),
        ]
    }

    pub fn get_literals() -> Vec<RegexToken> {
        vec![
            RegexToken {
                regex: Regex::new(r"^\d+").unwrap(),
                token: Some(Token::Int(0)),
            },
            RegexToken {
                regex: Regex::new(r"^\d+\.\d+").unwrap(),
                token: Some(Token::Float(0.0)),
            },
            RegexToken {
                regex: Regex::new(r"^true").unwrap(),
                token: Some(Token::Bool(true)),
            },
            RegexToken {
                regex: Regex::new(r"^false").unwrap(),
                token: Some(Token::Bool(false)),
            },
            RegexToken {
                regex: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
                token: Some(Token::Text("".to_string())),
            },
        ]
    }
}
