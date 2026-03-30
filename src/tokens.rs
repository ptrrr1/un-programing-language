use std::fmt::{Debug, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single char tokens
    LeftParentesis,
    RightParentesis,
    LeftBrace,
    RightBrace,
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
    EOF,
}

// #[derive(Debug)]
// pub enum Literal {
//     StringValue(String),
//     IntegerValue(i32),
//     FloatValue(f32),
//     BooleanValue(bool),
//     IdentifierValue(String),
//     ExposedFunction(String), // Might be removed later, Idk how to impl this aspect
//     NilValue,
//     Neither,
// }

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    //literal: Literal,
    line: (usize, usize),
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        //literal_value: String,
        line: (usize, usize),
    ) -> Self {
        //unimplemented!();

        // let literal = match token_type {
        //     TokenType::String => Literal::StringValue(literal_value.trim_matches('"').to_string()),
        //     TokenType::NumberInt => Literal::IntegerValue(literal_value.parse::<i32>().unwrap()),
        //     TokenType::NumberFloat => Literal::FloatValue(literal_value.parse::<f32>().unwrap()),
        //     TokenType::Identifier => Literal::IdentifierValue(literal_value),
        //     TokenType::ExposedFunction => Literal::ExposedFunction(literal_value),
        //     TokenType::True => Literal::BooleanValue(true),
        //     TokenType::False => Literal::BooleanValue(false),
        //     TokenType::Nil => Literal::NilValue,
        //     _ => Literal::Neither,
        // };

        Token {
            token_type,
            lexeme: lexeme.to_string(),
            //literal,
            line,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[{:?} : Token::{:?} '{}']",
            self.line,
            self.token_type,
            self.lexeme //, self.literal
        )
    }
}
