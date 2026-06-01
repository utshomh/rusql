use crate::{
    ast::{
        Ast, ColumnDefinition, CreateTableStatement, Expression, InsertStatement,
        LiteralExpression, SelectStatement, Statement,
    },
    lexer::{Keyword, Symbol, Token, TokenKind},
};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub tokens: Vec<Token>,
}

impl ParseError {
    fn new(message: String, tokens: Vec<Token>) -> Self {
        Self { message, tokens }
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Ast, ParseError> {
        let mut ast = Vec::new();

        while let Some(token) = self.current_token() {
            match token.kind {
                TokenKind::Keyword(Keyword::Insert) => ast.push(self.parse_insert_statement()?),
                TokenKind::Keyword(Keyword::Select) => ast.push(self.parse_select_statement()?),
                TokenKind::Keyword(Keyword::Create) => ast.push(self.parse_create_statement()?),
                _ => {
                    return Err(ParseError::new(
                        format!("Unexpected token"),
                        vec![token.clone()],
                    ));
                }
            }
        }

        Ok(ast)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current_token(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }

    fn next_token(&self) -> Option<&Token> {
        if self.pos + 1 < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }

    fn expect_and_consume_token(
        &mut self,
        expected_token_kinds: &[TokenKind],
    ) -> Result<Token, ParseError> {
        if let Some(token) = self.current_token().cloned() {
            if expected_token_kinds.contains(&token.kind) {
                self.advance();
                Ok(token)
            } else {
                Err(ParseError::new(
                    format!(
                        "Expected token to be one of the following: {}",
                        expected_token_kinds
                            .iter()
                            .map(|it| it.to_string())
                            .collect::<Vec<_>>()
                            .join(" or ")
                    ),
                    vec![token.clone()],
                ))
            }
        } else {
            Err(ParseError::new(format!("Unexpected end of token"), vec![]))
        }
    }

    fn parse_insert_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_and_consume_token(&[TokenKind::Keyword(Keyword::Insert)])?;
        self.expect_and_consume_token(&[TokenKind::Keyword(Keyword::Into)])?;
        let table_name = self.expect_and_consume_token(&[TokenKind::Identifier])?;
        self.expect_and_consume_token(&[TokenKind::Keyword(Keyword::Values)])?;
        self.expect_and_consume_token(&[TokenKind::Symbol(Symbol::LeftParen)])?;

        let mut values = Vec::new();
        while let Some(token) = self.current_token()
            && token.kind != TokenKind::Symbol(Symbol::Semicolon)
        {
            values.push(self.parse_expression()?);
            if let Some(token) = self.current_token()
                && token.kind == TokenKind::Symbol(Symbol::Comma)
            {
                self.advance();
                continue;
            } else {
                break;
            }
        }

        self.expect_and_consume_token(&[TokenKind::Symbol(Symbol::RightParen)])?;
        self.expect_and_consume_token(&[TokenKind::Symbol(Symbol::Semicolon)])?;

        Ok(Statement::Insert(InsertStatement::new(table_name, values)))
    }

    fn parse_select_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_and_consume_token(&[TokenKind::Keyword(Keyword::Select)])?;

        let mut items = Vec::new();
        if let Some(token) = self.current_token()
            && token.kind == TokenKind::Symbol(Symbol::Asterisk)
        {
            self.advance();
        } else {
            while let Some(token) = self.current_token()
                && token.kind != TokenKind::Symbol(Symbol::Semicolon)
            {
                items.push(self.parse_expression()?);
                if let Some(token) = self.current_token()
                    && token.kind == TokenKind::Symbol(Symbol::Comma)
                {
                    self.advance();
                    continue;
                } else {
                    break;
                }
            }
        }

        self.expect_and_consume_token(&[TokenKind::Keyword(Keyword::From)])?;
        let from = self.expect_and_consume_token(&[TokenKind::Identifier])?;
        self.expect_and_consume_token(&[TokenKind::Symbol(Symbol::Semicolon)])?;

        Ok(Statement::Select(SelectStatement::new(items, from)))
    }

    fn parse_create_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_and_consume_token(&[TokenKind::Keyword(Keyword::Create)])?;
        self.expect_and_consume_token(&[TokenKind::Keyword(Keyword::Table)])?;
        let name = self.expect_and_consume_token(&[TokenKind::Identifier])?;
        self.expect_and_consume_token(&[TokenKind::Symbol(Symbol::LeftParen)])?;

