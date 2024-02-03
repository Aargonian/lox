mod lexer_error;
mod token;
mod token_type;

use std::iter::Peekable;

pub use lexer_error::*;
pub use token::*;
pub use token_type::*;

pub struct Lexer<I: Iterator<Item = char>> {
    source: Peekable<I>,
    current_line: usize,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(chars: I) -> Self {
        Self {
            source: chars.peekable(),
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
