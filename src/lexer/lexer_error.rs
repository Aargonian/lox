use std::io;

use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum TokenIteratorError {
    #[error("I/O error occurred: {0}")]
    Io(#[from] io::Error),

    #[error("Unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedCharacter(char, usize, usize),

    #[error("Unterminated string literal at line {0}, column {1}")]
    UnterminatedString(usize, usize),

    #[error("Invalid number format at line {0}, column {1}")]
    InvalidNumber(usize, usize),

    #[error("Unknown token '{0}' at line {1}, column {2}")]
    UnknownToken(String, usize, usize),

    #[error("Nested comment not allowed at line {0}, column {1}")]
    NestedComment(usize, usize),

    #[error("Unterminated comment starting at line {0}, column {1}")]
    UnterminatedComment(usize, usize),

    #[error("Attempted to parse a token but End of File was reached.")]
    EofReached,
}
