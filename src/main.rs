mod lexer;

use crate::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("42 4.2\n4e2");
    let tokens = lexer.lex();

    println!("{tokens:#?}");
}
