use std::fmt::{Debug, Formatter, Result};

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single char tokens
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,  // UNUSED
    RightBrace, // UNUSED
    LeftBracket,
    RightBracket,
    Comma,
    Dot, // UNUSED
    Semicolon,
    Minus,
    Plus,
    Slash,
    Star,
    Equal,

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
    // If I understood correctly, this should be static because
    // It will live till the end of the program or be coerced to end before
    // Otherwise I should use Box<str>
    // I'm also changing from String to str because
    // 1. They don't get changed
    // 2. I don't need to clone as much since it has copy
    // ...
    // Changing it failed (maybe i'm stoopid) but since I don't own
    // the String i'm using to generate the tokens during Scanning then
    // I can't make sure that it will live till the end of the program
    Identifier(String),
    ExposedFunction(String),
    String(String),
    Number(f64),

    // TODO: Add type keywords: string, number and bool
    // Keywords
    Not,
    And,
    Or,

    Fun,
    Return,

    For,
    In,
    While,
    Do,
    If,
    Then,
    Else,
    Begin, // For scoped blocks
    End,
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

    pub fn as_token_type(self) -> TokenType {
        self.token_type
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{:?} : Token::{:?}]", self.line, self.token_type,)
    }
}
