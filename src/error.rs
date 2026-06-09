use crate::{backend::ExecutionError, lexer::LexError, parser::ParseError};

#[derive(Debug)]
pub enum Error {
    Lex(LexError),
    Parse(ParseError),
    Execution(ExecutionError),
}
