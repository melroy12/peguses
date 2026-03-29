mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let source = r#"
        let x = 1 + 2 * 3;
        print x;
    "#;

    let mut lexer = Lexer::new(source);

    match lexer.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(err) => {
            eprintln!("Lexer error: {}", err);
        }
    }
}