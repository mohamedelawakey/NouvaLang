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

#[test]
fn test_compound_assignments() {
    let source = "x += 5 y -= 2 z *= 3 w /= 4 m %= 2";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token(), Token::PlusEqual);
    assert_eq!(lexer.next_token(), Token::IntLiteral(5));

    assert_eq!(lexer.next_token(), Token::Identifier("y".to_string()));
    assert_eq!(lexer.next_token(), Token::MinusEqual);
    assert_eq!(lexer.next_token(), Token::IntLiteral(2));

    assert_eq!(lexer.next_token(), Token::Identifier("z".to_string()));
    assert_eq!(lexer.next_token(), Token::AsteriskEqual);
    assert_eq!(lexer.next_token(), Token::IntLiteral(3));

    assert_eq!(lexer.next_token(), Token::Identifier("w".to_string()));
    assert_eq!(lexer.next_token(), Token::SlashEqual);
    assert_eq!(lexer.next_token(), Token::IntLiteral(4));

    assert_eq!(lexer.next_token(), Token::Identifier("m".to_string()));
    assert_eq!(lexer.next_token(), Token::PercentEqual);
    assert_eq!(lexer.next_token(), Token::IntLiteral(2));

    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_logical_operators_and_comparisons() {
    let source = "a >= 10 && b <= 20 || !(c != d) == true";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token(), Token::GreaterEqual);
    assert_eq!(lexer.next_token(), Token::IntLiteral(10));
    assert_eq!(lexer.next_token(), Token::And);
    assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
    assert_eq!(lexer.next_token(), Token::LessEqual);
    assert_eq!(lexer.next_token(), Token::IntLiteral(20));
    assert_eq!(lexer.next_token(), Token::Or);
    assert_eq!(lexer.next_token(), Token::Not);
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(lexer.next_token(), Token::Identifier("c".to_string()));
    assert_eq!(lexer.next_token(), Token::BangEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("d".to_string()));
    assert_eq!(lexer.next_token(), Token::RParen);
    assert_eq!(lexer.next_token(), Token::Equals);
    assert_eq!(lexer.next_token(), Token::BooleanLiteral(true));
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_violent_mixed_operators() {
    // Testing operators jammed together without spaces
    let source = "a+=b c-=d x*=y z/=w m%=n p&&q r||s !t!=u v>=w x<=y";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token(), Token::PlusEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));

    assert_eq!(lexer.next_token(), Token::Identifier("c".to_string()));
    assert_eq!(lexer.next_token(), Token::MinusEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("d".to_string()));

    assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token(), Token::AsteriskEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("y".to_string()));

    assert_eq!(lexer.next_token(), Token::Identifier("z".to_string()));
    assert_eq!(lexer.next_token(), Token::SlashEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("w".to_string()));

    assert_eq!(lexer.next_token(), Token::Identifier("m".to_string()));
    assert_eq!(lexer.next_token(), Token::PercentEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("n".to_string()));

    assert_eq!(lexer.next_token(), Token::Identifier("p".to_string()));
    assert_eq!(lexer.next_token(), Token::And);
    assert_eq!(lexer.next_token(), Token::Identifier("q".to_string()));

    assert_eq!(lexer.next_token(), Token::Identifier("r".to_string()));
    assert_eq!(lexer.next_token(), Token::Or);
    assert_eq!(lexer.next_token(), Token::Identifier("s".to_string()));

    assert_eq!(lexer.next_token(), Token::Not);
    assert_eq!(lexer.next_token(), Token::Identifier("t".to_string()));
    assert_eq!(lexer.next_token(), Token::BangEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("u".to_string()));

    assert_eq!(lexer.next_token(), Token::Identifier("v".to_string()));
    assert_eq!(lexer.next_token(), Token::GreaterEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("w".to_string()));

    assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token(), Token::LessEqual);
    assert_eq!(lexer.next_token(), Token::Identifier("y".to_string()));

    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_edge_case_spaces_between_compound_operators() {
    // If user types "+ =" with a space, it should be parsed as Plus then Assign, NOT PlusEqual
    let source = "+ = - = * = / = % = ! = < = > =";
    let mut lexer = Lexer::new(source);

    assert_eq!(lexer.next_token(), Token::Plus);
    assert_eq!(lexer.next_token(), Token::Assign);

    assert_eq!(lexer.next_token(), Token::Minus);
    assert_eq!(lexer.next_token(), Token::Assign);

    assert_eq!(lexer.next_token(), Token::Asterisk);
    assert_eq!(lexer.next_token(), Token::Assign);

    assert_eq!(lexer.next_token(), Token::Slash);
    assert_eq!(lexer.next_token(), Token::Assign);

    assert_eq!(lexer.next_token(), Token::Percent);
    assert_eq!(lexer.next_token(), Token::Assign);

    assert_eq!(lexer.next_token(), Token::Not);
    assert_eq!(lexer.next_token(), Token::Assign);

    assert_eq!(lexer.next_token(), Token::LessThan);
    assert_eq!(lexer.next_token(), Token::Assign);

    assert_eq!(lexer.next_token(), Token::GreaterThan);
    assert_eq!(lexer.next_token(), Token::Assign);

    assert_eq!(lexer.next_token(), Token::Eof);
}
