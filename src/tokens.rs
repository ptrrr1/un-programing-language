use std::fmt::{Debug, Formatter, Result};

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single char tokens
    LeftParentesis,
    RightParentesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two char tokens
    ColonEqual, // :=
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    CommentStarter, // '//'
    Arrow,          // ->
    DotDot,         // .. for [n..m;s]

    // Literals
    Identifier(String),
    ExposedFunction(String), // '@func'
    String(String),
    NumberInt(i32),
    NumberFloat(f32),

    // Keywords
    Not,
    And,
    Or,

    Fun,
    Return,

    For,
    In,
    While,
    If,
    Else,
    Nil,
    Print,
    True,
    False,
    Let,

    Comment(String),
    Space,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: (usize, usize),
}

impl Token {
    pub fn new(token_type: TokenType, line: (usize, usize)) -> Self {
        Token { token_type, line }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{:?} : Token::{:?}]", self.line, self.token_type,)
    }
}
