use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::object::{Environment, Object};
use crate::parser::Parser;

pub fn run_repl() {
    println!("NouvaLang REPL v0.1.0 (Interactive Shell)");
    println!("Type 'exit' or press Ctrl+C to quit.\n");

    let env = Rc::new(RefCell::new(Environment::new()));
    let mut evaluator = Evaluator::new();

    loop {
        print!("nouva> ");
        if let Err(_) = io::stdout().flush() {
            break;
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!();
                break;
            }
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed == "exit" || trimmed == "quit" {
                    break;
                }

                let lexer = Lexer::new(trimmed);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();

                if !parser.errors.is_empty() {
                    println!("Syntax Error:");
                    for error in parser.errors {
                        println!("  - {}", error);
                    }
                    continue;
                }

                let result = evaluator.eval_program(&program, &env);
                match result {
                    Object::Null => {}
                    Object::Error(err) => println!("Runtime Error: {}", err),
                    other => println!("{}", other.inspect()),
                }
            }
            Err(err) => {
                println!("Error reading input: {}", err);
                break;
            }
        }
    }
}
