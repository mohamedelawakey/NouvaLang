mod lexer;
mod tokens;
use lexer::Lexer;
use tokens::Token;

fn main() {
    let source_code = "let x = 10 /* this comment never ends + 20";

    let mut lexer = Lexer::new(source_code);
    println!("lexer starts:");

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == Token::Eof {
            break;
        }
    }
}
