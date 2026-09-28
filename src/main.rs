mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod tokens;

use std::cell::RefCell;
use std::env;
use std::fs;
use std::process;
use std::rc::Rc;

use evaluator::Evaluator;
use lexer::Lexer;
use object::{Environment, Object};
use parser::Parser;

fn run_file(file_path: &str) {
    let source_code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: Could not read file '{}': {}", file_path, err);
            process::exit(1);
        }
    };

    let lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        eprintln!("Syntax Errors Found in '{}':", file_path);
        for error in parser.errors {
            eprintln!("  - {}", error);
        }
        process::exit(1);
    }

    let env = Rc::new(RefCell::new(Environment::new()));
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_program(&program, &env);

    if let Object::Error(err) = result {
        eprintln!("Runtime Error: {}", err);
        process::exit(1);
    }
}

fn print_help() {
    println!("NouvaLang CLI - v0.1.0\n");
    println!("Usage:");
    println!("  nouva                      Start the interactive REPL");
    println!("  nouva run <file.nv>        Execute a Nouva source file");
    println!("  nouva <file.nv>            Execute a Nouva source file directly");
    println!("  nouva help                 Show this help message\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No arguments -> Start REPL
            repl::run_repl();
        }
        2 => match args[1].as_str() {
            "help" | "--help" | "-h" => print_help(),
            "version" | "--version" | "-v" => println!("nouva v0.1.0"),
            file_path => run_file(file_path),
        },
        3 => {
            if args[1] == "run" {
                run_file(&args[2]);
            } else {
                eprintln!("Unknown command: {}", args[1]);
                print_help();
                process::exit(1);
            }
        }
        _ => {
            print_help();
            process::exit(1);
        }
    }
}
