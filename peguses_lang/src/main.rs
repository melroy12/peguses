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
        // Simple arithmetic
        let x = 10;
        let y = 20;
        print x + y;
        
        // Modulo operator
        let remainder = 17 % 5;
        print remainder;
        
        // Boolean values and comparison
        let is_greater = x > 5;
        print is_greater;
        
        // If-else statement
        if x < y {
            print 1;
        } else {
            print 0;
        }
        
        // While loop - countdown
        let counter = 5;
        while counter > 0 {
            print counter;
            counter = counter - 1;
        }
        
        // Logical operators
        let a = true;
        let b = false;
        print a && b;
        print a || b;
        print !b;
        
        // Complex expression
        let result = (10 + 5) * 2 - 3;
        print result;
        
        // Fibonacci-like sequence
        let n1 = 0;
        let n2 = 1;
        let count = 10;
        while count > 0 {
            print n1;
            let temp = n1 + n2;
            n1 = n2;
            n2 = temp;
            count = count - 1;
        }
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