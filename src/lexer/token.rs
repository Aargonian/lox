#[derive(Debug, Copy, Clone)]
enum NumberValue {
    Integer(usize),
    Real(usize, usize),
}

#[derive(Debug, Copy, Clone)]
enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minux,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number(NumberValue),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct LoxToken {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    column: usize,
}