        let mut cols = Vec::new();
        if let Some(token) = self.current_token()
            && token.kind != TokenKind::Symbol(Symbol::RightParen)
        {
            while let Some(token) = self.current_token()
                && token.kind != TokenKind::Symbol(Symbol::Semicolon)
            {
                let name = self.expect_and_consume_token(&[TokenKind::Identifier])?;
                let datatype = self.expect_and_consume_token(&[
                    TokenKind::Keyword(Keyword::Text),
                    TokenKind::Keyword(Keyword::Int),
                ])?;
                cols.push(ColumnDefinition::new(name, datatype));
                if let Some(token) = self.current_token()
                    && token.kind == TokenKind::Symbol(Symbol::Comma)
                {
                    self.advance();
                    continue;
                } else {
                    break;
                }
            }
        }

        self.expect_and_consume_token(&[TokenKind::Symbol(Symbol::RightParen)])?;
        self.expect_and_consume_token(&[TokenKind::Symbol(Symbol::Semicolon)])?;

        Ok(Statement::CreateTable(CreateTableStatement::new(
            name, cols,
        )))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        if let Some(token) = self.current_token().cloned() {
            match token.kind {
                TokenKind::String | TokenKind::Numberic | TokenKind::Identifier => {
                    self.advance();
                    Ok(Expression::Literal(LiteralExpression::new(token)))
                }
                _ => Err(ParseError::new(
                    format!("Unexpected token: {}", token.kind),
                    vec![token],
                )),
            }
        } else {
            Err(ParseError::new(format!("Unexpected end of token"), vec![]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ast::{Ast, ColumnDefinition, Expression, LiteralExpression, Statement};
    use crate::lexer::{Keyword, Lexer, Location, Token, TokenKind};

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

    fn parse_source(source: &str) -> Ast {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().expect(source);

        let mut parser = Parser::new(tokens);
        parser.parse().expect(source)
    }

    fn parse_source_err(source: &str) -> ParseError {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().expect(source);
        let mut parser = Parser::new(tokens);
        parser.parse().expect_err(source)
    }

    fn assert_literal(expr: &Expression, expected: Token) {
        match expr {
            Expression::Literal(LiteralExpression { literal }) => {
                assert_eq!(literal, &expected);
            }
        }
    }

    fn assert_col(col: &ColumnDefinition, expected_name: Token, expected_datatype: Token) {
        assert_eq!(col.name, expected_name);
        assert_eq!(col.datatype, expected_datatype);
    }

    #[test]
    fn test_parse_expression_numeric_literal() {
        let mut lexer = Lexer::new("2");
        let tokens = lexer.lex().expect("lex numeric literal");

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression().expect("parse numeric literal");

        assert_literal(&expr, token("2", TokenKind::Numberic, 1, 1, 0, 1));
        assert_eq!(parser.pos, 1);
    }

    #[test]
    fn test_parse_insert() {
        let ast = parse_source("INSERT INTO users VALUES (1, 'utsho');");

        assert_eq!(ast.len(), 1);

        match &ast[0] {
            Statement::Insert(stmt) => {
                assert_eq!(
                    stmt.table_name,
                    token("users", TokenKind::Identifier, 1, 13, 12, 17)
                );

                assert_eq!(stmt.values.len(), 2);

                assert_literal(
                    &stmt.values[0],
                    token("1", TokenKind::Numberic, 1, 27, 26, 27),
                );

                assert_literal(
                    &stmt.values[1],
                    token("utsho", TokenKind::String, 1, 32, 31, 36),
                );
            }
            other => panic!("expected INSERT statement, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_insert_mixed_literal_values() {
        let ast = parse_source("INSERT INTO users VALUES (1, 'utsho', name);");
        assert_eq!(ast.len(), 1);

        match &ast[0] {
            Statement::Insert(stmt) => {
                assert_eq!(stmt.values.len(), 3);
                assert_literal(
                    &stmt.values[0],
                    token("1", TokenKind::Numberic, 1, 27, 26, 27),
                );
                assert_literal(
                    &stmt.values[1],
                    token("utsho", TokenKind::String, 1, 32, 31, 36),
                );
                assert_literal(
                    &stmt.values[2],
                    token("name", TokenKind::Identifier, 1, 39, 38, 42),
                );
            }
            other => panic!("expected INSERT statement, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_insert_requires_closing_paren() {
        let err = parse_source_err("INSERT INTO users VALUES (1;");

        assert!(err.message.contains(")"));
        assert_eq!(
            err.tokens[0],
            token(
                ";",
                TokenKind::Symbol(crate::lexer::Symbol::Semicolon),
                1,
                28,
                27,
                28,
            )
        );
    }

    #[test]
    fn test_parse_create_table() {
        let ast = parse_source("CREATE TABLE users (id INT, name TEXT);");

        assert_eq!(ast.len(), 1);

        match &ast[0] {
            Statement::CreateTable(stmt) => {
                assert_eq!(
                    stmt.name,
                    token("users", TokenKind::Identifier, 1, 14, 13, 18)
                );

                assert_eq!(stmt.cols.len(), 2);

                assert_col(
                    &stmt.cols[0],
                    token("id", TokenKind::Identifier, 1, 21, 20, 22),
                    token("INT", TokenKind::Keyword(Keyword::Int), 1, 24, 23, 26),
                );

                assert_col(
                    &stmt.cols[1],
                    token("name", TokenKind::Identifier, 1, 29, 28, 32),
                    token("TEXT", TokenKind::Keyword(Keyword::Text), 1, 34, 33, 37),
                );
            }
            other => panic!("expected CREATE TABLE statement, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_empty_create_table() {
        let ast = parse_source("CREATE TABLE users ();");
        assert_eq!(ast.len(), 1);

        match &ast[0] {
            Statement::CreateTable(stmt) => {
                assert_eq!(
                    stmt.name,
                    token("users", TokenKind::Identifier, 1, 14, 13, 18)
                );
                assert!(stmt.cols.is_empty());
            }
            other => panic!("expected CREATE TABLE statement, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_select() {
        let ast = parse_source("SELECT id, name FROM users;");

        assert_eq!(ast.len(), 1);

        match &ast[0] {
            Statement::Select(stmt) => {
                assert_eq!(stmt.items.len(), 2);

                assert_literal(
                    &stmt.items[0],
                    token("id", TokenKind::Identifier, 1, 8, 7, 9),
                );

                assert_literal(
                    &stmt.items[1],
                    token("name", TokenKind::Identifier, 1, 12, 11, 15),
                );

                assert_eq!(
                    stmt.from,
                    token("users", TokenKind::Identifier, 1, 22, 21, 26)
                );
            }
            other => panic!("expected SELECT statement, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_select_asterisk() {
        let ast = parse_source("SELECT * FROM users;");
        assert_eq!(ast.len(), 1);

        match &ast[0] {
            Statement::Select(stmt) => {
                assert!(stmt.items.is_empty());
                assert_eq!(
                    stmt.from,
                    token("users", TokenKind::Identifier, 1, 15, 14, 19)
                );
            }
            other => panic!("expected SELECT statement, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_multiple_statements() {
        let ast = parse_source("SELECT id FROM users; INSERT INTO users VALUES (1);");
        assert_eq!(ast.len(), 2);
        assert!(matches!(&ast[0], Statement::Select(_)));
        assert!(matches!(&ast[1], Statement::Insert(_)));
    }

    #[test]
    fn test_parse_select_requires_from_clause() {
        let err = parse_source_err("SELECT id users;");

        assert!(err.message.contains("Expected token"));
        assert_eq!(
            err.tokens[0],
            token("users", TokenKind::Identifier, 1, 11, 10, 15)
        );
    }

    #[test]
    fn test_parse_expression_rejects_unexpected_token() {
        let mut lexer = Lexer::new("*");
        let tokens = lexer.lex().expect("lex asterisk");
        let mut parser = Parser::new(tokens);

        let err = parser
            .parse_expression()
            .expect_err("asterisk is not a literal expression");

        assert_eq!(err.message, "Unexpected token: *");
        assert_eq!(
            err.tokens[0],
            token(
                "*",
                TokenKind::Symbol(crate::lexer::Symbol::Asterisk),
                1,
                1,
                0,
                1,
            )
        );
    }
}
