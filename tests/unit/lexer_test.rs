use super::*;
use crate::tokens::Token;

#[test]
fn test_variable_declaration() {
    let source = "let age: int = 25";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Identifier("age".to_string()));
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::IntType);
    assert_eq!(lexer.next_token(), Token::Assign);
    assert_eq!(lexer.next_token(), Token::IntLiteral(25));
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_string_and_boolean_variables() {
    let source = "let name: string = \"Mohamed\"\nlet is_admin: bool = true";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Identifier("name".to_string()));
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::StringType);
    assert_eq!(lexer.next_token(), Token::Assign);
    assert_eq!(
        lexer.next_token(),
        Token::StringLiteral("Mohamed".to_string())
    );

    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(
        lexer.next_token(),
        Token::Identifier("is_admin".to_string())
    );
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::BoolType);
    assert_eq!(lexer.next_token(), Token::Assign);
    assert_eq!(lexer.next_token(), Token::BooleanLiteral(true));
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_function_declaration() {
    let source = "
    fun add(a: int, b: int) -> int {
        let result: int = a + b
        return result
    }
    ";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::Fun);
    assert_eq!(lexer.next_token(), Token::Identifier("add".to_string()));
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::IntType);
    assert_eq!(lexer.next_token(), Token::Comma);
    assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::IntType);
    assert_eq!(lexer.next_token(), Token::RParen);
    assert_eq!(lexer.next_token(), Token::Arrow);
    assert_eq!(lexer.next_token(), Token::IntType);
    assert_eq!(lexer.next_token(), Token::LBrace);

    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Identifier("result".to_string()));
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::IntType);
    assert_eq!(lexer.next_token(), Token::Assign);
    assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token(), Token::Plus);
    assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));

    assert_eq!(lexer.next_token(), Token::Return);
    assert_eq!(lexer.next_token(), Token::Identifier("result".to_string()));
    assert_eq!(lexer.next_token(), Token::RBrace);
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_control_flow_and_comparisons() {
    let source = "if age > 18 { print(\"Adult\") } else { print(\"Minor\") }";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::If);
    assert_eq!(lexer.next_token(), Token::Identifier("age".to_string()));
    assert_eq!(lexer.next_token(), Token::GreaterThan);
    assert_eq!(lexer.next_token(), Token::IntLiteral(18));
    assert_eq!(lexer.next_token(), Token::LBrace);
    assert_eq!(lexer.next_token(), Token::Identifier("print".to_string()));
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(
        lexer.next_token(),
        Token::StringLiteral("Adult".to_string())
    );
    assert_eq!(lexer.next_token(), Token::RParen);
    assert_eq!(lexer.next_token(), Token::RBrace);

    assert_eq!(lexer.next_token(), Token::Else);
    assert_eq!(lexer.next_token(), Token::LBrace);
    assert_eq!(lexer.next_token(), Token::Identifier("print".to_string()));
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(
        lexer.next_token(),
        Token::StringLiteral("Minor".to_string())
    );
    assert_eq!(lexer.next_token(), Token::RParen);
    assert_eq!(lexer.next_token(), Token::RBrace);
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_comments_and_whitespaces() {
    let source = "
    // This is a single line comment
    let x = 10 
    /* 
       This is a multi-line comment
       testing lexer logic
    */
    let y = 20
    ";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token(), Token::Assign);
    assert_eq!(lexer.next_token(), Token::IntLiteral(10));

    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Identifier("y".to_string()));
    assert_eq!(lexer.next_token(), Token::Assign);
    assert_eq!(lexer.next_token(), Token::IntLiteral(20));
    assert_eq!(lexer.next_token(), Token::Eof);
}
