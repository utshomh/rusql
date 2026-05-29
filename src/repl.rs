use std::io::{Write, stdin, stdout};

use crate::{lexer::Lexer, parser::Parser};

pub fn repl() {
    let mut input = String::new();

    loop {
        print!("rsql_> ");
        stdout().flush().unwrap();

        input.clear();

        if stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        if input.trim().is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(&input);
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens.unwrap());
        let ast = parser.parse();

        println!("{:#?}", ast);
    }
}
