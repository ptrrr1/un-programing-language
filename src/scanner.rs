use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::{Enumerate, Peekable},
    str,
};

use crate::tokens::{Token, TokenType};

enum States {
    Start,
    InNumber,
    InString,
    InIdentifier,
    InExposedFunction,
    InComment,
}

#[derive(Debug)]
pub struct ScannerError {
    pos: (usize, usize),
    msg: String,
    //val: ScannerErrorType,
}

impl ScannerError {
    pub fn new(pos: (usize, usize), msg: &str) -> Self {
        Self {
            pos,
            msg: msg.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Scanner {
    errors: Vec<ScannerError>,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn scan_file(&mut self, buffer: &mut BufReader<File>) {
        for (pos_v, line_result) in buffer.lines().enumerate() {
            let line: String = line_result.expect("Failed to read line");
            self.scan_line(line, pos_v);
        }
    }

    pub fn scan_line(&mut self, line: String, pos_v: usize) {
        let mut chars = line.chars().enumerate().peekable();
        let mut literal = String::new();

        let mut state = States::Start;
        //let mut dot_count: u32 = 0;

        while let Some((pos_h, char)) = chars.next() {
            match state {
                States::Start => match char {
                    _ if char.is_ascii_digit() => {
                        literal.push(char);

                        if Scanner::makes_token_with_next(&mut chars, &literal) {
                            state = States::InNumber;
                            continue;
                        }

                        self.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                    }
                    _ if char.is_ascii_alphabetic() => {
                        literal.push(char);

                        if Scanner::makes_token_with_next(&mut chars, &literal) {
                            state = States::InIdentifier; // Or Keyword
                            continue;
                        }

                        self.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                    }
                    '"' => {
                        literal.push(char);
                        state = States::InString
                    }
                    '@' => {
                        literal.push(char);
                        state = States::InExposedFunction
                    }
                    '/' if chars.peek().is_some_and(|(_, c)| *c == '/') => {
                        literal.push(char);
                        state = States::InComment
                    }
                    _ => {
                        literal.push(char);

                        if Scanner::makes_token_with_next(&mut chars, &literal) {
                            continue;
                        }

                        self.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                    }
                },

                States::InNumber => match char {
                    _ if char.is_ascii_digit() || char == '.' => {
                        literal.push(char);

                        if Scanner::makes_token_with_next(&mut chars, &literal) {
                            continue;
                        }

                        self.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                        state = States::Start;
                    }
                    _ => unimplemented!(), // alphabetic is error, + - > etc is ok
                },

                States::InString => match char {
                    '"' => {
                        literal.push(char);

                        self.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                        state = States::Start;
                    }
                    _ if chars.peek().is_none() => {
                        self.errors
                            .push(ScannerError::new((pos_v, pos_h), "Unclosed String"));
                    } // error
                    _ => literal.push(char),
                },

                States::InIdentifier | States::InExposedFunction => match char {
                    _ if char.is_ascii_alphanumeric() => {
                        literal.push(char);

                        if Scanner::makes_token_with_next(&mut chars, &literal) {
                            continue;
                        }

                        self.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                        state = States::Start;
                    }
                    _ => unimplemented!(),
                },

                States::InComment => match char {
                    '/' if literal == "/" => {
                        // If it's the second '/' for starting the comment
                        literal.push(char);

                        if let Some(token) = Scanner::scan_token(&literal) {
                            self.tokens
                                .push(Token::new(token, &literal, (pos_v, pos_h)));
                            literal.clear();
                        }
                    }

                    _ if chars.peek().is_none() => {
                        literal.push(char);

                        self.tokens.push(Token::new(
                            TokenType::Comment(literal.to_owned()),
                            &literal,
                            (pos_v, pos_h),
                        ));

                        literal.clear();
                        state = States::Start;
                    }
                    _ => literal.push(char),
                },
            }
        }
    }

    fn scan_token(literal: &str) -> Option<TokenType> {
        match literal {
            "(" => Some(TokenType::LeftParentesis),
            ")" => Some(TokenType::RightParentesis),
            "{" => Some(TokenType::LeftBrace),
            "}" => Some(TokenType::RightBrace),
            "," => Some(TokenType::Comma),
            "." => Some(TokenType::Dot),
            ";" => Some(TokenType::Semicolon),
            "-" => Some(TokenType::Minus),
            "+" => Some(TokenType::Plus),
            "/" => Some(TokenType::Slash),
            "*" => Some(TokenType::Star),
            ":=" => Some(TokenType::ColonEqual),
            "!=" => Some(TokenType::BangEqual),
            "==" => Some(TokenType::EqualEqual),
            ">" => Some(TokenType::Greater),
            ">=" => Some(TokenType::GreaterEqual),
            "<" => Some(TokenType::Lesser),
            "<=" => Some(TokenType::LesserEqual),
            "//" => Some(TokenType::CommentStarter),
            "->" => Some(TokenType::Arrow),
            ".." => Some(TokenType::DotDot),
            "not" => Some(TokenType::Not),
            "and" => Some(TokenType::And),
            "or" => Some(TokenType::Or),
            "fun" => Some(TokenType::Fun),
            "return" => Some(TokenType::Return),
            "for" => Some(TokenType::For),
            "in" => Some(TokenType::In),
            "while" => Some(TokenType::While),
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "nil" => Some(TokenType::Nil),
            "print" => Some(TokenType::Print),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "let" => Some(TokenType::Let),
            _ if literal.chars().all(|c| c.is_ascii_whitespace()) => Some(TokenType::Space),
            _ if literal.starts_with("@")
                && literal
                    .chars()
                    .skip(1)
                    .all(|c: char| c.is_ascii_alphanumeric()) =>
            {
                Some(TokenType::ExposedFunction(literal.to_string()))
            }
            _ if literal.starts_with("\"") && literal.ends_with("\"") => {
                Some(TokenType::String(literal.to_string()))
            }
            _ if literal.parse::<i32>().is_ok() => {
                Some(TokenType::NumberInt(literal.parse::<i32>().unwrap()))
            }
            _ if literal.parse::<f32>().is_ok() => {
                Some(TokenType::NumberFloat(literal.parse::<f32>().unwrap()))
            }
            _ if literal.starts_with(|c: char| c.is_ascii_alphabetic())
                && literal.chars().all(|c| c.is_ascii_alphanumeric()) =>
            {
                Some(TokenType::Identifier(literal.to_string()))
            }
            _ => None,
        }
    }

    fn makes_token_with_next(
        chars: &mut Peekable<Enumerate<str::Chars<'_>>>,
        literal: &str,
    ) -> bool {
        if let Some(ch) = chars.peek().map(|(_, c)| *c) {
            let mut potential_literal = literal.to_string();
            potential_literal.push(ch);

            return Scanner::scan_token(&potential_literal).is_some();
        }

        false
    }

    fn add_token(&mut self, pos: (usize, usize), literal: &str) {
        if let Some(token) = Scanner::scan_token(literal) {
            self.tokens.push(Token::new(token, literal, pos));
        } else {
            self.errors
                .push(ScannerError::new(pos, "Not a valid token"));
        }
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            errors: vec![],
            tokens: vec![],
        }
    }
}
