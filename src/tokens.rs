#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Let,
    Fun,
    If,
    Else,
    While,
    For,
    In,
    To,
    Return,
    Const,
    Break,
    Continue,
    As,

    // Types
    IntType,
    FloatType,
    BoolType,
    StringType,

    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    None,

    // Operators
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Percent,     // %
    Caret,       // ^
    Assign,      // =
    GreaterThan, // >
    LessThan,    // <
    Equals,      // ==

    PlusEqual,     // +=
    MinusEqual,    // -=
    AsteriskEqual, // *=
    SlashEqual,    // /=
    PercentEqual,  // %=

    // Logical Operators
    And,          // &&
    Or,           // ||
    Not,          // !
    BangEqual,    // !=
    GreaterEqual, // >=
    LessEqual,    // <=

    // Punctuation
    Colon,
    Semicolon,
    Comma,
    Question,

    // Brackets,
    Arrow,  // ->
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Eof,
}
