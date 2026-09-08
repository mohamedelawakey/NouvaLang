mod lexer;
mod tokens;
mod ast;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    // let source_code = "let x: int = ((10 + -5) * 2) ^ 0.5 + (10 * 12 + (8 ^ 2) / 88.5) + 17";
    // let source_code = "let is_valid: bool = (10 + 5 > 12) && (x == 20 || !false);";
    // let source_code = "return (10 + 5 > 12) && (x == 20 || !false);";
    let source_code = "
        for i in 0 to 10 {
            let x: int = i * 2;
        }
    ";

    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        println!("Syntax Errors Found:");
        for error in parser.errors {
            println!("  - {}", error);
        }
    } else {
        println!("AST Tree:");
        println!("{:#?}", program);
    }
}
