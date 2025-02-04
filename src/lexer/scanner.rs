use super::LoxToken;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unexpected symbol encountered on line {0}, col {1}: {2}")]
    UnexpectedSymbol(usize, usize, String),

    #[error("Unexpected EOF reached.")]
    EarlyEOF,
}

pub struct LoxScanner<'a> {
    source: &'a str,
    current_line: usize,
    current_column: usize,
}

impl<'a> LoxScanner<'a> {
    pub fn from_str(source: &'a str) -> Self {
        Self {
            source,
            current_line: 0,
            current_column: 0,
        }
    }
}

impl Iterator for LoxScanner<'_> {
    type Item = Result<LoxToken, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Err(LexerError::EarlyEOF))
    }
}
