mod ast;
mod backend;
mod error;
mod lexer;
mod parser;
mod repl;

use crate::repl::repl;

fn main() {
    repl();
}
