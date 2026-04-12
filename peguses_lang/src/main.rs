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
        
        // == NEW FEATURES ==
        
        // Strings
        let greeting = "Hello";
        let name = "World";
        let message = greeting + " " + name + "!";
        print message;
        
        // String with escapes
        let multiline = "Line 1\nLine 2\tTabbed";
        print multiline;
        
        // For loops
        print "Counting to 5:";
        for i in 0..5 {
            print i;
        }
        
        // For loop with range
        print "Numbers 10 to 15:";
        for num in 10..15 {
            print num;
        }
        
        // Break statement
        print "Break example:";
        for i in 0..10 {
            if i == 5 {
                break;
            }
            print i;
        }
        
        // Continue statement
        print "Continue example (skip 3):";
        for i in 0..7 {
            if i == 3 {
                continue;
            }
           print i;
        }
        
        // Nested loops with break
        print "Nested loop:";
        for i in 0..3 {
            for j in 0..3 {
                if j == 2 {
                    break;
                }
                print i * 10 + j;
            }
        }
        
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