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
    Assign,      // =
    Equals,      // ==
    GreaterThan, // >
    LessThan,    // <

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
