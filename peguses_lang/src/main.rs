mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = r#"
        let x = 1 + 2 * 3;
        print x;
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Parser error: {}", err);
            return;
        }
    };

    let mut interpreter = Interpreter::new();
    if let Err(err) = interpreter.run(&program) {
        eprintln!("Runtime error: {}", err);
    }
}