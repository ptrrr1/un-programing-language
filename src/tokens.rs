use std::fmt::{Debug, Display, Formatter, Result};

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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single char tokens
    LeftParenthesis,
    RightParenthesis,
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

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenType::LeftParenthesis => write!(f, "("),
            TokenType::RightParenthesis => write!(f, ")"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Equal => write!(f, "="),
            TokenType::ColonEqual => write!(f, ":="),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Lesser => write!(f, "<"),
            TokenType::LesserEqual => write!(f, "<="),
            TokenType::CommentStarter => write!(f, "//"),
            TokenType::DotDot => write!(f, ".."),

            TokenType::Identifier(name) => write!(f, "identifier({})", name),
            TokenType::ExposedFunction(name) => write!(f, "exposed_fn({})", name),
            TokenType::String(s) => write!(f, "\"{}\"", s),
            TokenType::Number(n) => write!(f, "{}", n),

            TokenType::Not => write!(f, "not"),
            TokenType::And => write!(f, "and"),
            TokenType::Or => write!(f, "or"),
            TokenType::Fun => write!(f, "fun"),
            TokenType::Return => write!(f, "return"),
            TokenType::For => write!(f, "for"),
            TokenType::In => write!(f, "in"),
            TokenType::While => write!(f, "while"),
            TokenType::Do => write!(f, "do"),
            TokenType::If => write!(f, "if"),
            TokenType::Then => write!(f, "then"),
            TokenType::Else => write!(f, "else"),
            TokenType::Begin => write!(f, "begin"),
            TokenType::End => write!(f, "end"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Print => write!(f, "print"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Let => write!(f, "let"),

            TokenType::Comment(c) => write!(f, "{}", c),
            TokenType::Space => write!(f, " "),
        }
    }
}
