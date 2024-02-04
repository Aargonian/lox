use strum::{Display, EnumIter};

#[derive(Debug, Display)]
pub enum TokenType {
    // Single Character Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or Two char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Identifiers
    Identifier(String),
    String(String),
    Number(f64),
    Keyword(Keyword),
}

#[derive(Copy, Clone, Debug, Display, EnumIter)]
pub enum Keyword {
    #[strum(message = "and")]
    And,

    #[strum(message = "class")]
    Class,

    #[strum(message = "else")]
    Else,

    #[strum(message = "false")]
    False,

    #[strum(message = "fun")]
    Fun,

    #[strum(message = "for")]
    For,

    #[strum(message = "if")]
    If,

    #[strum(message = "nil")]
    Nil,

    #[strum(message = "or")]
    Or,

    #[strum(message = "print")]
    Print,

    #[strum(message = "return")]
    Return,

    #[strum(message = "super")]
    Super,

    #[strum(message = "this")]
    This,

    #[strum(message = "true")]
    True,

    #[strum(message = "var")]
    Var,

    #[strum(message = "while")]
    While,
}
