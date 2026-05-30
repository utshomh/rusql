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
                _ => todo!(),
            }
        } else {
            Err(ParseError::new(format!("Unexpected end of token"), vec![]))
        }
    }
}
