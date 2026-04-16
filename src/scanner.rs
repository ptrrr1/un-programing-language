use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::{Enumerate, Peekable},
    str,
};

use crate::{
    errors::{Error, Pos, scanner_errors::ScannerError},
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
pub struct ScannerResult {
    errors: Vec<Error<ScannerError>>,
    tokens: Vec<Token>,
}

impl ScannerResult {
    fn append(&mut self, other: &mut Self) {
        self.tokens.append(&mut other.tokens);
        self.errors.append(&mut other.errors);
    }

    fn add_token(&mut self, pos: (usize, usize), literal: &str) {
        if let Some(token) = Scanner::scan_token(literal) {
            self.tokens.push(Token::new(token, pos));
        } else {
            self.errors.push(Error::new(
                Pos::from(pos),
                ScannerError::InvalidToken(literal.to_string()),
            ));
        }
    }

    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub fn into_err(self) -> Vec<Error<ScannerError>> {
        self.errors
    }

    pub fn has_err(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[derive(Debug)]
pub struct Scanner;

impl Scanner {
    pub fn scan_file(buffer: &mut BufReader<File>) -> ScannerResult {
        let mut scanner_result = ScannerResult::default();

        for (pos_v, line_result) in buffer.lines().enumerate() {
            let line: String = line_result.expect("Failed to read line");
            scanner_result.append(&mut Self::scan_line(line, pos_v));
        }

        scanner_result
    }

    pub fn scan_line(line: String, pos_v: usize) -> ScannerResult {
        let mut scanner_result = ScannerResult::default();

        let mut chars = line.chars().enumerate().peekable();
        let mut literal = String::new();

        let mut state = States::Start;

        let mut seen_dot = false;

        while let Some((pos_h, char)) = chars.next() {
            match state {
                States::Start => match char {
                    _ if char.is_ascii_digit() => {
                        seen_dot = false;
                        literal.push(char);

                        match chars.peek() {
                            Some((_, c)) if c.is_ascii_digit() || *c == '.' || *c == '_' => {
                                state = States::InNumber;
                                continue;
                            }
                            Some((_, c)) if c.is_ascii_alphabetic() => {
                                scanner_result.add_token((pos_v, pos_h), &literal);
                                literal.clear();
                                state = States::Start;

                                scanner_result.errors.push(Error::new(
                                    Pos::Known(pos_v, pos_h),
                                    ScannerError::MissingSeparation,
                                ));
                            }
                            _ => {
                                scanner_result.add_token((pos_v, pos_h), &literal);
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

                        scanner_result.add_token((pos_v, pos_h), &literal);
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

                        scanner_result.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                    }
                },

                States::InNumber => match char {
                    // 12.34.54 => '12.3456' + error
                    '.' => {
                        match chars.peek() {
                            Some((_, c)) if *c == '_' => {
                                scanner_result.errors.push(Error::new(
                                    Pos::Known(pos_v, pos_h),
                                    ScannerError::InvalidToken(c.to_string()),
                                ));

                                continue;
                            }
                            Some((_, c)) if c.is_ascii_digit() && !seen_dot => literal.push(char),
                            Some((_, c)) if c.is_ascii_digit() && seen_dot => continue,
                            Some((_, c)) if *c == '.' => {
                                scanner_result.add_token((pos_v, pos_h - 1), &literal);
                                literal.clear();
                                literal.push(char); // Reprocess it, expecting TokenType::DotDot
                                state = States::Start;
                            }
                            _ => {
                                scanner_result.add_token((pos_v, pos_h - 1), &literal);
                                scanner_result.add_token((pos_v, pos_h), &char.to_string());
                                literal.clear();
                                state = States::Start;
                            }
                        }

                        seen_dot = true;
                    }
                    // NOTE: Allow '_' to help write numbers
                    '_' => {
                        match chars.peek() {
                            Some((_, c)) if c.is_ascii_digit() => {}
                            Some((_, c)) if *c == '.' || *c == '_' => {
                                scanner_result.errors.push(Error::new(
                                    Pos::Known(pos_v, pos_h),
                                    ScannerError::UnexpectedNumberSeparator,
                                ));
                            }
                            _ => {
                                scanner_result.errors.push(Error::new(
                                    Pos::Known(pos_v, pos_h),
                                    ScannerError::UnexpectedNumberSeparator,
                                ));

                                scanner_result.add_token((pos_v, pos_h), &literal);
                                literal.clear();
                                state = States::Start;
                            }
                        }

                        continue;
                    }
                    _ if char.is_ascii_digit() => {
                        literal.push(char);
                        match chars.peek() {
                            Some((_, c)) if c.is_ascii_digit() || *c == '_' => continue,
                            Some((_, c)) if *c == '.' && !seen_dot => continue,
                            Some((_, c)) if *c == '.' && seen_dot => {
                                scanner_result.errors.push(Error::new(
                                    Pos::Known(pos_v, pos_h),
                                    ScannerError::MultipleDecimalDivider,
                                ));

                                continue;
                            }
                            Some((_, c)) if c.is_ascii_alphabetic() => {
                                // Consume token
                                scanner_result.errors.push(Error::new(
                                    Pos::Known(pos_v, pos_h),
                                    ScannerError::MissingSeparation,
                                ));
                            }
                            _ => {} // Consume token
                        }

                        scanner_result.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                        state = States::Start;
                    }
                    _ => unreachable!(),
                },

                States::InString => match char {
                    '"' => {
                        literal.push(char);

                        scanner_result.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                        state = States::Start;
                    }
                    _ if chars.peek().is_none() => {
                        scanner_result.errors.push(Error::new(
                            Pos::Known(pos_v, pos_h),
                            ScannerError::UnclosedString,
                        ));
                    } // error
                    _ => literal.push(char),
                },

                States::InIdentifier | States::InExposedFunction => match char {
                    _ if char.is_ascii_alphanumeric() || char == '_' => {
                        literal.push(char);

                        if Scanner::makes_token_with_next(&mut chars, &literal).is_some() {
                            continue;
                        }

                        scanner_result.add_token((pos_v, pos_h), &literal);
                        literal.clear();
                        state = States::Start;
                    }
                    _ => unreachable!(),
                },

                States::InComment => match char {
                    '/' if literal == "/" => {
                        // If it's the second '/' for starting the comment
                        literal.push(char);

                        if let Some(token) = Scanner::scan_token(&literal) {
                            scanner_result
                                .tokens
                                .push(Token::new(token, (pos_v, pos_h)));
                            literal.clear();
                        }
                    }

                    _ if chars.peek().is_none() => {
                        literal.push(char);

                        scanner_result.tokens.push(Token::new(
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

        scanner_result
    }

    fn scan_token(literal: &str) -> Option<TokenType> {
        match literal {
            "(" => Some(TokenType::LeftParenthesis),
            ")" => Some(TokenType::RightParenthesis),
            "[" => Some(TokenType::LeftBracket),
            "]" => Some(TokenType::RightBracket),
            "," => Some(TokenType::Comma),
            "." => Some(TokenType::Dot),
            ";" => Some(TokenType::Semicolon),
            "-" => Some(TokenType::Minus),
            "+" => Some(TokenType::Plus),
            "/" => Some(TokenType::Slash),
            "*" => Some(TokenType::Star),
            "=" => Some(TokenType::Equal),
            ":=" => Some(TokenType::ColonEqual),
            "!=" => Some(TokenType::BangEqual),
            "==" => Some(TokenType::EqualEqual),
            ">" => Some(TokenType::Greater),
            ">=" => Some(TokenType::GreaterEqual),
            "<" => Some(TokenType::Lesser),
            "<=" => Some(TokenType::LesserEqual),
            "//" => Some(TokenType::CommentStarter),
            ".." => Some(TokenType::DotDot),
            "not" => Some(TokenType::Not),
            "and" => Some(TokenType::And),
            "or" => Some(TokenType::Or),
            "fun" => Some(TokenType::Fun),
            "lambda" => Some(TokenType::Lambda),
            "return" => Some(TokenType::Return),
            "for" => Some(TokenType::For),
            "in" => Some(TokenType::In),
            "while" => Some(TokenType::While),
            "do" => Some(TokenType::Do),
            "if" => Some(TokenType::If),
            "then" => Some(TokenType::Then),
            "else" => Some(TokenType::Else),
            "begin" => Some(TokenType::Begin),
            "end" => Some(TokenType::End),
            "nil" => Some(TokenType::Nil),
            "print" => Some(TokenType::Print),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "let" => Some(TokenType::Let),
            "break" => Some(TokenType::Break),
            "continue" => Some(TokenType::Continue),
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
                Some(TokenType::Number(literal.parse::<f64>().unwrap()))
            }
            _ if !literal.starts_with('-')
                && literal.ends_with(|c: char| c.is_ascii_digit())
                && literal.starts_with(|c: char| c.is_ascii_digit())
                && literal.contains('.') =>
            {
                Some(TokenType::Number(literal.parse::<f64>().unwrap()))
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
}
