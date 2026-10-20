#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),
    Eof,
    Ident(String),
    Int(i64),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    Equal,
}
