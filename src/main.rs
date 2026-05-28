mod lexer;

use crate::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("CREATE TABLE gods;");
    let tokens = lexer.lex();

    println!("{tokens:?}");
}
