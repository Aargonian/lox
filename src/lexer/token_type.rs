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
    #[strum(to_string = "and")]
    And,

    #[strum(to_string = "class")]
    Class,

    #[strum(to_string = "else")]
    Else,

    #[strum(to_string = "false")]
    False,

    #[strum(to_string = "fun")]
    Fun,

    #[strum(to_string = "for")]
    For,

    #[strum(to_string = "if")]
    If,

    #[strum(to_string = "nil")]
    Nil,

    #[strum(to_string = "or")]
    Or,

    #[strum(to_string = "print")]
    Print,

    #[strum(to_string = "return")]
    Return,

    #[strum(to_string = "super")]
    Super,

    #[strum(to_string = "this")]
    This,

    #[strum(to_string = "true")]
    True,

    #[strum(to_string = "var")]
    Var,

    #[strum(to_string = "while")]
    While,
}
