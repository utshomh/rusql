use std::io::{Write, stdin, stdout};

use crate::{backend::Database, error::Error, lexer::Lexer, parser::Parser};

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

        if let Err(err) = run(&input, &mut database) {
            eprintln!("{:#?}", err);
        }
    }
}

fn run(src: &str, database: &mut Database) -> Result<(), Error> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex().map_err(|err| Error::Lex(err))?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|err| Error::Parse(err))?;

    for stmt in ast {
        let execution_result = database
            .execute(stmt)
            .map_err(|err| Error::Execution(err))?;
        println!("{:#?}", execution_result);
    }

    Ok(())
}
