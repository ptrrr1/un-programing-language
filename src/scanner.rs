use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::{Enumerate, Peekable},
    str,
};

use crate::{
    errors::{Error, scanner_errors::ScannerError},
    tokens::{Token, TokenType},
};

enum States {
    Start,
    InNumber,
    InString,
    InIdentifier,
    InExposedFunction,
    InComment,
}

#[derive(Debug, Default)]
pub struct Scanner {
    errors: Vec<Error<ScannerError>>,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn scan_file(&mut self, buffer: &mut BufReader<File>) {
        for (pos_v, line_result) in buffer.lines().enumerate() {
            let line: String = line_result.expect("Failed to read line");
            self.scan_line(line, pos_v);
        }
    }

    // TODO: Return -> { tokens: Vec<Token>, err: Vec<Error<ScannerError>> }
    pub fn scan_line(&mut self, line: String, pos_v: usize) {
        let mut chars = line.chars().enumerate().peekable();
        let mut literal = String::new();

        let mut state = States::Start;

        let mut seen_dot = false;

        // TODO: use chars.peek() instead for the loop, makes it easier to reprocess
        // but i'll likely lose the lookahead feature
        while let Some((pos_h, char)) = chars.next() {
            match state {
                States::Start => match char {
                    _ if char.is_ascii_digit() => {
                        seen_dot = false;
                        literal.push(char);

                        match chars.peek() {
                            Some((_, c)) if c.is_ascii_digit() || *c == '.' => {
                                state = States::InNumber;
                                continue;
                            }
                            Some((_, c)) if c.is_ascii_alphabetic() => {
                                self.add_token((pos_v, pos_h), &literal);
                                literal.clear();
                                state = States::Start;

                                self.errors.push(Error::new(
                                    (pos_v, pos_h),
                                    ScannerError::MissingWhitespace,
                                ));
                            }
                            _ => {
                                self.add_token((pos_v, pos_h), &literal);
                                literal.clear();
                            }
                        }
                    }
                    _ if char.is_ascii_alphabetic() || char == '_' => {
                        literal.push(char);

                        if Scanner::makes_token_with_next(&mut chars, &literal).is_some() {
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

                        if Scanner::makes_token_with_next(&mut chars, &literal).is_some() {
                            continue;
                        }

                        self.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                    }
                },

                States::InNumber => match char {
                    // 12.34.54 => '12.3456' + error
                    '.' => {
                        match chars.peek() {
                            Some((_, c)) if c.is_ascii_digit() && !seen_dot => literal.push(char),
                            Some((_, c)) if c.is_ascii_digit() && seen_dot => continue,
                            Some((_, c)) if *c == '.' => {
                                self.add_token((pos_v, pos_h - 1), &literal);
                                literal.clear();
                                literal.push(char); // Reprocess it, expecting TokenType::DotDot
                                state = States::Start;
                            }
                            _ => {
                                self.add_token((pos_v, pos_h - 1), &literal);
                                self.add_token((pos_v, pos_h), &char.to_string());
                                literal.clear();
                                state = States::Start;
                            }
                        }

                        seen_dot = true;
                    }
                    _ if char.is_ascii_digit() => {
                        literal.push(char);

                        match chars.peek() {
                            Some((_, c)) if c.is_ascii_digit() => continue,
                            Some((_, c)) if *c == '.' && !seen_dot => continue,
                            Some((_, c)) if *c == '.' && seen_dot => {
                                self.errors.push(Error::new(
                                    (pos_v, pos_h),
                                    ScannerError::MultipleDecimalDivider,
                                ));

                                continue;
                            }
                            Some((_, c)) if c.is_ascii_alphabetic() => {
                                // Consume token
                                self.errors.push(Error::new(
                                    (pos_v, pos_h),
                                    ScannerError::MissingWhitespace,
                                ));
                            }
                            _ => {} // Consume token
                        }

                        self.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                        state = States::Start;
                    }
                    _ => unimplemented!(),
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
                            .push(Error::new((pos_v, pos_h), ScannerError::UnclosedString));
                    } // error
                    _ => literal.push(char),
                },

                States::InIdentifier | States::InExposedFunction => match char {
                    _ if char.is_ascii_alphanumeric() || char == '_' => {
                        literal.push(char);

                        if Scanner::makes_token_with_next(&mut chars, &literal).is_some() {
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
                            self.tokens.push(Token::new(token, (pos_v, pos_h)));
                            literal.clear();
                        }
                    }

                    _ if chars.peek().is_none() => {
                        literal.push(char);

                        self.tokens.push(Token::new(
                            TokenType::Comment(literal.to_owned()),
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
            "(" => Some(TokenType::LeftParenthesis),
            ")" => Some(TokenType::RightParenthesis),
            "{" => Some(TokenType::LeftBrace),
            "}" => Some(TokenType::RightBrace),
            "[" => Some(TokenType::LeftBracket),
            "]" => Some(TokenType::RightBracket),
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
                    .all(|c: char| c.is_ascii_alphanumeric() || c == '_') =>
            {
                Some(TokenType::ExposedFunction(literal.to_string()))
            }
            _ if literal.starts_with("\"") && literal.ends_with("\"") => {
                Some(TokenType::String(literal.trim_matches('"').to_string()))
            }
            _ if literal.chars().all(|c| c.is_ascii_digit()) => {
                Some(TokenType::NumberInt(literal.parse::<i32>().unwrap()))
            }
            _ if !literal.starts_with('-')
                && literal.ends_with(|c: char| c.is_ascii_digit())
                && literal.starts_with(|c: char| c.is_ascii_digit())
                && literal.contains('.') =>
            {
                Some(TokenType::NumberFloat(literal.parse::<f32>().unwrap()))
            }
            _ if literal.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                && literal
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_') =>
            {
                Some(TokenType::Identifier(literal.to_string()))
            }
            _ => None,
        }
    }

    fn makes_token_with_next(
        chars: &mut Peekable<Enumerate<str::Chars<'_>>>,
        literal: &str,
    ) -> Option<TokenType> {
        if let Some(ch) = chars.peek().map(|(_, c)| *c) {
            let mut potential_literal = literal.to_string();
            potential_literal.push(ch);

            return Scanner::scan_token(&potential_literal);
        }

        None
    }

    fn add_token(&mut self, pos: (usize, usize), literal: &str) {
        if let Some(token) = Scanner::scan_token(literal) {
            self.tokens.push(Token::new(token, pos));
        } else {
            self.errors.push(Error::new(
                pos,
                ScannerError::InvalidToken(literal.to_string()),
            ));
        }
    }

    pub fn into_errors(self) -> Vec<Error<ScannerError>> {
        self.errors
    }

    // TODO: pub fn has_error

    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }
}
