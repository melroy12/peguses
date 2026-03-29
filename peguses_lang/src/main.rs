mod lexer;
mod token;
mod ast;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = r#"
        let x = 1 + 2 * 3;
        print x;
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    println!("{:#?}", program);
}