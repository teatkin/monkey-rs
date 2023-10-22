#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal(String),
    Eof,

    // Identifiers & literals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        id => Token::Ident(id.into()),
    }
}
