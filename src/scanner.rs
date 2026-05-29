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

        while let Some((_, char)) = chars.peek() {
            if char.is_whitespace() {
                chars.next();
                continue;
            }

            match char {
                'A'..='Z' | 'a'..='z' | '_' => {
                    Self::scan_identifier(&mut scanner_result, &mut chars, pos_v)
                }
                '0'..='9' => Self::scan_number(&mut scanner_result, &mut chars, pos_v),
                '"' => Self::scan_string(&mut scanner_result, &mut chars, pos_v),
                '/' => Self::scan_comment(&mut scanner_result, &mut chars, pos_v),
                _ => Self::scan_others(&mut scanner_result, &mut chars, pos_v),
            }
        }

        scanner_result
    }

    fn scan_identifier(
        scanner_result: &mut ScannerResult,
        chars: &mut Peekable<Enumerate<str::Chars<'_>>>,
        pos_v: usize,
    ) {
        let mut literal = String::new();

        let (_, ch) = chars.next().unwrap();
        literal.push(ch);

        while let Some(&(pos_h, char)) = chars.peek() {
            match char {
                '_' => {
                    let (_, ch) = chars.next().unwrap();
                    literal.push(ch);

                    if Scanner::makes_token_with_next(chars, &literal).is_none() {
                        scanner_result.add_token((pos_v, pos_h), &literal);
                        break;
                    }
                }
                _ if char.is_ascii_alphabetic() => {
                    let (_, ch) = chars.next().unwrap();
                    literal.push(ch);

                    if Scanner::makes_token_with_next(chars, &literal).is_none() {
                        scanner_result.add_token((pos_v, pos_h), &literal);
                        break;
                    }
                }
                _ if char.is_ascii_digit() && literal.len() > 1 => {
                    let (_, ch) = chars.next().unwrap();
                    literal.push(ch);

                    if Scanner::makes_token_with_next(chars, &literal).is_none() {
                        scanner_result.add_token((pos_v, pos_h), &literal);
                        break;
                    }
                }
                _ => {
                    scanner_result.add_token((pos_v, pos_h), &literal);
                    break;
                }
            }
        }
    }

    fn scan_number(
        scanner_result: &mut ScannerResult,
        chars: &mut Peekable<Enumerate<str::Chars<'_>>>,
        pos_v: usize,
    ) {
        let mut seen_dot = false;
        let mut seen_underscore = false;
        let mut literal = String::new();

        while let Some(&(pos_h, char)) = chars.peek() {
            match char {
                '.' => {
                    let (pos_hi, ch) = chars.next().unwrap();

                    if seen_underscore {
                        scanner_result.errors.push(Error::new(
                            Pos::Known(pos_v, pos_hi.saturating_sub(1)),
                            ScannerError::UnexpectedNumberSeparator,
                        ));
                    }

                    if seen_dot {
                        scanner_result.errors.push(Error::new(
                            Pos::Known(pos_v, pos_hi),
                            ScannerError::InvalidToken(ch.to_string()),
                        ));
                    } else {
                        literal.push(ch);
                        seen_dot = true;
                    }
                }
                '_' => {
                    let (pos_hi, ch) = chars.next().unwrap();

                    if literal.ends_with(".") {
                        scanner_result.errors.push(Error::new(
                            Pos::Known(pos_v, pos_hi),
                            ScannerError::InvalidToken(ch.to_string()),
                        ));
                    }

                    seen_underscore = true;
                }
                _ if char.is_ascii_digit() => {
                    let (_, ch) = chars.next().unwrap();
                    literal.push(ch);
                    seen_underscore = false;
                }

                _ => {
                    if !char.is_whitespace() && char != ';' {
                        scanner_result.errors.push(Error::new(
                            Pos::Known(pos_v, pos_h),
                            ScannerError::MissingSeparation,
                        ));
                    }

                    scanner_result.add_token((pos_v, pos_h), &literal);
                    break;
                }
            }
        }
    }

    fn scan_string(
        scanner_result: &mut ScannerResult,
        chars: &mut Peekable<Enumerate<str::Chars<'_>>>,
        pos_v: usize,
    ) {
        let mut literal = String::new();

        let (_, ch) = chars.next().unwrap();
        literal.push(ch);

        while let Some(&(pos_h, char)) = chars.peek() {
            match char {
                '"' => {
                    let (_, ch) = chars.next().unwrap();
                    literal.push(ch);

                    scanner_result.add_token((pos_v, pos_h), &literal);
                    break;
                }
                _ => {
                    let (pos_hi, ch) = chars.next().unwrap();
                    literal.push(ch);

                    if chars.peek().is_none() {
                        scanner_result.errors.push(Error::new(
                            Pos::Known(pos_v, pos_hi),
                            ScannerError::UnclosedString,
                        ));
                    }
                }
            }
        }
    }

    fn scan_comment(
        scanner_result: &mut ScannerResult,
        chars: &mut Peekable<Enumerate<str::Chars<'_>>>,
        pos_v: usize,
    ) {
        let mut literal = String::new();

        let (pos_h, ch) = chars.next().unwrap();
        literal.push(ch);

        if chars.peek().is_some_and(|(_, c)| *c == '/') {
            let _ = chars.next();

            scanner_result
                .tokens
                .push(Token::new(TokenType::CommentStarter, (pos_v, pos_h)));
            literal.clear();

            let mut end_h = pos_h;
            for (pos_hi, char) in chars.by_ref() {
                literal.push(char);
                end_h = pos_hi;
            }

            if chars.peek().is_none() {
                scanner_result.tokens.push(Token::new(
                    TokenType::Comment(literal.to_owned()),
                    (pos_v, end_h),
                ));
            }
        } else {
            scanner_result
                .tokens
                .push(Token::new(TokenType::Slash, (pos_v, pos_h)));
        }
    }

    fn scan_others(
        scanner_result: &mut ScannerResult,
        chars: &mut Peekable<Enumerate<str::Chars<'_>>>,
        pos_v: usize,
    ) {
        let mut literal = String::new();

        while let Some((pos_h, char)) = chars.next() {
            literal.push(char);

            if Scanner::makes_token_with_next(chars, &literal).is_some() {
                continue;
            }

            scanner_result.add_token((pos_v, pos_h), &literal);
            break;
        }
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
            "fn" => Some(TokenType::Fn),
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
            // "continue" => Some(TokenType::Continue),
            _ if literal.chars().all(|c| c.is_ascii_whitespace()) => Some(TokenType::Space),
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
