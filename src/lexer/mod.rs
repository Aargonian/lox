mod lexer_error;
mod token;
mod token_type;

use std::collections::HashMap;

pub use lexer_error::*;
use strum::IntoEnumIterator;
pub use token::*;
pub use token_type::*;

use crate::utils::multipeek::{MultiPeekableIterator, MultiPeekable};
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<String, Keyword> = {
        let mut map = HashMap::new();
        for keyword in Keyword::iter() {
            map.insert(keyword.to_string(), keyword);
        }
        map
    };
}

pub struct Lexer<I: Iterator<Item = char>> {
    source: MultiPeekableIterator<I>,
    current_line: usize,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(chars: I) -> Self {
        Self {
            source: chars.multi_peekable(),
            current_line: 0,
        }
    }

    fn match_next_or(
        &mut self,
        expected: char,
        match_type: TokenType,
        fallback: TokenType,
    ) -> TokenType {
        if self.source.peek() == Some(&expected) {
            self.source.next();
            match_type
        } else {
            fallback
        }
    }

    fn parse_string_literal(&mut self) -> Result<TokenType, TokenIteratorError> {
        let mut collected = String::new();
        loop {
            match self.source.next() {
                Some('"') => break,
                Some(character) => {
                    if character == '\n' {
                        self.current_line += 1;
                    }
                    collected.push(character);
                }
                None => return Err(TokenIteratorError::UnterminatedString(0, self.current_line))
            }
        }
        Ok(TokenType::String(collected))
    }

    fn parse_number_literal(&mut self, first_digit: char) -> Result<TokenType, TokenIteratorError> {
        let mut collected = String::from(first_digit);

        loop {
            match self.source.peek() {
                Some(number) if number.is_ascii_digit() => collected.push(self.source.next().expect("Expected a number")),
                Some('.') => {
                    match self.source.peek_ahead(1) {
                        Some(number) if number.is_ascii_digit() => {
                            collected.push(self.source.next().expect("Expected the dot after peeking"));
                            collected.push(self.source.next().expect("Expected a value after the dot after peeking."));
                        }
                        _ => break,
                    }
                }
                _ => break,
            }
        }

        match collected.parse::<f64>() {
            Ok(num) => Ok(TokenType::Number(num)),
            Err(_) => Err(TokenIteratorError::InvalidNumber(0, self.current_line))
        }
    }

    fn parse_identifier(&mut self, first_character: char) -> TokenType {
        let mut identifier = String::from(first_character);
        loop {
            match self.source.peek() {
                Some(alpha) if alpha.is_alphanumeric() || *alpha == '_' => identifier.push(self.source.next().expect("No character found")),
                _ => break,
            }
        }

        // Check if identifier is a keyword
        KEYWORDS.get(&identifier).map_or(TokenType::Identifier(identifier), |keyword| TokenType::Keyword(*keyword))
    }

    fn attempt_parse_token(&mut self) -> Result<Token, TokenIteratorError> {
        loop {
            let character = self.source.next().ok_or(TokenIteratorError::EofReached)?;

            let token_type = match character {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ',' => TokenType::Comma,
                '.' => TokenType::Dot,
                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                ';' => TokenType::Semicolon,
                '*' => TokenType::Star,
                '!' => self.match_next_or('=', TokenType::BangEqual, TokenType::Bang),
                '=' => self.match_next_or('=', TokenType::EqualEqual, TokenType::Equal),
                '<' => self.match_next_or('=', TokenType::LessEqual, TokenType::Less),
                '>' => self.match_next_or('=', TokenType::GreaterEqual, TokenType::Greater),
                '/' => {
                    if self.source.peek() == Some(&'/') {
                        self.source.next();
                        while let Some(&c) = self.source.peek() {
                            self.source.next();
                            if c == '\n' {
                                self.current_line += 1;
                                break;
                            }
                        }
                        continue;
                    }
                    TokenType::Slash
                }
                ' ' | '\r' | '\t' => continue,
                '\n' => {
                    self.current_line += 1;
                    continue;
                }
                '"' => {
                    let result = self.parse_string_literal();
                    match result {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    }
                }
                number if number.is_ascii_digit() => {
                    let result = self.parse_number_literal(number);
                    match result {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    }
                }
                alpha if alpha.is_alphabetic() || alpha == '_' => self.parse_identifier(alpha),
                _ => return Err(TokenIteratorError::UnexpectedCharacter(character, 0, 0)),
            };

            return Ok(Token::new(token_type, character.into(), self.current_line))
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, TokenIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.attempt_parse_token() {
            Err(TokenIteratorError::EofReached) => None,
            other => Some(other),
        }
    }
}

pub trait ToTokenIterator {
    fn tokens(self) -> Lexer<Self>
    where
        Self: Sized + Iterator<Item = char>;
}

impl<I> ToTokenIterator for I
where
    I: Iterator<Item = char>,
{
    fn tokens(self) -> Lexer<Self> {
        Lexer::new(self)
    }
}
