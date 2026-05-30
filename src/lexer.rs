use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    Create,
    Table,
    Select,
    Where,
    From,
    Insert,
    Into,
    Values,
    As,
    Int,
    Text,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Semicolon,
    Asterisk,
    Comma,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier,
    String,
    Numberic,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(keyword) => match keyword {
                Keyword::Create => write!(f, "CREATE"),
                Keyword::Table => write!(f, "TABLE"),
                Keyword::Select => write!(f, "SELECT"),
                Keyword::Where => write!(f, "WHERE"),
                Keyword::From => write!(f, "FROM"),
                Keyword::Insert => write!(f, "INSERT"),
                Keyword::Into => write!(f, "INTO"),
                Keyword::Values => write!(f, "VALUES"),
                Keyword::As => write!(f, "AS"),
                Keyword::Int => write!(f, "INT"),
                Keyword::Text => write!(f, "TEXT"),
            },
            Self::Symbol(symbol) => match symbol {
                Symbol::Semicolon => write!(f, ";"),
                Symbol::Asterisk => write!(f, "*"),
                Symbol::Comma => write!(f, ","),
                Symbol::LeftParen => write!(f, "("),
                Symbol::RightParen => write!(f, ")"),
            },
            TokenKind::Identifier => write!(f, "<IDENTIFIER>"),
            TokenKind::String => write!(f, "<STRING>"),
            TokenKind::Numberic => write!(f, "<NUMERIC>"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn loc(line: usize, col: usize, start: usize, end: usize) -> Location {
        Location::new(line, col, start, end)
    }

    fn token(
        value: &str,
        kind: TokenKind,
        line: usize,
        col: usize,
        start: usize,
        end: usize,
    ) -> Token {
        Token::new(value.to_string(), kind, loc(line, col, start, end))
    }

    fn lex(source: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(source);
        lexer.lex().expect(source)
    }

    #[test]
    fn test_lex_create_table() {
        let tokens = lex("CREATE TABLE users (id INT, name TEXT);");

        assert_eq!(
            tokens,
            vec![
                token("CREATE", TokenKind::Keyword(Keyword::Create), 1, 1, 0, 6),
                token("TABLE", TokenKind::Keyword(Keyword::Table), 1, 8, 7, 12),
                token("users", TokenKind::Identifier, 1, 14, 13, 18),
                token("(", TokenKind::Symbol(Symbol::LeftParen), 1, 20, 19, 20),
                token("id", TokenKind::Identifier, 1, 21, 20, 22),
                token("INT", TokenKind::Keyword(Keyword::Int), 1, 24, 23, 26),
                token(",", TokenKind::Symbol(Symbol::Comma), 1, 27, 26, 27),
                token("name", TokenKind::Identifier, 1, 29, 28, 32),
                token("TEXT", TokenKind::Keyword(Keyword::Text), 1, 34, 33, 37),
                token(")", TokenKind::Symbol(Symbol::RightParen), 1, 38, 37, 38),
                token(";", TokenKind::Symbol(Symbol::Semicolon), 1, 39, 38, 39),
            ]
        );
    }

    #[test]
    fn test_lex_insert_values() {
        let tokens = lex("INSERT INTO users VALUES (111, 'utsho');");

        assert_eq!(
            tokens,
            vec![
                token("INSERT", TokenKind::Keyword(Keyword::Insert), 1, 1, 0, 6),
                token("INTO", TokenKind::Keyword(Keyword::Into), 1, 8, 7, 11),
                token("users", TokenKind::Identifier, 1, 13, 12, 17),
                token("VALUES", TokenKind::Keyword(Keyword::Values), 1, 19, 18, 24),
                token("(", TokenKind::Symbol(Symbol::LeftParen), 1, 26, 25, 26),
                token("111", TokenKind::Numberic, 1, 27, 26, 29),
                token(",", TokenKind::Symbol(Symbol::Comma), 1, 30, 29, 30),
                token("utsho", TokenKind::String, 1, 34, 33, 38),
                token(")", TokenKind::Symbol(Symbol::RightParen), 1, 39, 38, 39),
                token(";", TokenKind::Symbol(Symbol::Semicolon), 1, 40, 39, 40),
            ]
        );
    }

    #[test]
    fn test_lex_select_with_alias() {
        let tokens = lex("SELECT id, name AS fullname FROM users;");

        assert_eq!(
            tokens,
            vec![
                token("SELECT", TokenKind::Keyword(Keyword::Select), 1, 1, 0, 6),
                token("id", TokenKind::Identifier, 1, 8, 7, 9),
                token(",", TokenKind::Symbol(Symbol::Comma), 1, 10, 9, 10),
                token("name", TokenKind::Identifier, 1, 12, 11, 15),
                token("AS", TokenKind::Keyword(Keyword::As), 1, 17, 16, 18),
                token("fullname", TokenKind::Identifier, 1, 20, 19, 27),
                token("FROM", TokenKind::Keyword(Keyword::From), 1, 29, 28, 32),
                token("users", TokenKind::Identifier, 1, 34, 33, 38),
                token(";", TokenKind::Symbol(Symbol::Semicolon), 1, 39, 38, 39),
            ]
        );
    }

    #[test]
    fn test_lex_select_asterisk() {
        let tokens = lex("SELECT * FROM users;");

        assert_eq!(
            tokens,
            vec![
                token("SELECT", TokenKind::Keyword(Keyword::Select), 1, 1, 0, 6),
                token("*", TokenKind::Symbol(Symbol::Asterisk), 1, 8, 7, 8),
                token("FROM", TokenKind::Keyword(Keyword::From), 1, 10, 9, 13),
                token("users", TokenKind::Identifier, 1, 15, 14, 19),
                token(";", TokenKind::Symbol(Symbol::Semicolon), 1, 20, 19, 20),
            ]
        );
    }

    #[test]
    fn test_lex_keywords_are_case_insensitive_but_preserve_original_value() {
        let tokens = lex("select Id FrOm users;");

        assert_eq!(
            tokens,
            vec![
                token("select", TokenKind::Keyword(Keyword::Select), 1, 1, 0, 6),
                token("Id", TokenKind::Identifier, 1, 8, 7, 9),
                token("FrOm", TokenKind::Keyword(Keyword::From), 1, 11, 10, 14),
                token("users", TokenKind::Identifier, 1, 16, 15, 20),
                token(";", TokenKind::Symbol(Symbol::Semicolon), 1, 21, 20, 21),
            ]
        );
    }

    #[test]
    fn test_lex_multiline_locations() {
        let tokens = lex("SELECT id\nFROM users;");

        assert_eq!(
            tokens,
            vec![
                token("SELECT", TokenKind::Keyword(Keyword::Select), 1, 1, 0, 6),
                token("id", TokenKind::Identifier, 1, 8, 7, 9),
                token("FROM", TokenKind::Keyword(Keyword::From), 2, 1, 10, 14),
                token("users", TokenKind::Identifier, 2, 6, 15, 20),
                token(";", TokenKind::Symbol(Symbol::Semicolon), 2, 11, 20, 21),
            ]
        );
    }

    #[test]
    fn test_lex_numeric_variants() {
        let tokens = lex("123 45.67 8e9");

        assert_eq!(
            tokens,
            vec![
                token("123", TokenKind::Numberic, 1, 1, 0, 3),
                token("45.67", TokenKind::Numberic, 1, 5, 4, 9),
                token("8e9", TokenKind::Numberic, 1, 11, 10, 13),
            ]
        );
    }

    #[test]
    fn test_lex_unknown_character_returns_error() {
        let mut lexer = Lexer::new("SELECT @;");

        let err = lexer.lex().expect_err("expected @ to be rejected");

        assert_eq!(err.value, "@");
        assert_eq!(err.message, "Unknown character");
        assert_eq!(err.loc, loc(1, 8, 7, 8));
    }

    #[test]
    fn test_lex_unclosed_string_returns_error() {
        let mut lexer = Lexer::new("INSERT INTO users VALUES ('alice);");

        let err = lexer
            .lex()
            .expect_err("expected unclosed string to be rejected");

        assert_eq!(err.message, "Expected closing qoute (')");
    }
}
