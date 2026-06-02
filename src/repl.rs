use std::io::{Write, stdin, stdout};

use crate::{backend::Database, lexer::Lexer, parser::Parser};

pub fn repl() {
    let mut input = String::new();
    let mut database = Database::new();

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
        let ast = parser.parse().unwrap();

        for stmt in ast {
            let execution_result = database.execute(stmt);
            println!("{:#?}", execution_result);
        }
    }
}
