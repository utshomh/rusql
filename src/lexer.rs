#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Select,
    Where,
    From,
    As,
    Table,
    Create,
    Insert,
    Into,
    Values,
    Int,
    Text,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Semicolon,
    Asterisk,
    Comma,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier,
    String,
    Numberic,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub start: usize,
    pub end: usize,
}

impl Location {
    pub fn new(line: usize, col: usize, start: usize, end: usize) -> Self {
        Self {
            line,
            col,
            start,
            end,
        }
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

impl LexError {
    fn new(value: String, message: String, loc: Location) -> Self {
        Self {
            value,
            message,
            loc,
        }
    }
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
            pos: 0,
            col: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(current_char) = self.current_char() {
            match current_char {
                ' ' | '\t' | '\r' | '\n' => self.skip_whitespaces(),
                ';' => {
                    self.advance();
                    tokens.push(self.token(
                        current_char.to_string(),
                        TokenKind::Symbol(Symbol::Semicolon),
                    ));
                }
                '*' => {
                    self.advance();
                    tokens.push(self.token(
                        current_char.to_string(),
                        TokenKind::Symbol(Symbol::Asterisk),
                    ));
                }
                ',' => {
                    self.advance();
                    tokens.push(
                        self.token(current_char.to_string(), TokenKind::Symbol(Symbol::Comma)),
                    );
                }
                '(' => {
                    self.advance();
                    tokens.push(self.token(
                        current_char.to_string(),
                        TokenKind::Symbol(Symbol::LeftParen),
                    ));
                }
                ')' => {
                    self.advance();
                    tokens.push(self.token(
                        current_char.to_string(),
                        TokenKind::Symbol(Symbol::RightParen),
                    ));
                }
                '\'' => tokens.push(self.lex_string()?),
                _ => {
                    if current_char.is_numeric() {
                        tokens.push(self.lex_numeric()?);
                    } else if current_char.is_alphabetic() || current_char == '_' {
                        tokens.push(self.lex_keyword_or_identifier()?);
                    } else {
                        self.advance();
                        return Err(
                            self.error(current_char.to_string(), format!("Unknown character"))
                        );
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.col += 1;
    }

    fn current_char(&self) -> Option<char> {
        if self.pos < self.source.len() {
            Some(self.source[self.pos])
        } else {
            None
        }
    }

    fn next_char(&self) -> Option<char> {
        if self.pos + 1 < self.source.len() {
            Some(self.source[self.pos + 1])
        } else {
            None
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(current_char) = self.current_char() {
            match current_char {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.col = 0;
                }
                _ => return,
            }
        }
    }

    fn lex_numeric(&mut self) -> Result<Token, LexError> {
        let mut period_found = false;
        let mut exp_marker_found = false;
        let mut number_to_be = String::new();

        while let Some(current_char) = self.current_char() {
            if current_char.is_numeric() {
                number_to_be.push(current_char);
                self.advance();
            } else if current_char == '.' && !period_found && !exp_marker_found {
                if let Some(next_char) = self.next_char() {
                    if next_char.is_numeric() {
                        period_found = true;
                        number_to_be.push(current_char);
                        self.advance();
                    } else {
                        return Err(self.error(
                            next_char.to_string(),
                            format!("Expected numeric value after period (.)"),
                        ));
                    }
                }
            } else if current_char == 'e' && !exp_marker_found {
                if let Some(next_char) = self.next_char() {
                    if next_char.is_numeric() {
                        exp_marker_found = true;
                        number_to_be.push(current_char);
                        self.advance();
                    } else {
                        return Err(self.error(
                            next_char.to_string(),
                            format!("Expected numeric value after exponent marker (e)"),
                        ));
                    }
                }
            } else {
                break;
            }
        }

        Ok(self.token(number_to_be, TokenKind::Numberic))
    }

    fn lex_string(&mut self) -> Result<Token, LexError> {
        self.advance(); // Consume starting qoute
        let mut string_to_be = String::new();

        while let Some(current_char) = self.current_char() {
            if current_char == '\'' {
                break;
            } else if current_char == '\n' {
                self.advance();
                return Err(self.error(
                    current_char.to_string(),
                    format!("Expected closing qoute (')"),
                ));
            }
            string_to_be.push(current_char);
            self.advance();
        }

        if let Some(current_char) = self.current_char() {
            self.advance();
            if current_char == '\'' {
                return Ok(self.token(string_to_be, TokenKind::String));
            }
        }

        Err(self.error('\0'.to_string(), format!("Expected closing qoute (')")))
    }

    fn lex_keyword_or_identifier(&mut self) -> Result<Token, LexError> {
        let mut candidate = String::new();
        while let Some(current_char) = self.current_char() {
            if current_char.is_alphanumeric() || current_char == '_' {
                candidate.push(current_char);
                self.advance();
            } else {
                break;
            }
        }

        match candidate.to_uppercase().as_str() {
            "SELECT" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Select))),
            "WHERE" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Where))),
            "FROM" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::From))),
            "AS" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::As))),
            "TABLE" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Table))),
            "CREATE" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Create))),
            "INSERT" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Insert))),
            "INTO" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Into))),
            "VALUES" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Values))),
            "INT" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Int))),
            "TEXT" => Ok(self.token(candidate, TokenKind::Keyword(Keyword::Text))),
            _ => Ok(self.token(candidate, TokenKind::Identifier)),
        }
    }

    fn token(&self, value: String, kind: TokenKind) -> Token {
        let value_len = value.len();
        Token::new(value, kind, self.location(value_len))
    }

    fn error(&self, value: String, message: String) -> LexError {
        let value_len = value.len();
        LexError::new(value, message, self.location(value_len))
    }

    fn location(&self, value_len: usize) -> Location {
        let end = self.pos;
        let start = self.pos - value_len;
        Location::new(self.line, (self.col - value_len) + 1, start, end)
    }
}
