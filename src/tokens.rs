#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Let,
    Fun,
    If,
    Else,
    While,
    Return,

    // Types
    IntType,
    FloatType,
    BoolType,
    StringType,

    // Literals
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),

    // Operators
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Percent,     // %
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
    Colon,  // :
    Comma,  // ,
    Arrow,  // ->
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    Eof,
}
