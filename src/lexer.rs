#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Select,
    From,
    As,
    Table,
    Create,
    Insert,
    Into,
    Value,
    Int,
    Text,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Keyword,
    Symbol,
    Identifier,
    String,
    Numberic,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
    pub loc: Location,
}

impl Token {
    pub fn new(value: String, kind: TokenKind, loc: Location) -> Self {
        Self { value, kind, loc }
    }
}

#[derive(Debug)]
pub struct LexError {
    pub value: String,
    pub message: String,
    pub loc: Location,
}

pub struct Lexer {
    source: Vec<char>,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
            col: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        Ok(tokens)
    }
}
